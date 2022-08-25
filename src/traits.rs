
pub trait Patchable {
    type Fragment;

    fn apply_fragment(&mut self, fragment: &Self::Fragment);

    fn apply_fragments(&mut self, fragments: &Vec<Self::Fragment>) {
        fragments
            .iter()
            .for_each(|f| self.apply_fragment(&f))
    }

    fn revert_fragment(&mut self, fragment: &Self::Fragment);

    fn revert_fragments(&mut self, fragments: &Vec<Self::Fragment>) {
        fragments
            .iter()
            .for_each(|f| self.revert_fragment(&f));
    }
}

pub trait MutIntoFragment: Sized + Patchable {
    type Fragment;

    fn mut_into_fragment(&mut self, value: Self) -> <Self as MutIntoFragment>::Fragment;
}
