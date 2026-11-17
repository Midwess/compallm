#![allow(dead_code)]
use compallm_core::{
    onboarding::{Context, Progress, Settings},
    persistent::onboarding::{OnboardingRepository, OnboardingRepositoryError as Error},
};
use std::{collections::HashMap, sync::Mutex};
#[derive(Default)]
pub struct MemoryOnboarding {
    values: Mutex<HashMap<(u64, u64), Context>>,
}
#[async_trait::async_trait]
impl OnboardingRepository for MemoryOnboarding {
    async fn settings(&self, _: u64) -> Result<Option<Settings>, Error> {
        Ok(None)
    }
    async fn save_settings(&self, _: u64, value: Settings) -> Result<Settings, Error> {
        Ok(value)
    }
    async fn initialize(&self, org: u64, user: u64, progress: Progress) -> Result<(), Error> {
        self.values
            .lock()
            .unwrap()
            .entry((org, user))
            .or_insert(Context {
                required: true,
                progress,
            });
        Ok(())
    }
    async fn find(&self, org: u64, user: u64, _: bool) -> Result<Option<Context>, Error> {
        Ok(self.values.lock().unwrap().get(&(org, user)).cloned())
    }
    async fn save_progress(&self, org: u64, user: u64, progress: Progress) -> Result<(), Error> {
        self.values
            .lock()
            .unwrap()
            .get_mut(&(org, user))
            .unwrap()
            .progress = progress;
        Ok(())
    }
    async fn complete(&self, org: u64, user: u64) -> Result<(), Error> {
        self.values
            .lock()
            .unwrap()
            .get_mut(&(org, user))
            .unwrap()
            .required = false;
        Ok(())
    }
}
pub fn repository() -> &'static MemoryOnboarding {
    Box::leak(Box::new(MemoryOnboarding::default()))
}
