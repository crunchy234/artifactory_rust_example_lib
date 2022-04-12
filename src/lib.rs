use num::PrimInt;

pub fn add_two<T: PrimInt>(a: T) -> T {
    a + (T::one() + T::one())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_add_two_simple() {
        assert_eq!(4, add_two(2));
    }

    #[rstest]
    fn test_add_two_different_types() {
        assert_eq!(4u32, add_two(2u32));
        assert_eq!(4u64, add_two(2u64));
        assert_eq!(4i8, add_two(2i8));
        assert_eq!(4i64, add_two(2i64));
    }
}
