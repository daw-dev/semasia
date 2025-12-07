use std::cell::RefCell;
use std::rc::Rc;

struct SharedState<T> {
    value: Option<T>,
    // Callbacks just get notified. They must inspect 'value' themselves.
    callbacks: Vec<Box<dyn FnOnce()>>,
}

pub struct Inherited<T> {
    state: Rc<RefCell<SharedState<T>>>,
}

impl<T> Clone for Inherited<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

// Note: No T: Clone bound here!
impl<T: 'static> Inherited<T> {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(SharedState {
                value: None,
                callbacks: Vec::new(),
            })),
        }
    }

    /// Sets the value.
    /// This gives ownership of 'val' to the Promise internal storage.
    pub fn set(&self, val: T) {
        let mut state = self.state.borrow_mut();
        if state.value.is_some() {
            panic!("Promise resolved twice!");
        }
        state.value = Some(val);

        // Fire all callbacks
        let callbacks = std::mem::take(&mut state.callbacks);
        // We drop the lock so callbacks can modify/take the value
        drop(state);

        for cb in callbacks {
            cb();
        }
    }

    // --- ZERO-CLONE METHODS (Destructive) ---

    /// Consumes the value from this promise to create a new one.
    /// This empties the current promise. If you call this twice, it panics.
    pub fn map<U: 'static, F>(&self, mapper: F) -> Inherited<U>
    where
        F: 'static + FnOnce(T) -> U,
    {
        let new_prom = Inherited::new();
        let new_prom_clone = new_prom.clone();

        // We capture a weak clone of our own state
        let my_state = self.state.clone();

        self.register_callback(move || {
            // THE MAGIC: Option::take() moves the value out without cloning
            let val = my_state
                .borrow_mut()
                .value
                .take()
                .expect("Promise value moved twice!");

            let new_val = mapper(val);
            new_prom_clone.set(new_val);
        });

        new_prom
    }

    /// Used for Inherited Attributes (Linear flow down)
    pub fn inherit(other_promise: Self) -> Self {
        let prom = Self::new();
        let my_state = prom.state.clone();

        prom.register_callback(move || {
            let val = my_state
                .borrow_mut()
                .value
                .take()
                .expect("Value moved twice");
            other_promise.set(val);
        });
        prom
    }

    pub fn inherit_map<U, F>(other_promise: Inherited<U>, mapper: F) -> Self
    where
        U: 'static,
        F: FnOnce(T) -> U + 'static,
    {
        let prom = Self::new();
        let my_state = prom.state.clone();

        prom.register_callback(move || {
            let val = my_state
                .borrow_mut()
                .value
                .take()
                .expect("Value moved twice");
            other_promise.set(mapper(val));
        });
        prom
    }

    pub fn unwrap_consume(&self) -> T {
        self.state
            .borrow_mut()
            .value
            .take()
            .expect("Promise not resolved or already consumed")
    }

    pub fn map_ref<U: 'static, F>(&self, mapper: F) -> Inherited<U>
    where
        F: 'static + FnOnce(&T) -> U,
    {
        let new_prom = Inherited::new();
        let new_prom_clone = new_prom.clone();
        let my_state = self.state.clone();

        self.register_callback(move || {
            let new_val = mapper(my_state.borrow().value.as_ref().unwrap());
            new_prom_clone.set(new_val);
        });

        new_prom
    }

    // Helper
    fn register_callback<F: 'static + FnOnce()>(&self, cb: F) {
        let mut state = self.state.borrow_mut();
        if state.value.is_some() {
            drop(state);
            cb();
        } else {
            state.callbacks.push(Box::new(cb));
        }
    }
}

#[test]
fn inherited_test() {
    let p1: Inherited<usize> = Inherited::new();
    let p2 = p1.map(|i| {
        println!("got {i}, setting {}", i + 1);
        i + 1
    });
    let p3 = p2.map(|i| {
        println!("got {i}, setting {}", i + 1);
        i + 1
    });
    let p4: Inherited<String> = Inherited::inherit_map(p1, |s: String| s.len());
    let p5 = Inherited::inherit(p4);
    let p6 = Inherited::inherit(p5);
    p6.set("Hello World!".into());
    println!("{}", p3.unwrap_consume());
}
