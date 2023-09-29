use std::fmt;

use crate::interner;

use super::{Error, Existential, Type};

/// An element of the typing context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Element {
    /// A variable along with its type.
    TypedVariable(interner::Id, Type),
    /// A type that must exist, but whose identity isn't yet known.
    Existential(Existential),
    /// A former existential, now solved.
    Solved(Existential, Type),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::TypedVariable(i, t) => write!(f, "{}: {t}", i.name()),
            Element::Existential(e) => write!(f, "{e}"),
            Element::Solved(e, t) => write!(f, "{e} = {t}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The typing context
pub(crate) struct Ctx {
    /// The elements of the typing context.
    ///
    /// NOTE: In order for variable shadowing to work correctly the typing
    /// context must always be searched from the back. That means that most
    /// methods which iterate through `elements` will do so using a reversed
    /// iterator.
    elements: Vec<Element>,
}

impl fmt::Display for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (idx, e) in self.elements.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{e}")?;
        }
        write!(f, "]")
    }
}

impl Ctx {
    pub(crate) fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub(crate) fn add(self, element: Element) -> Self {
        let mut elements = self.elements;
        elements.push(element);
        Self { elements }
    }

    pub(crate) fn split_at(&self, element: &Element) -> Result<(Ctx, Ctx), Error> {
        self.first_appearance_from_back(element)
            .map(|i| {
                let (left, right) = self.elements.split_at(i);
                (
                    Ctx {
                        elements: left.to_vec(),
                    },
                    Ctx {
                        elements: right.to_vec(),
                    },
                )
            })
            .ok_or(Error::Internal(format!(
                "split_at called with non-existent element {element:?}"
            )))
    }

    pub(crate) fn insert_in_place(
        self,
        element: Element,
        replacements: &[Element],
    ) -> Result<Self, Error> {
        self.first_appearance_from_back(&element)
            .map(|i| {
                let mut elements = self.elements;
                let _ = elements
                    .splice(i..=i, replacements.into_iter().cloned())
                    .count();
                Ctx { elements }
            })
            .ok_or(Error::Internal(format!(
                "insert_in_place called with non-existent element: {element:?}"
            )))
    }

    pub(crate) fn drop(self, element: &Element) -> Result<Self, Error> {
        self.first_appearance_from_back(element)
            .map(|i| {
                let mut elements = self.elements;
                let _ = elements.split_off(i);
                Ctx { elements }
            })
            .ok_or(Error::Internal(format!(
                "drop called with non-existent element: {element:?}"
            )))
    }

    pub(crate) fn get_solved(&self, alpha: &Existential) -> Option<&Type> {
        self.elements.iter().rev().find_map(|e| match e {
            Element::Solved(a, ty) if alpha == a => Some(ty),
            _ => None,
        })
    }

    pub(crate) fn has_existential(&self, alpha: &Existential) -> bool {
        self.elements.iter().any(|e| match e {
            Element::Existential(a) if a == alpha => true,
            _ => false,
        })
    }

    pub(crate) fn get_annotation(&self, x: interner::Id) -> Result<&Type, Error> {
        self.elements
            .iter()
            .rev()
            .find_map(|e| match e {
                Element::TypedVariable(v, t) if v == &x => Some(t),
                _ => None,
            })
            .ok_or(Error::UnboundVariable(x))
    }

    pub(crate) fn check_type_well_formed(&self, t: &Type) -> Result<(), Error> {
        match t {
            Type::Primitive(_) => Ok(()),
            Type::Existential(a) => {
                if self.has_existential(a) || self.get_solved(a).is_some() {
                    Ok(())
                } else {
                    Err(Error::IllFormedType(t.clone()))
                }
            }
            Type::Arrow(from, to) => {
                self.check_type_well_formed(from)?;
                self.check_type_well_formed(to)
            }
        }
    }

    fn first_appearance_from_back(&self, element: &Element) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .position(|e1| e1 == element)
            .map(|p| self.elements.len() - p - 1)
    }
}
