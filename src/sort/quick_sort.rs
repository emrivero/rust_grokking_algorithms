pub mod quick_sort {
    use rand::Rng;

    pub fn execute(arr: &Vec<i32>) -> Vec<i32> {
        match arr.len() {
            0 => arr.clone(),
            1 => arr.clone(),
            _ => {
                let rnd_index = rand::thread_rng().gen_range(0, arr.len());
                let pivot = arr.get(rnd_index).unwrap();
                let mut low_partition = vec![];
                let mut high_partition = vec![];

                for element in arr.iter() {
                    if element < &pivot {
                        low_partition.push(*element);
                    }

                    if element > &pivot {
                        high_partition.push(*element);
                    }
                }

                let mut result = vec![];
                result.append(&mut execute(&low_partition));
                result.push(*pivot);
                result.append(&mut execute(&high_partition));

                result
            }
        }
    }
}
