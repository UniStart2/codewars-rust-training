use std::ops::{Div, Mul};

///u1 = (1 / 1!) * (1!)
// u2 = (1 / 2!) * (1! + 2!)
// u3 = (1 / 3!) * (1! + 2! + 3!)
// ...
// un = (1 / n!) * (1! + 2! + 3! + ... + n!)
// U(n) = 1 + U(n-1) / n
// fn going(n: i32) -> f64 {
//     if n == 1 {
//         return 1.0f64;
//     }
//
//     1.0f64 + (going(n-1) / n as f64).mul(1e6).floor().div(1e6)
// }

fn going(n: i32) -> f64 {
    let mut sum = 0.0;
    let mut factorial = 1.0;
    for i in (1..=n).rev() {
        sum += factorial;
        factorial /= i as f64;
    }
    (sum * 1e6).floor() / 1e6
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::float_eq;

    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-12;
        let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(res, "Expected value must be near: {:e} but was:{:e}", expected, actual);
    }

    fn dotest(n: i32, exp: f64) -> () {
        assert_float_equals(going(n), exp);
    }

    #[test]
    fn basics_going() {
        dotest(5, 1.275);
        dotest(6, 1.2125);
        dotest(7, 1.173214);
        dotest(8, 1.146651);
    }
}