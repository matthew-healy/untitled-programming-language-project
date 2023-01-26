#[derive(Clone)]
pub struct Stack<T>(Vec<T>);

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack::new()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stacked = self.0.iter().rev().collect::<Vec<_>>();
        f.debug_tuple("Stack").field(&stacked).finish()
    }
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }

    /// Creates a Stack from a Vec which is already "stacked".
    /// i.e., the "first element" conceptually is at the end of the Vec.
    pub fn from_stacked_vec(v: Vec<T>) -> Stack<T> {
        Stack(v)
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }
}

impl<T: PartialEq> PartialEq for Stack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod stack_axioms {
    use quickcheck::{quickcheck, Arbitrary};

    impl Arbitrary for Stack<usize> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Stack(Vec::<usize>::arbitrary(g))
        }
    }

    use super::*;

    #[test]
    fn empty_stack_pops_nothing() {
        let mut stack: Stack<usize> = Stack::new();
        assert_eq!(None, stack.pop())
    }

    quickcheck! {
        fn pop_returns_last_pushed_element(t: usize) -> bool {
            let mut s = Stack::new();
            s.push(t);
            Some(t) == s.pop()
        }
    }

    quickcheck! {
        fn pop_after_push_leaves_stack_unchanged(s: Stack<usize>) -> bool {
            let before = s.clone();
            let mut s = s;
            s.push(0);
            s.pop();
            s == before
        }
    }
}
