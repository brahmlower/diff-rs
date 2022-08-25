use crate::traits::Patchable;

pub struct DiffBuilder<F> {
    fragments: Vec<F>
}

impl <F> DiffBuilder<F> {
    pub fn new() -> DiffBuilder<F> {
        DiffBuilder::default()
    }

    pub fn add_fragment(&mut self, fragment: F) -> &mut DiffBuilder<F> {
        self.fragments.push(fragment);
        self
    }

    pub fn finalize(&mut self) -> Diff<F> {
        let s = std::mem::take(self);
        Diff::new(s.fragments)
    }
}

impl<F> Default for DiffBuilder<F> {
    fn default() -> Self {
        DiffBuilder { fragments: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct Diff<F> {
    fragments: Vec<F>
}

impl<F> Diff<F> {
    pub fn new(fragments: Vec<F>) -> Diff<F> {
        Diff { fragments }
    }

    pub fn builder() -> DiffBuilder<F> {
        DiffBuilder::new()
    }

    pub fn fragments(&self) -> &Vec<F> {
        &self.fragments
    }
}

/// A trait automatically implemented for types that implement [Patchable]. Provides
/// functions for applying and reverting several Fragments at once via the [Diff] struct.
pub trait Diffable {
    type Fragment;

    /// Apply the list of Fragments to Self
    fn apply_diff(&mut self, diff: &Diff<Self::Fragment>);

    /// Revert the list of Fragments from Self
    fn revert_diff(&mut self, diff: &Diff<Self::Fragment>);
}

impl <F, T: Patchable<Fragment = F>>Diffable for T {
    type Fragment = F;

    fn apply_diff(&mut self, diff: &Diff<Self::Fragment>) {
        self.apply_fragments(diff.fragments())
    }

    fn revert_diff(&mut self, diff: &Diff<Self::Fragment>) {
        self.revert_fragments(diff.fragments())
    }
}
