/// 一个整数可以被分解为多个整数的乘积，例如，6 可以分解为 2x3。
/// 请使用递归编程的方法，为给定的整数 n，找到所有可能的分解（1 在解中最多只能出现 1 次）。
/// 例如，输入 8，输出是可以是 1x8, 8x1, 2x4, 4x2, 1x2x2x2, 1x2x4, ……
fn factorization(n: i32, mut result: Vec<i32>) {
    if n == 1 { // n=1时无法继续分解，结束递归
        if !result.contains(&1) { // 如果结果集中不包含1，则加入1再输出
            result.push(1);
        }
        println!("{:?}", result);
        return;
    }else {
        for i in 1..=n {
            // 两种情况可以将当前数放入结果集：
            // 1. 当前数是1，并且结果集中没有出现过1
            // 2. 当前数不是1，并且n能够被其整除
            if (i == 1 && !result.contains(&1)) || (i != 1 && n % i == 0) {
                let mut new_result = result.clone(); // 复制上一次结果集的数据
                new_result.push(i); // 保存当前选择
                factorization(n / i, new_result); // 继续分解
            }
        }
    }
}

#[cfg(test)]
mod test{
    use super::factorization;

    #[test]
    fn test_factorization(){
        factorization(8, vec![]);
    }
}