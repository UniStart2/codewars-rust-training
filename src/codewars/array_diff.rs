use std::fmt::Debug;
use std::fs::read;
use std::ops::Index;

// 方式一：
//pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T>
// {
//     // a 或 b为空数组，直接返回a数组
//     // if a.is_empty() || b.is_empty() { return a };
//
//     let mut result = Vec::new();
//     for a_item in a {
//         if !b.contains(&a_item) {
//             result.push(a_item);
//         }
//     }
//     result
// }

// 方式二：
pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T>
{
    let mut a = a;
    a.retain(|x| !b.contains(x));
    a
}


// Add your tests here
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
        assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
        assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
        assert_eq!(array_diff(vec![1,2,3], vec![1,2]), vec![3]);
    }
}
