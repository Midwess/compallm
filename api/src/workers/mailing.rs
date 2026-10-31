use crate::di::Di;
use async_trait::async_trait;
use compallm_mailing::MailingService;
use loco_rs::{app::AppContext, bgworker::BackgroundWorker};

pub struct MailingWorker {
    service: &'static MailingService,
}

impl MailingWorker {
    pub fn new(service: &'static MailingService) -> Self {
        Self { service }
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
