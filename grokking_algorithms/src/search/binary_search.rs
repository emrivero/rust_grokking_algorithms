pub mod binary_search {
    pub fn execute(list: &Vec<i32>, search_value: i32) -> Option<i32> {
        if list.len() == 1 {
            let head = list.get(0);
            match head {
                Some(&n) => {
                    if n == search_value {
                        Some(search_value)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            let mid_index = list.len() / 2;
            let mid_value = list.get(mid_index);

            match mid_value {
                Some(&value) => {
                    if value == search_value {
                        Some(search_value)
                    } else if value > search_value {
                        let lower_slice = &list[0..mid_index];
                        execute(&lower_slice.to_vec(), search_value)
                    } else {
                        let upper_slice = &list[mid_index..];
                        execute(&upper_slice.to_vec(), search_value)
                    }
                }
                None => None,
            }
        }
    }
}

// [1,2,3,4,5,6,7,8,9], len == 9, mid_index = 9 / 2 = 4

// [1,2,3,4] -- [5,6,7,8,9]

//
