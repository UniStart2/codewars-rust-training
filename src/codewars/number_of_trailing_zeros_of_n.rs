// 找出这个数里面有多少个5及5的倍数
///  5!  5/5 = 1
/// 10! 10/5 = 2
/// 15! 15/5 = 3
/// 20! 20/5 = 4
/// 25! 25/5 = 5 + 25/25 = 6
/// 30! 30/5 = 6 + 30/25 = 7
/// ...
fn zeros(n: u64) -> u64 {
    let mut  trailing_zeros: u64 = 0;
    let mut base: u64 = 5;

    while base < n {
        trailing_zeros += n / base;
        base *= 5;
    }

    trailing_zeros
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }
}