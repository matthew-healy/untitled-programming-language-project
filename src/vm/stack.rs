#[derive(Clone, Debug)]
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
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
