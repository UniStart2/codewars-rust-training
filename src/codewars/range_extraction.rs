mod solution {
    use std::cmp::max;
    use std::fmt::format;

    pub fn range_extraction(a: &[i32]) -> String {
        let mut res = String::new();
        let mut start_idx = 0;
        let mut tail_idx = 0;
        let max_idx = a.len() - 1;

        if start_idx == max_idx {
            return format!("{}", a[start_idx]);
        }
        if start_idx + 1 == max_idx {
            return format!("{},{}", a[start_idx], a[start_idx + 1]);
        }

        while start_idx <= max_idx {
            if tail_idx != max_idx && a[tail_idx + 1] - a[tail_idx] == 1 { // tail可以后移的情况
                tail_idx += 1;
            }else { // tail无法后移的情况
                // 1. tail根本没有移动：和start指向同一个元素位置
                if start_idx == tail_idx {
                    res.push_str(&*format!("{},", a[start_idx]));
                    start_idx += 1;
                    tail_idx += 1;
                }

                // 2. tail移动了，但移动的元素个数为两个
                if tail_idx - start_idx == 1  {
                    res.push_str(&*format!("{},{},", a[start_idx], a[tail_idx]));
                    start_idx = tail_idx + 1;
                    tail_idx = start_idx;
                }

                // 3. tail移动了，且移动的元素个数大于等于3个
                if tail_idx - start_idx >= 2 {
                    res.push_str(&*format!("{}-{},", a[start_idx], a[tail_idx]));
                    start_idx = tail_idx + 1;
                    tail_idx = start_idx;
                }
            }
        }

        // 去除末尾多出来的','
        res.remove(res.len()-1);

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]), "-6,-3-1,3-5,7-11,14,15,17-20");
        assert_eq!(solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]), "-3--1,2,10,15,16,18-20");
        // println!("{}", solution::range_extraction(&[1]));
        // println!("{}", solution::range_extraction(&[1, 2]));
        // println!("{}", solution::range_extraction(&[-1, 0, 1, 2, 5, 6, 8, 9, 10, 13]))
    }
}
