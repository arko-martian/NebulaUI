use std::cell::RefCell;
use std::rc::Rc;

/// A reactive signal that notifies subscribers when its value changes
/// This is the foundation of Nebula UI's reactivity system
#[derive(Clone)]
pub struct Signal<T: Clone> {
    inner: Rc<RefCell<SignalInner<T>>>,
}

struct SignalInner<T: Clone> {
    value: T,
    subscribers: Vec<Box<dyn Fn(&T)>>,
}

impl<T: Clone> Signal<T> {
    /// Create a new signal with an initial value
    pub fn new(initial_value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(SignalInner {
                value: initial_value,
                subscribers: Vec::new(),
            })),
        }
    }

    /// Get the current value of the signal
    pub fn get(&self) -> T {
        self.inner.borrow().value.clone()
    }

    /// Set a new value and notify all subscribers
    pub fn set(&self, new_value: T) {
        let mut inner = self.inner.borrow_mut();
        inner.value = new_value.clone();
        
        // Notify all subscribers
        for subscriber in &inner.subscribers {
            subscriber(&new_value);
        }
    }

    

    /// Update the value using a function and notify subscribers
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&T) -> T,
    {
        let new_value = {
            let inner = self.inner.borrow();
            f(&inner.value)
        };
        self.set(new_value);
    }

    /// Subscribe to changes in this signal
    /// Returns a subscription ID (currently just the index)
    pub fn subscribe<F>(&self, callback: F) -> usize
    where
        F: Fn(&T) + 'static,
    {
        let mut inner = self.inner.borrow_mut();
        inner.subscribers.push(Box::new(callback));
        inner.subscribers.len() - 1
    }

    /// Get the number of subscribers
    pub fn subscriber_count(&self) -> usize {
        self.inner.borrow().subscribers.len()
    }
}

// Implement Debug for Signal
impl<T: Clone + std::fmt::Debug> std::fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal")
            .field("value", &self.get())
            .field("subscribers", &self.subscriber_count())
            .finish()
    }
}

// Implement PartialEq for Signal (compares values only)
impl<T: Clone + PartialEq> PartialEq for Signal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn signal_creation_and_get() {
        let signal = Signal::new(42);
        assert_eq!(signal.get(), 42);
    }

    #[test]
    fn signal_set_updates_value() {
        let signal = Signal::new(10);
        signal.set(20);
        assert_eq!(signal.get(), 20);
    }

    #[test]
    fn signal_update_with_function() {
        let signal = Signal::new(5);
        signal.update(|v| v + 10);
        assert_eq!(signal.get(), 15);
    }

    #[test]
    fn signal_notifies_subscribers() {
        let signal = Signal::new(0);
        let received = Rc::new(RefCell::new(Vec::new()));
        let received_clone = received.clone();

        signal.subscribe(move |value| {
            received_clone.borrow_mut().push(*value);
        });

        signal.set(1);
        signal.set(2);
        signal.set(3);

        assert_eq!(*received.borrow(), vec![1, 2, 3]);
    }

    #[test]
    fn signal_multiple_subscribers() {
        let signal = Signal::new(0);
        let count1 = Rc::new(RefCell::new(0));
        let count2 = Rc::new(RefCell::new(0));

        let count1_clone = count1.clone();
        let count2_clone = count2.clone();

        signal.subscribe(move |_| {
            *count1_clone.borrow_mut() += 1;
        });

        signal.subscribe(move |_| {
            *count2_clone.borrow_mut() += 1;
        });

        signal.set(1);
        signal.set(2);

        assert_eq!(*count1.borrow(), 2);
        assert_eq!(*count2.borrow(), 2);
    }

    #[test]
    fn signal_clone_shares_state() {
        let signal1 = Signal::new(100);
        let signal2 = signal1.clone();

        signal1.set(200);
        assert_eq!(signal2.get(), 200);

        signal2.set(300);
        assert_eq!(signal1.get(), 300);
    }

    #[test]
    fn signal_subscriber_count() {
        let signal = Signal::new(0);
        assert_eq!(signal.subscriber_count(), 0);

        signal.subscribe(|_| {});
        assert_eq!(signal.subscriber_count(), 1);

        signal.subscribe(|_| {});
        assert_eq!(signal.subscriber_count(), 2);
    }

    #[test]
    fn signal_with_string() {
        let signal = Signal::new("Hello".to_string());
        assert_eq!(signal.get(), "Hello");

        signal.set("World".to_string());
        assert_eq!(signal.get(), "World");
    }

    #[test]
    fn signal_debug_format() {
        let signal = Signal::new(42);
        let debug_str = format!("{:?}", signal);
        assert!(debug_str.contains("Signal"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn signal_equality() {
        let signal1 = Signal::new(42);
        let signal2 = Signal::new(42);
        let signal3 = Signal::new(100);

        assert_eq!(signal1, signal2);
        assert_ne!(signal1, signal3);
    }
}
