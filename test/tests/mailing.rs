use compallm_core::mail::{EmailError, EmailRequest, EmailService, EmailTemplate, OtpCode};
use compallm_mailing::{MailingService, SmtpSettings, SmtpTls};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::Semaphore,
    task::{JoinHandle, JoinSet},
    time::{advance, pause, resume, timeout},
};

struct Smtp {
    port: u16,
    task: Option<JoinHandle<()>>,
    gate: Arc<Semaphore>,
    started: Arc<AtomicUsize>,
    peak: Arc<AtomicUsize>,
    messages: Arc<Mutex<Vec<String>>>,
}

impl Smtp {
    async fn start(hold: bool, reject: bool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let gate = Arc::new(Semaphore::new(if hold { 0 } else { 10000 }));
        let started = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let active = Arc::new(AtomicUsize::new(0));
        let messages = Arc::new(Mutex::new(Vec::new()));
        let (g, s, p, a, m) = (
            gate.clone(),
            started.clone(),
            peak.clone(),
            active.clone(),
            messages.clone(),
        );
        let task = tokio::spawn(async move {
            let mut connections = JoinSet::new();
            loop {
                tokio::select! {
                    connection=listener.accept()=>{let (stream,_)=connection.unwrap();
                        let (g,s,p,a,m)=(g.clone(),s.clone(),p.clone(),a.clone(),m.clone());
                        connections.spawn(async move {let _=Self::connection(stream,g,s,p,a,m,reject).await;});
                    }
                    Some(_)=connections.join_next()=>{}
                }
            }
        });
        Self {
            port,
            task: Some(task),
            gate,
            started,
            peak,
            messages,
        }
    }

    async fn connection(
        stream: TcpStream,
        gate: Arc<Semaphore>,
        started: Arc<AtomicUsize>,
        peak: Arc<AtomicUsize>,
        active: Arc<AtomicUsize>,
        messages: Arc<Mutex<Vec<String>>>,
        reject: bool,
    ) -> std::io::Result<()> {
        let (read, mut write) = stream.into_split();
        let mut read = BufReader::new(read);
        write.write_all(b"220 local SMTP test server\r\n").await?;
        loop {
            let mut line = String::new();
            if read.read_line(&mut line).await? == 0 {
                return Ok(());
            }
            if line.starts_with("EHLO") || line.starts_with("HELO") {
                write
                    .write_all(b"250-localhost\r\n250 PIPELINING\r\n")
                    .await?;
            } else if line.starts_with("DATA") {
                write.write_all(b"354 send message\r\n").await?;
                let mut body = String::new();
                loop {
                    let mut line = String::new();
                    if read.read_line(&mut line).await? == 0 {
                        return Ok(());
                    }
                    if line == ".\r\n" {
                        break;
                    }
                    body.push_str(&line);
                }
                let count = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(count, Ordering::SeqCst);
                started.fetch_add(1, Ordering::SeqCst);
                gate.acquire().await.unwrap().forget();
                active.fetch_sub(1, Ordering::SeqCst);
                if reject {
                    write.write_all(b"550 rejected\r\n").await?;
                } else {
                    messages.lock().unwrap().push(body);
                    write.write_all(b"250 accepted\r\n").await?;
                }
            } else if line.starts_with("QUIT") {
                write.write_all(b"221 bye\r\n").await?;
                return Ok(());
            } else {
                write.write_all(b"250 ok\r\n").await?;
            }
        }
    }

    fn settings(&self) -> SmtpSettings {
        SmtpSettings {
            enabled: true,
            host: "127.0.0.1".into(),
            port: self.port,
            from_address: "noreply@example.com".into(),
            username: String::new(),
            password: String::new(),
            tls: SmtpTls::None,
            concurrency: 10,
            queue_capacity: 1024,
            admission_timeout_seconds: 30,
            send_timeout_seconds: 60,
            recipient_cooldown_seconds: 30,
            shutdown_timeout_seconds: 60,
        }
    }
    fn service(&self) -> Arc<MailingService> {
        self.service_with_settings(self.settings())
    }
    fn service_with_settings(&self, settings: SmtpSettings) -> Arc<MailingService> {
        let service = Arc::new(MailingService::new(settings).unwrap());
        let runtime = service.clone();
        tokio::spawn(async move { runtime.run().await });
        service
    }
    fn request(to: impl Into<String>) -> EmailRequest {
        EmailRequest {
            to: to.into(),
            subject: "Verification code".into(),
            text: EmailTemplate::OtpCode(OtpCode::new("123456", Duration::from_secs(600))),
        }
    }
    async fn wait(predicate: impl Fn() -> bool) {
        timeout(Duration::from_secs(10), async {
            while !predicate() {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        })
        .await
        .unwrap();
    }
    async fn stop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
            let _ = task.await;
        }
    }
}
impl Drop for Smtp {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}

