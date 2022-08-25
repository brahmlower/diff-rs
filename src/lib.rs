mod diff;
mod traits;
mod fragment;

pub use crate::diff::{
    Diff,
    Diffable,
};

pub use crate::traits::{
    Patchable,
    MutIntoFragment
};

pub use crate::fragment::{
    ScalarFragment,
};

#[cfg(test)]
mod tests {
    use crate::{Patchable, MutIntoFragment};

    #[test]
    fn creates_diff_for_u8() {
        let mut start: u8 = 1;

        let diff = start.mut_into_fragment(2u8);
        assert_eq!(start, 2u8);

        start.revert_fragment(&diff);
        assert_eq!(start, 1u8);

        start.apply_fragment(&diff);
        assert_eq!(start, 2u8);
    }

    #[test]
    fn creates_diff_for_i32() {
        let mut start: i32 = 9001;

        let diff = start.mut_into_fragment(123);
        assert_eq!(start, 123);

        start.revert_fragment(&diff);
        assert_eq!(start, 9001);

        start.apply_fragment(&diff);
        assert_eq!(start, 123);
    }

    #[test]
    fn creates_diff_for_str() {
        let mut start: &str = "foo bar baz";

        let diff = start.mut_into_fragment("qux corge");
        assert_eq!(start, "qux corge");

        start.revert_fragment(&diff);
        assert_eq!(start, "foo bar baz");

        start.apply_fragment(&diff);
        assert_eq!(start, "qux corge");
    }

    #[test]
    fn creates_diff_for_array() {
        let mut start: [i32; 3] = [1, 2, 3];

        let diff = start.mut_into_fragment([4, 5, 6]);
        assert_eq!(start, [4, 5, 6]);

        start.revert_fragment(&diff);
        assert_eq!(start, [1, 2, 3]);

        start.apply_fragment(&diff);
        assert_eq!(start, [4, 5, 6]);
    }

    // #[test]
    // fn creates_diff_for_vec_i32() {
    //     let mut start: Vec<i32> = vec![1, 2, 3];

    //     let diff = start.mut_and_diff(vec![4, 5, 6]);
    //     assert_eq!(start, vec![4, 5, 6]);

    //     start.revert_diff(&diff);
    //     assert_eq!(start, "foo bar baz");

    //     start.apply_diff(&diff);
    //     assert_eq!(start, "qux corge");
    // }


}
