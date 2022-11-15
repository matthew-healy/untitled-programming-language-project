use std::{cell::RefCell, rc::Rc};

// Strongly inspired by the design of Nickel's environment, but
// with the added wrinkle of using de Bruijn indices.
#[derive(Debug, PartialEq)]
pub struct Env<T> {
    /// The current environment layer, in reverse order.
    current: Rc<Vec<T>>,
    /// Pointers to each of the previous environment layers.
    previous: RefCell<Option<Rc<Env<T>>>>,
}

impl<T> Clone for Env<T> {
    fn clone(&self) -> Self {
        if !(self.current.is_empty() || self.was_cloned()) {
            self.previous.replace_with(|old| {
                Some(Rc::new(Env {
                    current: self.current.clone(),
                    previous: RefCell::new(old.clone()),
                }))
            });
        }
        Self {
            current: Rc::new(Vec::new()),
            previous: self.previous.clone(),
        }
    }
}

impl<T: Clone + core::fmt::Debug> Env<T> {
    pub fn lookup(&self, n: usize) -> Option<T> {
        // n is the de Bruijn index of the variable, which means we need to
        // count backwards from the end of the environment. for example,
        // if n is 1, we need to take the second-to-last element.
        //
        // the complication here is that n might be in a previous layer, so
        // we calculate what the index of n would be were it in the current
        // layer, using checked arithmetic. if we get None, then we underflowed
        // so we need to check the next layer.
        let current_len = self.current.len();
        let poss_idx = current_len.checked_sub(1).and_then(|i| i.checked_sub(n));

        if let Some(idx) = poss_idx {
            self.current.get(idx).cloned()
        } else {
            self.previous
                .borrow()
                .as_ref()
                .and_then(|e| e.lookup(n - current_len))
        }
    }
}

impl<T> Env<T> {
    /// Create a new (empty) environment.
    pub fn new() -> Self {
        let current = Rc::new(Vec::new());
        let previous = RefCell::new(None);
        Env { current, previous }
    }

    /// Add a new binding to the environment.
    pub fn bind(&mut self, t: T) {
        if self.was_cloned() {
            self.current = Rc::new(Vec::new());
        }
        Rc::get_mut(&mut self.current).unwrap().push(t)
    }

    /// Pop the latest binding from the environment.
    pub fn unbind(&mut self) {
        Rc::get_mut(&mut self.current).unwrap().pop();
    }

    fn was_cloned(&self) -> bool {
        Rc::strong_count(&self.current) > 1
    }
}