#[tokio::test]
async fn ten_consumers_send_concurrently_while_remaining_jobs_wait() {
    let mut smtp = Smtp::start(true, false).await;
    let service = smtp.service();
    // A second run must return without creating additional consumers.
    tokio::task::yield_now().await;
    service.run().await;
    for index in 0..12 {
        service
            .send(Smtp::request(format!("reader{index}@example.com")))
            .await
            .unwrap();
    }
    Smtp::wait(|| smtp.started.load(Ordering::SeqCst) == 10).await;
    assert_eq!(service.status().active, 10);
    assert_eq!(service.status().queued, 2);
    assert_eq!(smtp.peak.load(Ordering::SeqCst), 10);
    assert!(matches!(
        service.send(Smtp::request(" READER0@EXAMPLE.COM ")).await,
        Err(EmailError::Throttled)
    ));
    smtp.gate.add_permits(12);
    Smtp::wait(|| service.status().sent == 12).await;
    assert!(smtp.peak.load(Ordering::SeqCst) <= 10);
    let mail = smtp.messages.lock().unwrap()[0].clone();
    assert!(mail.contains("multipart/alternative"));
    assert!(mail.contains("text/plain"));
    assert!(mail.contains("text/html"));
    assert!(mail.contains("123456"));
    service.shutdown().await;
    smtp.stop().await;
    assert!(matches!(
        service.send(Smtp::request("new@example.com")).await,
        Err(EmailError::Unavailable)
    ));
}

#[tokio::test]
async fn successful_delivery_throttles_the_normalized_recipient_for_thirty_seconds() {
    let mut smtp = Smtp::start(false, false).await;
    let service = smtp.service();
    service
        .send(Smtp::request(" Reader@Example.com "))
        .await
        .unwrap();
    Smtp::wait(|| service.status().sent == 1).await;
    pause();
    assert!(matches!(
        service.send(Smtp::request("reader@example.com")).await,
        Err(EmailError::Throttled)
    ));
    advance(Duration::from_secs(29)).await;
    assert!(matches!(
        service.send(Smtp::request("READER@EXAMPLE.COM")).await,
        Err(EmailError::Throttled)
    ));
    advance(Duration::from_secs(1)).await;
    service
        .send(Smtp::request("reader@example.com"))
        .await
        .unwrap();
    resume();
    Smtp::wait(|| service.status().sent == 2).await;
    service.shutdown().await;
    smtp.stop().await;
}

#[tokio::test]
async fn full_queue_waits_at_most_thirty_seconds_and_releases_the_failed_reservation() {
    let mut smtp = Smtp::start(true, false).await;
    let service = smtp.service();
    for index in 0..1034 {
        service
            .send(Smtp::request(format!("queue{index}@example.com")))
            .await
            .unwrap();
    }
    Smtp::wait(|| smtp.started.load(Ordering::SeqCst) == 10).await;
    assert_eq!(service.status().queued, 1024);
    let mail = service.clone();
    let waiting =
        tokio::spawn(async move { mail.send(Smtp::request("overflow@example.com")).await });
    tokio::task::yield_now().await;
    pause();
    advance(Duration::from_secs(29)).await;
    assert!(!waiting.is_finished());
    advance(Duration::from_secs(1)).await;
    assert!(matches!(
        waiting.await.unwrap(),
        Err(EmailError::QueueTimeout)
    ));
    assert_eq!(service.status().failed, 0);
    resume();
    smtp.stop().await;
    Smtp::wait(|| service.status().queued < 1024).await;
    service
        .send(Smtp::request("overflow@example.com"))
        .await
        .unwrap();
    service.shutdown().await;
}

