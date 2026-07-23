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

        let handlers: Vec<EventHandler> = {
            let subscribers = self
                .subscribers
                .lock()
                .expect("event bus subscriber lock");
            subscribers.clone()
        };

        for handler in handlers {
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
            intent: None,
            capability: None,
        }));

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn dispatches_to_multiple_subscribers() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..3 {
            let counter_clone = Arc::clone(&counter);
            bus.subscribe(move |_| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        bus.publish(DomainEvent::WorkspaceStarted(WorkspaceStarted {
            version: "0.1.0".into(),
            intent: None,
            capability: None,
        }));

        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn nested_publish_does_not_deadlock() {
        use crate::events::types::WorkspaceShutdown;

        let bus = EventBus::new();
        let bus_clone = bus.clone();
        let started_count = Arc::new(AtomicUsize::new(0));
        let shutdown_seen = Arc::new(AtomicUsize::new(0));

        let started = Arc::clone(&started_count);
        bus.subscribe(move |event| {
            if event.name() == "system.workspace.started"
                && started.fetch_add(1, Ordering::SeqCst) == 0
            {
                bus_clone.publish(DomainEvent::WorkspaceShutdown(WorkspaceShutdown {
                    actor: None,
                    intent: None,
                    capability: None,
                }));
            }
        });

        let shutdown = Arc::clone(&shutdown_seen);
        bus.subscribe(move |event| {
            if event.name() == "system.workspace.shutdown" {
                shutdown.fetch_add(1, Ordering::SeqCst);
            }
        });

        bus.publish(DomainEvent::WorkspaceStarted(WorkspaceStarted {
            version: "outer".into(),
            intent: None,
            capability: None,
        }));

        assert_eq!(started_count.load(Ordering::SeqCst), 1);
        assert_eq!(shutdown_seen.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn subscriber_mutation_during_dispatch_is_safe() {
        let bus = EventBus::new();
        let bus_clone = bus.clone();

        bus.subscribe(move |_| {
            bus_clone.subscribe(|_| {});
        });

        bus.publish(DomainEvent::WorkspaceStarted(WorkspaceStarted {
            version: "0.1.0".into(),
            intent: None,
            capability: None,
        }));

        assert_eq!(bus.subscriber_count(), 2);
    }
}
