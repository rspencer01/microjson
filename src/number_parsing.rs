const POWERS_OF_TEN: [f32; 39] = [
    1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
    1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 1e23, 1e24, 1e25, 1e26, 1e27, 1e28, 1e29, 1e30, 1e31, 1e32,
    1e33, 1e34, 1e35, 1e36, 1e37, 1e38,
];
pub fn parse_float(value: &str) -> Result<f32, ()> {
    let mut chars = value.chars().peekable();
    let positive = match chars.peek() {
        Some('-') => {
            chars.next();
            if chars.peek().is_none() {
                return Err(());
            }
            false
        }
        Some(_) => true,
        _ => return Err(()),
    };
    let mut value = 0.;
    let mut next_digit_size = 0.1;
    let mut exponent = 0;
    let mut seen_decimal = false;
    let mut seen_exponent = false;
    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                if !seen_decimal {
                    exponent += 1;
                }
                value += next_digit_size * c.to_digit(10).unwrap() as f32;
                next_digit_size *= 0.1;
            }
            '.' => {
                if seen_decimal || seen_exponent {
                    return Err(());
                } else {
                    seen_decimal = true;
                }
            }
            'e' | 'E' => {
                seen_exponent = true;
                break;
            }
            _ => return Err(()),
        }
    }
    if seen_exponent {
        let mut explicit_exponent = 0;
        let exponent_positive = match chars.peek() {
            Some('-' | '+') => chars.next() == Some('+'),
            Some('0'..='9') => true,
            _ => return Err(()),
        };
        for c in chars {
            match c {
                '0'..='9' => {
                    explicit_exponent *= 10;
                    explicit_exponent += c.to_digit(10).unwrap() as i32;
                }
                _ => return Err(()),
            }
        }
        if !exponent_positive {
            explicit_exponent = -explicit_exponent;
        }
        exponent += explicit_exponent;
    }
    if exponent < -37 || exponent > 38 {
        return Err(());
    }
    if 0 < exponent {
        value = value * POWERS_OF_TEN[exponent as usize];
    } else if exponent < 0 {
        value = value / POWERS_OF_TEN[exponent.abs() as usize];
    }

    if !positive {
        value = -value;
    }
    Ok(value)
}

#[cfg(test)]
mod test {
    extern crate std;
    use super::parse_float;
    use std::string::ToString;

    fn close(a: f32, b: f32) -> bool {
        if f32::min(a, b) == 0. {
            a == 0. && b == 0.
        } else {
            f32::abs((a - b) / f32::min(a, b)) < 0.000001
        }
    }

    #[test]
    fn errors() {
        assert!(parse_float("").is_err());
        assert!(parse_float("-").is_err());
        assert!(parse_float("foo").is_err());
        assert!(parse_float("1.23.4").is_err());
        assert!(parse_float("10e0.3").is_err());
    }

    #[test]
    fn zeros() {
        assert_eq!(parse_float("0").unwrap(), 0f32);
        assert_eq!(parse_float("-0").unwrap(), 0f32);
        assert_eq!(parse_float("0e1").unwrap(), 0f32);
        assert_eq!(parse_float("-0e1").unwrap(), 0f32);
        assert_eq!(parse_float("0e-1").unwrap(), 0f32);
        assert_eq!(parse_float("-0e-1").unwrap(), 0f32);
        assert_eq!(parse_float("0E1").unwrap(), 0f32);
        assert_eq!(parse_float("-0E1").unwrap(), 0f32);
        assert_eq!(parse_float("0E+1").unwrap(), 0f32);
        assert_eq!(parse_float("-0E+1").unwrap(), 0f32);
        assert_eq!(parse_float("0E-1").unwrap(), 0f32);
        assert_eq!(parse_float("-0E-1").unwrap(), 0f32);
    }

    #[test]
    fn exponents() {
        assert!(close(parse_float("1").unwrap(), 1f32));
        assert!(close(parse_float("10").unwrap(), 10f32));
        assert!(close(parse_float("100").unwrap(), 100f32));
        assert!(close(parse_float("1e2").unwrap(), 100f32));
        assert!(close(parse_float("1E2").unwrap(), 100f32));
        assert!(close(parse_float("0.1e3").unwrap(), 100f32));
        assert!(close(parse_float("1000e-1").unwrap(), 100f32));
        assert!(close(parse_float("1E00").unwrap(), 1f32));
        assert!(close(parse_float("1E001").unwrap(), 10f32));
    }

    #[test]
    fn manual() {
        assert!(close(parse_float("3.141592").unwrap(), 3.141592f32));
        assert!(close(parse_float("3.141592e-2").unwrap(), 0.03141592f32));
        assert!(close(parse_float("3.141592e+2").unwrap(), 314.1592f32));
        assert!(close(parse_float("3").unwrap(), 3f32));
    }

    #[test]
    fn logistic() {
        let mut t = 0.234;
        for _ in 0..400 {
            let s = 2. * (t - 0.5);
            assert!(close(s, parse_float(&s.to_string()).unwrap()));
            t = 3.8 * t * (1. - t);
        }
    }

    #[test]
    fn integers() {
        for t in -2000000..2000000 {
            assert!(close(t as f32, parse_float(&t.to_string()).unwrap()));
        }
    }

    #[test]
    fn overflows() {
        assert!(parse_float("1e39").is_err());
        assert!(parse_float("10000000000000000000000000000000000000000").is_err());
        assert!(parse_float("-10000000000000000000000000000000000000000").is_err());
        assert!(parse_float("1e-39").is_err());
    }
}
