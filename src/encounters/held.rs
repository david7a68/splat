//! A draft holder of the Encounters page (review 2 of PR 347).
//!
//! The page keeps member and editor results for a draft while some holder can still bring that
//! draft back. A holder is a `Held`: it reads as an `Option`, edits its draft in place through
//! `as_mut` or `iter_mut`, and changes which draft it holds only through `set` and `take`.
//! Both mark the change, and the page ends the results of a draft that no
//! holder keeps (`Encounters::end_unreachable_results`) only after such a change. There is no
//! `DerefMut`, so `Option::take` or an assignment of an `Option` does not compile on a holder.
#![warn(deprecated)]

/// A draft holder that marks each change of the draft it holds.
pub(super) struct Held<T> {
    value: Option<T>,
    changed: bool,
}

/// A new holder counts as a change: a whole holder swapped in by `std::mem::take` or
/// `std::mem::replace` (which bypass `set`) is marked too.
impl<T> Default for Held<T> {
    fn default() -> Self {
        Self {
            value: None,
            changed: true,
        }
    }
}

impl<T> From<Option<T>> for Held<T> {
    fn from(value: Option<T>) -> Self {
        Self {
            value,
            changed: true,
        }
    }
}

impl<T> std::ops::Deref for Held<T> {
    type Target = Option<T>;
    fn deref(&self) -> &Option<T> {
        &self.value
    }
}

impl<T> Held<T> {
    /// The held draft, to edit in place. The holder still holds the same draft.
    #[allow(
        clippy::should_implement_trait,
        reason = "an Option of the draft, as Option::as_mut gives it; not AsMut"
    )]
    pub(super) fn as_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }
    /// The held draft, to edit in place.
    pub(super) fn iter_mut(&mut self) -> std::option::IterMut<'_, T> {
        self.value.iter_mut()
    }
    /// Hold `value` in place of the draft held now.
    pub(super) fn set(&mut self, value: Option<T>) {
        self.value = value;
        self.changed = true;
    }
    /// Give up the held draft.
    pub(super) fn take(&mut self) -> Option<T> {
        self.changed = true;
        self.value.take()
    }
    /// Whether the held draft changed since the last call.
    pub(super) fn take_changed(&mut self) -> bool {
        std::mem::take(&mut self.changed)
    }
}
