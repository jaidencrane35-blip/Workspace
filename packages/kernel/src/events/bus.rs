use std::sync::{Arc, Mutex};

use super::types::{DomainEvent, Event};

type EventHandler = Arc<dyn Fn(&DomainEvent) + Send + Sync>;

/// Synchronous, thread-safe internal event bus.
#[derive(Default, Clone)]
pub struct EventBus {
    subscribers: Arc<Mutex<Vec<EventHandler>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<F>(&self, handler: F)
    where
        F: Fn(&DomainEvent) + Send + Sync + 'static,
    {
        self.subscribers
            .lock()
            .expect("event bus subscriber lock")
            .push(Arc::new(handler));
    }

    pub fn publish(&self, event: DomainEvent) {
        log::info!("EVENT: {}", event.event_name());

        let subscribers = self
            .subscribers
            .lock()
            .expect("event bus subscriber lock");

        for handler in subscribers.iter() {
            handler(&event);
        }
    }

    pub fn subscriber_count(&self) -> usize {
        self.subscribers
            .lock()
            .expect("event bus subscriber lock")
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::types::WorkspaceStarted;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn publishes_event_to_subscribers() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        bus.subscribe(move |event| {
            assert_eq!(event.name(), "system.workspace.started");
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        bus.publish(DomainEvent::WorkspaceStarted(WorkspaceStarted {
            version: "0.1.0".into(),
        }));

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
