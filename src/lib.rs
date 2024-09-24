pub use is_odd_macro_proc::is_odd;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_using_identifier() {
        using_identifier(11);
    }

    fn using_identifier(n: u32) {
        assert!(is_odd!(n));
    }

    #[test]
    fn using_number() {
        assert!(!is_odd!(10));
    }

    #[test]
    fn using_negative_number() {
        assert!(!is_odd!(-10));
    }

    #[test]
    fn hexidecimal_number() {
        assert!(!is_odd!(0x10));
    }

    #[test]
    fn negative_hexidecimal_number() {
        assert!(!is_odd!(-0x10));
    }

}
