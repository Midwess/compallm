use crate::{
    MailingError, SmtpSettings, SmtpTls,
    recipient::{Recipients, Reservation},
};
use async_channel::{Receiver, Sender};
use compallm_core::mail::{EmailError, EmailRequest, EmailService};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, MultiPart},
    transport::smtp::{PoolConfig, authentication::Credentials},
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::time::Duration;
use tokio::{
    sync::watch,
    time::{Instant, MissedTickBehavior, interval_at, timeout},
};

struct Job {
    message: Message,
    reservation: Reservation,
}

#[derive(Clone, Copy, Debug)]
pub struct MailingStatus {
    pub queued: usize,
    pub active: usize,
    pub sent: usize,
    pub failed: usize,
}

pub struct MailingService {
    tx: Sender<Job>,
    rx: Receiver<Job>,
    recipients: Arc<Recipients>,
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    started: AtomicBool,
    settings: SmtpSettings,
    stopped: watch::Sender<bool>,
    active: AtomicUsize,
    sent: AtomicUsize,
    failed: AtomicUsize,
}

impl MailingService {
    pub fn new(settings: SmtpSettings) -> Result<Self, MailingError> {
        if settings.host.trim().is_empty() || settings.port == 0 {
            return Err(MailingError::Configuration("host and port are required"));
        }
        if settings.username.is_empty() != settings.password.is_empty() {
            return Err(MailingError::Configuration(
                "SMTP_USERNAME and SMTP_PASSWORD must be set together",
            ));
        }
        let from = settings.from_address.parse().map_err(|_| {
            MailingError::Configuration("SMTP_FROM_ADDRESS must be a valid mailbox")
        })?;
        let mut builder = match settings.tls {
            SmtpTls::None => {
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(settings.host.clone())
            }
            SmtpTls::Starttls => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&settings.host)
                    .map_err(|_| MailingError::Configuration("invalid STARTTLS host"))?
            }
            SmtpTls::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(&settings.host)
                .map_err(|_| MailingError::Configuration("invalid TLS host"))?,
        }
        .port(settings.port)
        .timeout(Some(Duration::from_secs(settings.send_timeout_seconds)))
        .pool_config(PoolConfig::new().max_size(settings.concurrency as u32));
        if !settings.username.is_empty() {
            builder = builder.credentials(Credentials::new(
                settings.username.clone(),
                settings.password.clone(),
            ));
        }
        let (tx, rx) = async_channel::bounded(settings.queue_capacity);
        Ok(Self {
            tx,
            rx,
            transport: builder.build(),
            from,
            recipients: Arc::new(Recipients::new(Duration::from_secs(
                settings.recipient_cooldown_seconds,
            ))),
            started: AtomicBool::new(false),
            settings,
            stopped: watch::channel(false).0,
            active: AtomicUsize::new(0),
            sent: AtomicUsize::new(0),
            failed: AtomicUsize::new(0),
        })
    }

    /// Join all consumer loops within the application's single worker task.
    pub async fn run(&self) {
        if self.started.swap(true, Ordering::AcqRel) {
            return;
        }
        {
            let consumers = futures_util::future::join_all(
                (0..self.settings.concurrency).map(|_| self.consume()),
            );
            tokio::pin!(consumers);
            let minute = Duration::from_secs(60);
            let mut report = interval_at(Instant::now() + minute, minute);
            report.set_missed_tick_behavior(MissedTickBehavior::Skip);
            let mut previously_sent = 0;
            loop {
                tokio::select! {
                    _ = &mut consumers => break,
                    _ = self.tx.closed() => {
                        let _ = timeout(Duration::from_secs(self.settings.shutdown_timeout_seconds), &mut consumers).await;
                        break;
                    }
                    _ = report.tick() => {
                        let total_sent = self.sent.load(Ordering::Acquire);
                        tracing::info!(
                            sent_since_last_report = total_sent - previously_sent,
                            total_sent,
                            "Mailing delivery summary"
                        );
                        previously_sent = total_sent;
                    }
                }
            }
        }
        while self.rx.try_recv().is_ok() {}
        self.stopped.send_replace(true);
    }

    pub fn status(&self) -> MailingStatus {
        MailingStatus {
            queued: self.rx.len(),
            active: self.active.load(Ordering::Acquire),
            sent: self.sent.load(Ordering::Acquire),
            failed: self.failed.load(Ordering::Acquire),
        }
    }

    pub async fn enqueue(&self, request: EmailRequest) -> Result<(), EmailError> {
        if self.tx.is_closed() {
            return Err(EmailError::Unavailable);
        }
        let email =
            compallm_utils::email::normalize_email(&request.to).map_err(anyhow::Error::from)?;
        let reservation = self.recipients.reserve(email.clone())?;
        let to = email.parse::<Mailbox>().map_err(anyhow::Error::from)?;
        let message = Message::builder()
            .from(self.from.clone())
            .to(to)
            .subject(request.subject)
            .multipart(MultiPart::alternative_plain_html(
                request.text.plain_text(),
                request.text.to_string(),
            ))
            .map_err(anyhow::Error::from)?;
        let job = Job {
            message,
            reservation,
        };
        match timeout(
            Duration::from_secs(self.settings.admission_timeout_seconds),
            self.tx.send(job),
        )
        .await
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(_)) => Err(EmailError::Unavailable),
            Err(_) => Err(EmailError::QueueTimeout),
        }
    }

    async fn consume(&self) {
        let rx = self.rx.clone();
        while let Ok(job) = rx.recv().await {
            let active = Active::new(&self.active);
            match timeout(
                Duration::from_secs(self.settings.send_timeout_seconds),
                self.transport.send(job.message),
            )
            .await
            {
                Ok(Ok(_)) => {
                    job.reservation.sent();
                    self.sent.fetch_add(1, Ordering::AcqRel);
                }
                _ => {
                    self.failed.fetch_add(1, Ordering::AcqRel);
                    tracing::warn!("SMTP delivery failed or timed out; no automatic retry");
                }
            }
            drop(active);
        }
    }

    pub async fn shutdown(&self) {
        self.tx.close();
        if self.started.load(Ordering::Acquire) {
            let _ = self.stopped.subscribe().wait_for(|stopped| *stopped).await;
        } else {
            while self.rx.try_recv().is_ok() {}
        }
    }
}

#[async_trait::async_trait]
impl EmailService for MailingService {
    async fn send(&self, request: EmailRequest) -> Result<(), EmailError> {
        self.enqueue(request).await
    }
}
#[async_trait::async_trait]
impl EmailService for &'static MailingService {
    async fn send(&self, request: EmailRequest) -> Result<(), EmailError> {
        self.enqueue(request).await
    }
}

impl Drop for MailingService {
    fn drop(&mut self) {
        self.tx.close();
    }
}

struct Active<'a>(&'a AtomicUsize);
impl<'a> Active<'a> {
    fn new(count: &'a AtomicUsize) -> Self {
        count.fetch_add(1, Ordering::AcqRel);
        Self(count)
    }
}
impl Drop for Active<'_> {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
}
