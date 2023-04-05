pub mod linear_search {
    pub fn execute(list: &Vec<i32>, search_value: i32) -> Option<i32> {
        let head = list.get(0);

        match head {
            Some(n) => {
                if n == &search_value {
                    Some(search_value)
                } else {
                    let rest = &list[1..];
                    execute(&rest.to_vec(), search_value)
                }
            }
            _ => None,
        }
    }
}
