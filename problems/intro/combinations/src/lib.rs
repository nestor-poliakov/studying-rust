#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k == 0 {
        return Vec::from([Vec::new()]);
    }
    if arr.is_empty() {
        return Vec::new();
    }
    if k == 1 {
        return arr.iter().map(|x| Vec::from([*x])).collect();
    }

    let mut first_part = combinations(&arr[1..], k - 1);
    let second_part = combinations(&arr[1..], k);

    let mut result = Vec::with_capacity(first_part.len() + second_part.len());
    first_part.iter_mut().for_each(|x| x.insert(0, arr[0]));

    result.extend_from_slice(&first_part);
    result.extend_from_slice(&second_part);

    result
}
