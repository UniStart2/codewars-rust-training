// 方式一：加了一个 CLone trait 约束才能实现
//pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
//     where
//         T: std::iter::IntoIterator,
//         T::Item: std::cmp::PartialEq + std::fmt::Debug + std::clone::Clone,
// {
//     let mut result = Vec::new();
//     let mut prev: Option<T::Item> = None;
//
//     for x in sequence {
//         if prev.is_none() || x != prev.clone().unwrap() {
//             result.push(x.clone());
//             prev = Some(x.clone());
//         }
//     }
//     result
// }

// 方式二：使用集合的 dedup方法 去除连续的重复元素
pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
    where
        T: std::iter::IntoIterator,
        T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut v: Vec<_> = sequence.into_iter().collect::<Vec<T::Item>>();
    v.dedup();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
    }
}