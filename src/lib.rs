pub use is_odd_macro::is_odd;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn using_identifier() {
        let n = 11;
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
