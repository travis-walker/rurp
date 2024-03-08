use euclid::approxeq::ApproxEq;

pub mod bounds;
pub mod draw;
pub mod grid;
pub mod interpolate;
pub mod point;

/// Test if two f64s are equivalent.
///
/// Values are equivalent if they are within `f64::EPSILON` of each other.
/// This function also considers `f64::NAN == f64::NAN`, `f64::INFINITY == f64::INFINITY`, and `f64::NEG_INFINITY == f64::NEG_INFINITY`.
#[must_use]
pub fn equivalent(left: &f64, right: &f64) -> bool {
    left.approx_eq(right) || left.is_nan() && right.is_nan() || left == right
}

/// Normalize a value from one domain to another.
///
/// If src min == src max, the dst max is returned.
#[must_use]
pub(crate) fn normalize(value: f64, src_domain: &(f64, f64), dst_domain: &(f64, f64)) -> f64 {
    let (src_min, src_max) = src_domain;
    let (dst_min, dst_max) = dst_domain;
    if src_min == src_max {
        return *dst_max;
    }
    (value - src_min) * (dst_max - dst_min) / (src_max - src_min) + dst_min
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_equivalent() {
        assert!(!equivalent(&1.0, &2.0));
        assert!(equivalent(&1.0, &1.0));
        assert!(equivalent(&1.0, &1.000000000000001));
        assert!(equivalent(&0.0, &-0.0));
        assert!(equivalent(&f64::INFINITY, &f64::INFINITY));
        assert!(equivalent(&f64::NEG_INFINITY, &f64::NEG_INFINITY));
        assert!(!equivalent(&f64::INFINITY, &f64::NEG_INFINITY));
        assert!(equivalent(&f64::NAN, &f64::NAN));
    }

    #[test]
    fn test_normalize() {
        let src_domain = (0.0, 1.0);
        let dst_domain = (0.0, 100.0);

        assert_eq!(normalize(0.0, &src_domain, &dst_domain), 0.0);
        assert_eq!(normalize(1.0, &src_domain, &dst_domain), 100.0);
        assert_eq!(normalize(0.5, &src_domain, &dst_domain), 50.0);
    }

    #[test]
    fn test_normalize_2() {
        let src_domain = (-4.0, 1.0);
        let dst_domain = (100.0, 200.0);

        assert_eq!(normalize(-4.0, &src_domain, &dst_domain), 100.0);
        assert_eq!(normalize(-1.5, &src_domain, &dst_domain), 150.0);
        assert_eq!(normalize(1.0, &src_domain, &dst_domain), 200.0);
    }

    #[test]
    fn test_normalize_3() {
        let src_domain = (-3.0, -3.0);
        let dst_domain = (1.0, 65534.0);

        assert_eq!(normalize(-3.0, &src_domain, &dst_domain), 65534.0);
    }
}
