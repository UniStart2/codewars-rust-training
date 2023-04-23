use num::BigInt;
// 方式一（自己提交的）
fn increment_string(input: &str) -> String {
    // 如果是空串或者字符串中没有任何数字的情况
    if input.is_empty() || input.chars().all(|c| c.is_alphabetic()) {
        return format!("{}1", input);
    }

    // 字符串都是数字的情况
    if input.chars().all(|ch| ch.is_numeric()) {
        return match input.parse::<BigInt>() {
            Ok(value) => {
                format!("{:01$}", value+1, input.len())
            }
            Err(err) => {
                format!("[Error]: {}", err)
            }
        }
    }

    // 字符串中混杂数字和字母的情况
    let mut alphabet_idx = 0usize;
    let mut numeric_idx = 0usize;

    match input.rfind(char::is_alphabetic) { // 从右找到字母第一次出现的位置
        Some(idx) => { alphabet_idx = idx; },
        None => {},
    };
    match input.rfind(char::is_numeric) { // 从右找到数字第一次出现的位置
        Some(idx) => { numeric_idx = idx; },
        None => {},
    };

    if numeric_idx < alphabet_idx {
        return format!("{}1", input);
    }

    let prefix = &input[0..alphabet_idx+1];
    let suffix = &input[alphabet_idx+1..];
    return match suffix.parse::<BigInt>() {
        Ok(value) => {
            format!("{}{:02$}", prefix, value+1, numeric_idx - alphabet_idx)
        },
        Err(err) => {
            format!("[Error]: {}", err)
        }
    }
}

// 方式二（其他人的解决思路，利用递归）
fn increment_string2(input: &str) -> String {
    if let Some(last) = input.chars().last() {
        match last.to_digit(10) {
            // 最后一位是数字9的情况，利用递归处理进位
            Some(9) => { format!("{}0", &increment_string2(&input[..input.len()-1])) },
            // 最后一位是数字，但是不是数字9的情况
            Some(num) => { format!("{}{}", &input[..input.len()-1], num+1) },
            // 最后一位不是数字的情况
            None => { format!("{input}1") },
        }
    } else { // 空串的情况
        format!("1")
    }
}

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected,
                "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
        dotest("1", "2");
        dotest("000233", "000234");
        dotest("1ss2s1", "1ss2s2");
        dotest("1s001", "1s002");
        dotest("1s001s", "1s001s1");
    }
}
