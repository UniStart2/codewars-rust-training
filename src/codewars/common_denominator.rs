use num::integer::{gcd, lcm};

fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // 0. 对传入的每个分数先进行化简
    let mut simplify = l.iter()
        .map(|(numerator, denominator)| {
            // 求出分子和分母的最大公约数
            let divisor = gcd(*numerator, *denominator);
            (*numerator / divisor, *denominator / divisor)
        }).collect::<Vec<(i64, i64)>>();

    // 1、先将所有分数的分母取出来，并求出它们的最小公倍数
    let mut denominators = Vec::new();
    for fraction in &simplify {
        denominators.push(fraction.1);
    }
    let lcm = denominators.iter().fold(1, |acc, x| lcm(acc, *x));

    // 2、将所有分数的分子扩大
    simplify.iter()
        .map(|(num, den)| (*num * lcm / *den, lcm)).collect()
}

fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(convert_fracts(l), exp)
}

#[test]
fn basics_convert_fracts() {

    testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);

}