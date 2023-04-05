pub mod selection_sort {
    pub fn execute(arr: &Vec<i32>) -> Vec<i32> {
        let mut original_arr = arr.clone();
        let mut sorted_arr: Vec<i32> = vec![];

        let selected = original_arr.get(0);

        match selected {
            Some(value) => {
                let mut selected_value = value;
                for element in arr.iter() {
                    if element < selected_value {
                        selected_value = element;
                    }
                }

                sorted_arr.push(*selected_value);
                original_arr = remove_element(&original_arr, *selected_value);
                sorted_arr.append(&mut execute(&original_arr));
                sorted_arr
            }
            None => sorted_arr,
        }
    }

    fn remove_element(arr: &Vec<i32>, element: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for e in arr.iter() {
            if e != &element {
                result.push(*e);
            }
        }

        result
    }
}
