use std::{cell::RefCell, rc::Rc};

// Strongly inspired by the design of Nickel's environment, but
// with the added wrinkle of using de Bruijn indices.
#[derive(Debug, PartialEq)]
pub struct Env<T> {
    /// The current environment layer, in reverse order.
    current: Rc<RefCell<Vec<T>>>,
    /// Pointers to each of the previous environment layers.
    previous: RefCell<Option<Rc<Env<T>>>>,
}

impl<T> Clone for Env<T> {
    fn clone(&self) -> Self {
        if !self.current.borrow().is_empty() && !self.was_cloned() {
            self.previous.replace_with(|old| {
                Some(Rc::new(Env {
                    current: Rc::new(RefCell::new(self.current.take())),
                    previous: RefCell::new(old.clone()),
                }))
            });
        }
        Self {
            current: Rc::new(RefCell::new(Vec::new())),
            previous: self.previous.clone(),
        }
    }
}

impl<T: Clone> Env<T> {
    pub fn lookup(&self, n: usize) -> Option<T> {
        self.do_at_position(n, |t| t.clone())
    }
}

impl<T> Default for Env<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Env<T> {
    /// Create a new (empty) environment.
    pub fn new() -> Self {
        let current = Rc::new(RefCell::new(Vec::new()));
        let previous = RefCell::new(None);
        Env { current, previous }
    }

    /// Add a new binding to the environment.
    pub fn bind(&mut self, t: T) {
        if self.was_cloned() {
            self.current = Rc::new(RefCell::new(Vec::new()));
        }
        self.current.borrow_mut().push(t)
    }

    /// Pop the latest binding from this environment.
    pub fn unbind(&mut self) {
        self.current.borrow_mut().pop();
    }

    fn was_cloned(&self) -> bool {
        Rc::strong_count(&self.current) > 1
    }

    fn position_of_first_match(&self, pred: &impl Fn(&T) -> bool) -> Option<usize> {
        let pos = self.current.borrow().iter().rev().position(pred);
        pos.or_else(|| {
            let current_len = self.current.borrow().len();
            self.previous
                .borrow()
                .as_deref()
                .and_then(|prev| prev.position_of_first_match(pred))
                .map(|n| n + current_len)
        })
    }

    fn do_at_position<F, U>(&self, n: usize, f: F) -> Option<U>
    where
        F: FnOnce(&mut T) -> U,
    {
        // n is the de Bruijn index of the variable, which means we need to
        // count backwards from the end of the environment. for example,
        // if n is 1, we need to take the second-to-last element.
        //
        // the complication here is that n might be in a previous layer, so
        // we calculate what the index of n would be were it in the current
        // layer, using checked arithmetic. if we get None, then we underflowed
        // so we need to check the next layer.
        let current_len = self.current.borrow().len();
        let poss_idx = current_len.checked_sub(1).and_then(|i| i.checked_sub(n));
        if let Some(idx) = poss_idx {
            Some(f(&mut self.current.borrow_mut()[idx]))
        } else {
            let prev = self.previous.borrow();
            if let Some(prev) = prev.as_ref() {
                prev.do_at_position(n - current_len, f)
            } else {
                None
            }
        }
    }
}

impl<T: Clone> Env<RefCell<T>> {
    pub fn update_first_match(&mut self, new_val: T, pred: impl Fn(&T) -> bool) {
        let pos = self.position_of_first_match(&|r| pred(&r.borrow()));
        if let Some(pos) = pos {
            self.update(pos, new_val);
        }
    }

    pub fn update(&self, n: usize, new_val: T) {
        self.do_at_position(n, |r| {
            r.replace(new_val.clone());
        });
    }
}
