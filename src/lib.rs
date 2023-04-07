pub mod graph;
pub mod search;
pub mod sort;

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::graph::*;
    use super::search::binary_search;
    use super::sort::*;

    #[test]
    fn test_found_value() {
        let test_arr = (0..5 * 1000).collect();
        println!("{:?}", test_arr);
        assert_eq!(
            binary_search::binary_search::execute(&test_arr, 75),
            Some(75)
        )
    }

    #[test]
    fn test_empty_list() {
        let test_arr = vec![];
        println!("{:?}", test_arr);
        assert_eq!(binary_search::binary_search::execute(&test_arr, 75), None)
    }

    #[test]
    fn test_not_found_value() {
        let test_arr = (0..100).collect();
        println!("{:?}", test_arr);
        assert_eq!(binary_search::binary_search::execute(&test_arr, 200), None)
    }

    #[test]
    fn test_selection_sort_empty_arr() {
        let arr = vec![];

        assert_eq!(selection_sort::selection_sort::execute(&arr.into()), vec![])
    }

    #[test]
    fn test_selection_sort() {
        let arr = vec![4, 1, 0, 89, 3];

        assert_eq!(
            selection_sort::selection_sort::execute(&arr.into()),
            vec![0, 1, 3, 4, 89]
        )
    }

    #[test]
    fn test_quick_sort_empty_arr() {
        let arr = vec![];

        assert_eq!(quick_sort::quick_sort::execute(&arr.into()), vec![])
    }

    #[test]
    fn test_quick_sort() {
        let arr = vec![4, 1, 0, 89, 3];

        assert_eq!(
            quick_sort::quick_sort::execute(&arr.into()),
            vec![0, 1, 3, 4, 89]
        )
    }

    #[test]
    fn test_bfs() {
        let mut g: HashMap<String, Vec<String>> = HashMap::new();

        g.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        g.insert("B".to_string(), vec!["F".to_string(), "D".to_string()]);
        g.insert("C".to_string(), vec!["B".to_string()]);
        g.insert("D".to_string(), vec![]);
        g.insert("F".to_string(), vec!["C".to_string()]);

        assert_eq!(
            bfs::bfs(&g, &"A".to_string(), &"D".to_string()),
            vec!["A".to_string(), "B".to_string(), "D".to_string()]
        );
    }

    #[test]
    fn test_cycle_bfs() {
        let mut g: HashMap<String, Vec<String>> = HashMap::new();

        g.insert("A".to_string(), vec!["B".to_string()]);
        g.insert("B".to_string(), vec!["C".to_string()]);
        g.insert("C".to_string(), vec!["A".to_string()]);

        assert_eq!(
            bfs::bfs(&g, &"A".to_string(), &"D".to_string()),
            Vec::<String>::new()
        );
    }
}
