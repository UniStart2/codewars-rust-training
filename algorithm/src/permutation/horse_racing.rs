use std::collections::HashMap;
use std::ffi::NulError;
use std::fmt::Error;
use std::io::ErrorKind;
use std::ops::Index;

#[derive(Debug, Default)]
pub struct OwnerHorses<'a> {
    horses: &'a [&'a str],
    horses_time: HashMap<&'a str, f32>,
    horses_permutation:  Vec<Vec<&'a str>>,
}

impl<'a> OwnerHorses<'a> {
    fn new(horses: &'a [&'a str], horses_time: HashMap<&'a str, f32>) -> Self {
        OwnerHorses {
            horses,
            horses_time,
            ..Default::default()
        }
    }
}

/// 列出马匹的所有可能的出战排列
pub fn horses_racing<'a>(horses: Vec<&'a str>, result: Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut all_results = Vec::new();

    if horses.is_empty() { // 如果待排序马匹数量为0，说明排列完成
        all_results.push(result);
        return all_results;
    }

    for item in horses.iter().enumerate() {
        // 从剩下未出战的马匹中选择一匹，加入结果集
        let mut new_result = result.clone(); // 复制上一次的选择结果
        new_result.push(*item.1); // 加入结果集

        // 将已选择的马匹从为出战列表中删除
        let mut new_horses = horses.clone();
        new_horses.remove(item.0);

        // 继续递归调用处理剩下的未出战马匹
        let mut sub_results = horses_racing(new_horses, new_result);
        all_results.append(&mut sub_results);
    }

    all_results
}

/// 根据两边马匹的出战顺序，判断谁赢得当前场次的比赛
pub fn compare_horse(t_owner_horses: &OwnerHorses, q_owner_horses: &OwnerHorses) {
    let (mut t_win_cnt, mut q_win_cnt): (usize, usize) = (0, 0); // 总场次胜场数
    let t_horses_time = &t_owner_horses.horses_time;
    let t_horses_permutation = &t_owner_horses.horses_permutation;
    let q_horses_time = &q_owner_horses.horses_time;
    let q_horses_permutation = &q_owner_horses.horses_permutation;

    let mut t_win_permutation = Vec::<_>::new();
    let mut q_win_permutation = Vec::<_>::new();

    let mut times = 0; // 场次计数器
    for t_item in t_horses_permutation {
        for q_item in q_horses_permutation {
            times = times + 1; // 场次+1
            let (mut t_tmp_win, mut q_tmp_win): (usize, usize) = (0, 0); // 当前场次胜场数

            println!("***********************************************");
            println!("当前第{:}轮次两边马匹出战顺序\n t: {:?} q: {:?}", times, t_item, q_item);

            for i in 0..t_item.len() as usize {
                if t_horses_time.get(t_item[i]) < q_horses_time.get(q_item[i]) {
                    t_tmp_win = t_tmp_win + 1;
                }else {
                    q_tmp_win = q_tmp_win + 1;
                }
            }

            println!("当前第{:}轮次比赛结果：t_tmp_win: {:?}, q_tmp_win:{:?}", times, t_tmp_win, q_tmp_win);
            println!("***********************************************");
            println!();

            if t_tmp_win > q_tmp_win { // t赢
                t_win_permutation.push((t_item, q_item))
            }else if t_tmp_win < q_tmp_win { // q赢
                q_win_permutation.push((t_item, q_item))
            }

            t_win_cnt = t_win_cnt + t_tmp_win;
            q_win_cnt = q_win_cnt + q_tmp_win;
        }
    }

    println!("*******************比赛总结**********************");
    println!("总获胜轮数\n t:{:5?}, q:{:5?}", t_win_permutation.len(), q_win_permutation.len());
    println!("总胜场数\n t:{:5?}, q:{:5?}", t_win_cnt, q_win_cnt);
    println!("t 获胜的比赛出场顺序: {:?}", t_win_permutation);
    println!("q 获胜的比赛出场顺序: {:?}", q_win_permutation);
    println!("************************************************");
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::permutation::horse_racing::{compare_horse, horses_racing, OwnerHorses};

    #[test]
    fn test() {
        let mut t_owner_horses = OwnerHorses::new(
            &["t1", "t2", "t3"],
            HashMap::from([("t1", 1.5f32), ("t2", 2.5f32), ("t3", 3.5f32)]
            ));
        let mut q_owner_horses = OwnerHorses::new(
            &["q1", "q2", "q3"],
            HashMap::from([("q1", 1.0f32), ("q2", 2.0f32), ("q3", 3.0f32)]
            ));

        t_owner_horses.horses_permutation = horses_racing(t_owner_horses.horses.to_vec(), vec![]);
        q_owner_horses.horses_permutation = horses_racing(q_owner_horses.horses.to_vec(), vec![]);

        // println!("{:?}", t_owner_horses);
        // println!("{:?}", q_owner_horses);
        compare_horse(&t_owner_horses, &q_owner_horses);
        // println!("{:?}", t_owner_horses);
        // println!("{:?}", q_owner_horses);
    }
}