//! Event dispatcher — broadcasts kernel events to subscribers via channels.
//!
//! STEP 2: tokio::broadcast channel for multi-subscriber event distribution.

use tokio::sync::broadcast;
use tracing::info;

use crate::events::KernelEvent;

/// Event dispatcher for kernel events with multiple subscribers.
pub struct EventDispatcher {
    sender: broadcast::Sender<KernelEvent>,
    capacity: usize,
}

/// A subscription handle that receives kernel events.
pub struct EventSubscription {
    receiver: broadcast::Receiver<KernelEvent>,
}

impl EventDispatcher {
    /// Create a new dispatcher with specified channel capacity.
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        info!("KernelEvent dispatcher created (capacity={capacity})");
        Self { sender, capacity }
    }

    /// Publish an event to all subscribers.
    pub fn publish(&self, event: KernelEvent) -> usize {
        self.sender.send(event).unwrap_or(0)
    }

    /// Subscribe to kernel events.
    pub fn subscribe(&self) -> EventSubscription {
        EventSubscription {
            receiver: self.sender.subscribe(),
        }
    }

    /// Number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Channel capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl EventSubscription {
    /// Receive the next event (async).
    pub async fn recv(&mut self) -> Result<KernelEvent, broadcast::error::RecvError> {
        self.receiver.recv().await
    }

    /// Try to receive without blocking.
    pub fn try_recv(&mut self) -> Result<KernelEvent, broadcast::error::TryRecvError> {
        self.receiver.try_recv()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{KernelEvent, KernelEventKind};

    #[tokio::test]
    async fn dispatcher_publish_subscribe() {
        let dispatcher = EventDispatcher::new(32);
        let mut sub = dispatcher.subscribe();

        let event = KernelEvent::new(
            42, 42, "test",
            KernelEventKind::PtraceAttach { target_pid: 1 },
        );

        dispatcher.publish(event);

        let received = sub.recv().await.unwrap();
        assert_eq!(received.pid, 42);
    }

    #[tokio::test]
    async fn dispatcher_multiple_subscribers() {
        let dispatcher = EventDispatcher::new(32);
        let mut sub1 = dispatcher.subscribe();
        let mut sub2 = dispatcher.subscribe();

        assert_eq!(dispatcher.subscriber_count(), 2);

        let event = KernelEvent::new(
            1, 1, "test",
            KernelEventKind::FileOpen { path: "/tmp".into(), flags: 0 },
        );

        let receivers = dispatcher.publish(event);
        assert_eq!(receivers, 2);

        let r1 = sub1.recv().await.unwrap();
        let r2 = sub2.recv().await.unwrap();
        assert_eq!(r1.pid, r2.pid);
    }

    #[test]
    fn default_capacity() {
        let dispatcher = EventDispatcher::default();
        assert_eq!(dispatcher.capacity(), 1024);
    }
}
