pub struct Env<T> {
    bindings: Vec<T>,
}

impl <T> Env<T> {
    pub fn new() -> Self {
        let bindings = Vec::new();
        Env { bindings }
    }

    pub fn bind(&mut self, t: T) {
        self.bindings.push(t)
    }

    pub fn unbind(&mut self) {
        self.bindings.pop();
    }

    pub fn lookup(&self, n: usize) -> Option<&T> {
        let end = self.bindings.len() - 1;
        let idx = end - n;
        self.bindings.get(idx)
    }
}