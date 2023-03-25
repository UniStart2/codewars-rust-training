// 方法一：
// fn order(sentence: &str) -> String {
//     if sentence.is_empty() {
//         return sentence.to_string();
//     }
//
//     let mut split_v: Vec<&str> = sentence.split(' ').collect();
//     let mut sort_v: Vec<&str> = vec![""; 9];
//
//     for item in split_v.into_iter() {
//         for ch in item.chars() {
//             match ch {
//                 '1' => { sort_v[0] = item },
//                 '2' => { sort_v[1] = item },
//                 '3' => { sort_v[2] = item },
//                 '4' => { sort_v[3] = item },
//                 '5' => { sort_v[4] = item },
//                 '6' => { sort_v[5] = item },
//                 '7' => { sort_v[6] = item },
//                 '8' => { sort_v[7] = item },
//                 '9' => { sort_v[8] = item },
//                 _ => {},
//             }
//         }
//     }
//
//     sort_v.join(" ").trim().to_string()
// }

// 方法二：
fn order(sentence: &str) -> String {
    let mut sw = sentence.split_whitespace().map(String::from).collect::<Vec<String>>();
    sw.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    sw.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        // order("is2 Thi1s T4est 3a");
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
