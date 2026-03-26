#![forbid(unsafe_code)]

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    longest_common_prefix_no_allocs(strs)
}

pub fn longest_common_prefix_no_allocs(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    if strs.len() == 1 {
        return strs[0].to_string();
    }
    let first_str = strs[0];
    let mut bend = 0;
    'ext: for (i, ch) in first_str.char_indices() {
        for str in strs[1..].iter() {
            if !str.is_char_boundary(i) {
                break 'ext;
            }
            let nch = str[i..].chars().next();
            if nch.is_none() {
                break 'ext;
            }
            if nch.unwrap() != ch {
                break 'ext;
            }
        }
        bend += ch.len_utf8();
    }
    first_str[..bend].to_string()
}

pub fn longest_common_prefix_allocs(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    if strs.len() == 1 {
        let x = *strs.get(0).unwrap();
        return String::from(x);
    }
    let mut chars = Vec::new();
    for str in &strs {
        chars.push(str.chars());
    }
    let mut len = 0;
    'ext: loop {
        let mut cur_char: char = ' ';
        for (i, chrs) in chars.iter_mut().enumerate() {
            let ch = chrs.next();
            if ch.is_none() {
                break 'ext;
            }
            if i == 0 {
                cur_char = ch.unwrap();
                continue;
            }
            if ch.unwrap() != cur_char {
                break 'ext;
            }
        }
        len += 1;
    }
    strs.get(0).unwrap().chars().take(len).collect()
}
