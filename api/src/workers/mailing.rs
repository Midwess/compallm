use crate::di::Di;
use async_trait::async_trait;
use compallm_mailing::MailingService;
use loco_rs::{app::AppContext, bgworker::BackgroundWorker};
use std::sync::Mutex;
use tokio::task::JoinHandle;

pub struct MailingWorker {
    service: &'static MailingService,
    task: Mutex<Option<JoinHandle<()>>>,
}

impl MailingWorker {
    pub fn new(service: &'static MailingService) -> Self {
        Self {
            service,
            task: Mutex::new(None),
        }
    }

    pub fn start(&'static self) {
        let mut task = self
            .task
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if task.is_none() {
            *task = Some(tokio::spawn(async move {
                if self.perform(()).await.is_err() {
                    tracing::error!("mailing worker stopped unexpectedly");
                }
            }));
        }
    }

    pub async fn shutdown(&self) {
        self.service.shutdown().await;
        let task = self
            .task
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        if let Some(task) = task {
            let _ = task.await;
        }
    }
}

#[async_trait]
impl BackgroundWorker<()> for &'static MailingWorker {
    fn build(_ctx: &AppContext) -> Self {
        Di::instance()
            .mailing_worker()
            .expect("mailing must be configured before worker startup")
    }

    async fn perform(&self, _args: ()) -> loco_rs::Result<()> {
        self.service.run().await;
        Ok(())
    }
}
