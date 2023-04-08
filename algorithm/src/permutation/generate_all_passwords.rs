/// 假设有一个 4 位字母密码，每位密码是 a～e 之间的小写字母。你能否编写一段代码，来暴力破解该密码？
/// （提示：根据可重复排列的规律，生成所有可能的 4 位密码。）

// const ALPHABET: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
const ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    '0', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn is_valid_password(passwd: &str) -> bool {
    // 检查密码是否符合要求
    passwd.len() == 4 && passwd.chars().all(|c| ALPHABET.contains(&c))
}

pub mod recursive {
    use std::collections::HashSet;
    use crate::permutation::generate_all_passwords::ALPHABET;
    use super::is_valid_password;

    pub fn generate_all_passwords(passwd: String, passwd_len: usize) -> Vec<String> {
        let mut passwd_res = Vec::new();

        if is_valid_password(&passwd) { // 判断当前生成的密码是否符合要求
            passwd_res.push(passwd);
            return passwd_res;
        }

        for pass_item in ALPHABET {
            let mut new_passwd_res = passwd.clone(); // 复制上一次的结果
            new_passwd_res.push(pass_item);   // 放入当前选择

            let mut curr_passwd = generate_all_passwords(new_passwd_res, passwd_len - 1 );  // 递归调用继续生成剩下位数的密码
            passwd_res.append(&mut curr_passwd); // 将生成好的密码放入最终结果集
        }

        passwd_res
    }
}

pub mod iterate {
    use super::ALPHABET;

    pub fn generate_all_passwords() -> Vec<String> {
        let mut passwd_res = Vec::new();

        for a in ALPHABET { // 第一位
            for b in ALPHABET { // 第二位
                for c in ALPHABET { // 第三位
                    for d in ALPHABET { // 第四位
                        let passwd = format!("{}{}{}{}", a, b, c, d);
                        passwd_res.push(passwd);
                    }
                }
            }
        }

        passwd_res
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_recursive() {
        let passwd_res = recursive::generate_all_passwords("".to_string(), 4);
        assert_eq!(456_976, passwd_res.len());
    }

    #[test]
    fn test_iterate() {
        let passwd_res = iterate::generate_all_passwords();
        assert_eq!(456_976, passwd_res.len());
    }
}
