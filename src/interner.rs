use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};
use typed_arena::Arena;

static INTERNER: Lazy<Interner> = Lazy::new(Interner::new);

/// An identifier for an interned string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(usize);

impl Id {
    /// Construct a new `Id`.
    pub fn new(s: impl AsRef<str>) -> Self {
        INTERNER.intern(s.as_ref())
    }

    /// Retrieves the string the `Id` was created with.
    pub fn name(&self) -> &str {
        INTERNER.lookup(*self)
    }
}

/// A string interner. Interning a string ensures we only have a single copy of
/// it across the interpreter's lifetime.
///
/// The implementation here borrows heavily from [Nickel](https://github.com/tweag/nickel)'s
/// identifier interner.
struct Interner<'a>(RwLock<InnerInterner<'a>>);

impl<'a> Interner<'a> {
    // Constructs a new [Interner]
    pub fn new() -> Self {
        Self(RwLock::new(InnerInterner::new()))
    }

    pub fn intern(&self, s: impl AsRef<str>) -> Id {
        self.0
            .write()
            .expect("Poisoned RWLock in Interner")
            .intern(s)
    }

    pub fn lookup(&self, id: Id) -> &str {
        // In order to read from the `InnerInterner` we have to acquire the
        // read lock. This gives it a lifetime of just this function body,
        // meaning that the `&str`s we get back can't be returned. But since
        // we know that these `str`s really live in the `InnerInterner`'s
        // arena, and won't be dropped until this struct is dropped, it's safe
        // to transmute the lifetime into `&'self`.
        unsafe {
            std::mem::transmute(
                self.0
                    .read()
                    .expect("Poisoned RWLock in Interner")
                    .lookup(id),
            )
        }
    }
}

struct InnerInterner<'a> {
    // Storage for interned strings.
    storage: Mutex<Arena<u8>>,
    // O(1) lookup for already-interned strings.
    str_lookup: HashMap<&'a str, Id>,
    // Look up a string from an Id's underlying u32.
    id_lookup: Vec<&'a str>,
}

impl<'a> InnerInterner<'a> {
    // Constructs a new [InnerInterner]
    fn new() -> Self {
        Self {
            storage: Mutex::new(Arena::new()),
            str_lookup: HashMap::new(),
            id_lookup: Vec::new(),
        }
    }

    // Intern a string and return the corresponding [Id]
    fn intern(&mut self, s: impl AsRef<str>) -> Id {
        if let Some(id) = self.str_lookup.get(s.as_ref()) {
            *id
        } else {
            // This transmutes the reference lifetime `&'arena str` into
            // `&'self str`. This is necessary since we can't statically prove
            // that the lifetime of the `interned` `&str` will live long enough
            // to insert into the lookup tables. It's okay to do this since the
            // lifetime of the arena is the same as the lifetime of the
            // `InnerInterner` itself, so we'll never end up in a situation
            // where the lookup tables have dangling references.
            let interned = unsafe {
                let storage = self
                    .storage
                    .lock()
                    .expect("Poisoned Mutex in InnerInterner");
                std::mem::transmute(storage.alloc_str(s.as_ref()))
            };

            let id = Id(self.id_lookup.len());
            self.id_lookup.push(interned);
            self.str_lookup.insert(interned, id);

            id
        }
    }

    // Lookup the [Id] and return a reference to the interned string.
    //
    // Panics if `id` was not created via a call to `intern`.
    fn lookup(&self, id: Id) -> &str {
        self.id_lookup[id.0]
    }
}

#[cfg(test)]
mod axioms {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn intern_then_lookup_returns_interned_string(s: String) -> bool {
            let interner = Interner::new();
            let id = interner.intern(&s);
            let lookup_result = interner.lookup(id);
            s == lookup_result
        }
    }

    quickcheck! {
        fn interning_same_string_twice_returns_same_id(s: String) -> bool {
            let interner = Interner::new();
            let id1 = interner.intern(&s);
            let id2 = interner.intern(&s);
            id1 == id2
        }
    }

    #[derive(Clone, Debug)]
    struct NonEqualStrings(String, String);

    impl quickcheck::Arbitrary for NonEqualStrings {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let s1 = String::arbitrary(g);
            let mut s2 = String::arbitrary(g);
            while s1 == s2 {
                s2 = String::arbitrary(g);
            }
            Self(s1, s2)
        }
    }

    quickcheck! {
        fn interning_different_strings_returns_different_ids(s: NonEqualStrings) -> bool {
            let interner = Interner::new();
            let id1 = interner.intern(s.0);
            let id2 = interner.intern(s.1);
            id1 != id2
        }
    }

    quickcheck! {
        fn looking_up_same_id_twice_returns_same_ref(s: String) -> bool {
            use std::ptr;

            let interner = Interner::new();
            let id = interner.intern(s);
            ptr::eq(interner.lookup(id), interner.lookup(id))
        }
    }
}
