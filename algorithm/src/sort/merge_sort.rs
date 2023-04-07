pub mod merge_inplace {
    pub fn merge_sort<T: Clone + Eq + Ord>(arr: &mut [T]) {
        let len = arr.len();
        if len < 2 {
            return;
        }
        let mid = len / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }

    fn merge<T: Clone + Eq + Ord>(arr: &mut [T], mid: usize) {
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                arr[k] = left[i].clone();
                i += 1;
            } else {
                arr[k] = right[j].clone();
                j += 1;
            }
            k += 1;
        }
        while i < left.len() {
            arr[k] = left[i].clone();
            i += 1;
            k += 1;
        }
        while j < right.len() {
            arr[k] = right[j].clone();
            j += 1;
            k += 1;
        }
    }
}

pub mod merge_not_inplace {
    use std::fmt::Debug;

    pub fn merge_sort<T>(origin_arr: Vec<T>) -> Vec<T>
        where
            T: Debug + Eq + Ord + Clone,
    {
        if origin_arr.is_empty() {
            panic!("This array is empty!");
        }

        // 如果分解到只剩一个数，返回该数
        if origin_arr.len() == 1 {
            return origin_arr;
        }

        // 将待排序数组分解成两半
        let mid = origin_arr.len() / 2;
        let mut left_arr = origin_arr[0..mid].to_vec();
        let mut right_arr = origin_arr[mid..].to_vec();

        // 嵌套调用，对两半分别进行排序
        left_arr = merge_sort(left_arr);
        right_arr = merge_sort(right_arr);

        // 合并排序后的两半
        let merged = merge(left_arr, right_arr);

        return merged;
    }

    fn merge<T>(left_arr: Vec<T>, right_arr: Vec<T>) -> Vec<T>
        where
            T: Debug + Eq + Ord + Clone,
    {
        if left_arr.is_empty() {
            panic!("left array is empty!")
        }
        if right_arr.is_empty() {
            panic!("right array is empty!")
        }

        let mut li = 0usize;
        let mut ri = 0usize;
        let mut result = Vec::<T>::with_capacity(left_arr.len() + right_arr.len());

        // 轮流从两个数组中取出较小的值，放入合并后的数组中
        while li < left_arr.len() && ri < right_arr.len() {
            if left_arr[li] <= right_arr[ri] {
                result.push(left_arr.get(li).unwrap().clone());
                li += 1;
            } else {
                result.push(right_arr.get(ri).unwrap().clone());
                ri += 1;
            }
        }

        // 如果两个数组有一个先结束了，就将另一个数组的元素直接依次放入进来
        if li < left_arr.len() {
            for i in li..left_arr.len() {
                result.push(left_arr.get(i).unwrap().clone());
            }
        } else {
            for i in ri..right_arr.len() {
                result.push(right_arr.get(i).unwrap().clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::{merge_inplace, merge_not_inplace};

    #[test]
    fn test_merge_inplace() {
        let mut unsorted_vec = [1, 2, 3, 4, 7, 5, 6, 9, 0, 8];
        merge_inplace::merge_sort(&mut unsorted_vec);
        assert_eq!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], unsorted_vec);
    }

    #[test]
    fn test_merge_not_inplace(){
        let unsorted_vec = vec!['a', 'b', 'd', 'c', 'e'];
        let sorted_vec = merge_not_inplace::merge_sort(unsorted_vec);
        assert_eq!(['a', 'b', 'c', 'd', 'e'].to_vec(), sorted_vec);
    }
}