#[tokio::test]
async fn cancelled_admission_does_not_leave_the_recipient_reserved() {
    let mut smtp = Smtp::start(true, false).await;
    let service = smtp.service();
    for index in 0..1034 {
        service
            .send(Smtp::request(format!("cancel{index}@example.com")))
            .await
            .unwrap();
    }
    Smtp::wait(|| smtp.started.load(Ordering::SeqCst) == 10).await;
    let mail = service.clone();
    let waiting =
        tokio::spawn(async move { mail.send(Smtp::request("cancelled@example.com")).await });
    tokio::task::yield_now().await;
    waiting.abort();
    assert!(waiting.await.unwrap_err().is_cancelled());
    smtp.stop().await;
    Smtp::wait(|| service.status().queued < 1024).await;
    service
        .send(Smtp::request("cancelled@example.com"))
        .await
        .unwrap();
    service.shutdown().await;
}

#[tokio::test]
async fn smtp_delivery_times_out_after_sixty_seconds_and_releases_the_recipient() {
    let mut smtp = Smtp::start(true, false).await;
    let service = smtp.service();
    service
        .send(Smtp::request("timeout@example.com"))
        .await
        .unwrap();
    Smtp::wait(|| smtp.started.load(Ordering::SeqCst) == 1).await;
    pause();
    advance(Duration::from_secs(59)).await;
    assert_eq!(service.status().failed, 0);
    advance(Duration::from_secs(1)).await;
    tokio::task::yield_now().await;
    assert_eq!(service.status().failed, 1);
    assert_eq!(service.status().active, 0);
    service
        .send(Smtp::request("timeout@example.com"))
        .await
        .unwrap();
    resume();
    smtp.stop().await;
    service.shutdown().await;
}

#[tokio::test]
async fn smtp_rejection_is_not_retried_and_does_not_create_a_success_cooldown() {
    let mut smtp = Smtp::start(false, true).await;
    let service = smtp.service();
    service
        .send(Smtp::request("failed@example.com"))
        .await
        .unwrap();
    Smtp::wait(|| service.status().failed == 1).await;
    assert_eq!(service.status().sent, 0);
    assert_eq!(smtp.started.load(Ordering::SeqCst), 1);
    service
        .send(Smtp::request("failed@example.com"))
        .await
        .unwrap();
    Smtp::wait(|| service.status().failed == 2).await;
    assert_eq!(smtp.started.load(Ordering::SeqCst), 2);
    service.shutdown().await;
    smtp.stop().await;
}

#[tokio::test]
async fn shutdown_cancels_remaining_deliveries_after_the_bounded_drain() {
    let mut smtp = Smtp::start(true, false).await;
    let service = smtp.service();
    service
        .send(Smtp::request("shutdown@example.com"))
        .await
        .unwrap();
    Smtp::wait(|| smtp.started.load(Ordering::SeqCst) == 1).await;
    pause();
    let mail = service.clone();
    let stopped = tokio::spawn(async move { mail.shutdown().await });
    tokio::task::yield_now().await;
    advance(Duration::from_secs(59)).await;
    assert!(!stopped.is_finished());
    advance(Duration::from_secs(1)).await;
    stopped.await.unwrap();
    assert_eq!(service.status().active, 0);
    assert_eq!(service.status().queued, 0);
    assert!(matches!(
        service.send(Smtp::request("later@example.com")).await,
        Err(EmailError::Unavailable)
    ));
    resume();
    smtp.stop().await;
}

#[tokio::test]
async fn constructor_settings_control_concurrency_and_queue_capacity() {
    let mut smtp = Smtp::start(true, false).await;
    let mut settings = smtp.settings();
    settings.concurrency = 2;
    settings.queue_capacity = 3;
    settings.admission_timeout_seconds = 1;
    let service = smtp.service_with_settings(settings);
    for index in 0..5 {
        service
            .send(Smtp::request(format!("custom{index}@example.com")))
            .await
            .unwrap();
    }
    Smtp::wait(|| smtp.started.load(Ordering::SeqCst) == 2).await;
    assert_eq!(service.status().active, 2);
    assert_eq!(service.status().queued, 3);
    pause();
    assert!(matches!(
        service.send(Smtp::request("overflow@example.com")).await,
        Err(EmailError::QueueTimeout)
    ));
    resume();
    smtp.gate.add_permits(5);
    Smtp::wait(|| service.status().sent == 5).await;
    service.shutdown().await;
    smtp.stop().await;
}

#[tokio::test]
async fn zero_queue_or_concurrency_is_rejected_without_panicking() {
    let mut smtp = Smtp::start(false, false).await;
    let mut settings = smtp.settings();
    settings.concurrency = 0;
    assert!(MailingService::new(settings).is_err());
    let mut settings = smtp.settings();
    settings.queue_capacity = 0;
    assert!(MailingService::new(settings).is_err());
    smtp.stop().await;
}
