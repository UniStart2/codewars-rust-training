use std::cmp::{min, Ordering};

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        return None;
    }

    // 结果集
    let mut res: Vec<i32> = Vec::new();
    let mut n_lng = lng;
    let mut n_wdth = wdth;

    while n_lng > 0 && n_wdth > 0 {
        match n_lng.cmp(&n_wdth) {
            Ordering::Less | Ordering::Equal => {
                res.push(n_lng);
                n_wdth = n_wdth - n_lng;
            },
            Ordering::Greater => {
                res.push(n_wdth);
                n_lng = n_lng - n_wdth;
            },
        }
    }

    Some(res)
}


fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {
    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);
}
