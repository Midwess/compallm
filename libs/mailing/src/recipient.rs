use compallm_core::mail::EmailError;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    sync::{Arc, Mutex, MutexGuard},
    time::Duration,
};
use tokio::time::Instant;

enum Recipient {
    Pending(u64),
    Sent(Instant),
}
#[derive(Default)]
struct State {
    recipients: HashMap<String, Recipient>,
    expirations: BinaryHeap<Reverse<(Instant, String)>>,
    sequence: u64,
}

pub(crate) struct Recipients {
    state: Mutex<State>,
    cooldown: Duration,
}

impl Recipients {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            state: Mutex::default(),
            cooldown,
        }
    }
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub fn reserve(self: &Arc<Self>, email: String) -> Result<Reservation, EmailError> {
        let mut state = self.lock();
        let now = Instant::now();
        while state
            .expirations
            .peek()
            .is_some_and(|Reverse((at, _))| *at <= now)
        {
            let Reverse((at, email)) = state.expirations.pop().expect("expiration was present");
            if matches!(state.recipients.get(&email),Some(Recipient::Sent(until)) if *until==at) {
                state.recipients.remove(&email);
            }
        }
        if state.recipients.contains_key(&email) {
            return Err(EmailError::Throttled);
        }
        state.sequence = state.sequence.wrapping_add(1);
        let id = state.sequence;
        state
            .recipients
            .insert(email.clone(), Recipient::Pending(id));
        Ok(Reservation {
            recipients: self.clone(),
            email,
            id,
        })
    }
}

/// Cancelling admission, dropping a queued job, or failed delivery releases its recipient.
pub(crate) struct Reservation {
    recipients: Arc<Recipients>,
    email: String,
    id: u64,
}
impl Reservation {
    pub fn sent(self) {
        let mut state = self.recipients.lock();
        if matches!(state.recipients.get(&self.email),Some(Recipient::Pending(id)) if *id==self.id)
        {
            let until = Instant::now() + self.recipients.cooldown;
            state
                .recipients
                .insert(self.email.clone(), Recipient::Sent(until));
            state.expirations.push(Reverse((until, self.email.clone())));
        }
    }
}
impl Drop for Reservation {
    fn drop(&mut self) {
        let mut state = self.recipients.lock();
        if matches!(state.recipients.get(&self.email),Some(Recipient::Pending(id)) if *id==self.id)
        {
            state.recipients.remove(&self.email);
        }
    }
}
