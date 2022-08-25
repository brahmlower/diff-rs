use crate::traits::{Patchable, MutIntoFragment};


#[derive(Debug)]
pub struct ScalarFragment<T> {
    value: [T; 2]
}

impl <T> ScalarFragment<T> {
    pub fn new(before: T, after: T) -> ScalarFragment<T> {
        ScalarFragment { value: [before, after] }
    }

    pub fn before(&self) -> &T {
        &self.value[0]
    }

    pub fn after(&self) -> &T {
        &self.value[1]
    }
}

impl <T: Sized + Copy> Patchable for T {
    type Fragment = ScalarFragment<T>;

    fn apply_fragment(&mut self, fragment: &Self::Fragment) {
        *self = *fragment.after();
    }

    fn revert_fragment(&mut self, fragment: &Self::Fragment) {
        *self = *fragment.before();
    }
}

impl <T: Sized + Copy + Patchable<Fragment = ScalarFragment<T>>> MutIntoFragment for T {
    type Fragment = ScalarFragment<T>;

    fn mut_into_fragment(&mut self, value: T) -> ScalarFragment<T> {
        let fragment: ScalarFragment<T> = ScalarFragment::new(*self, value);
        self.apply_fragment(&fragment);
        fragment
    }
}

// pub struct IndexableFragment<T> where T: Index {

// }