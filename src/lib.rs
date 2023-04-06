pub mod graph;
pub mod search;
pub mod sort;

#[cfg(test)]
mod tests {
    use crate::graph::graph::Graph;

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
    fn test_graph_bfs() {
        let g_a = Graph::new(String::from("A"), vec![]);
        let g_b = Graph::new(String::from("B"), vec![g_a.clone()]);
        let g_c = Graph::new(String::from("C"), vec![g_b.clone()]);
        let g_d = Graph::new(String::from("D"), vec![g_b.clone(), g_c.clone()]);
        let g_f = Graph::new(String::from("F"), vec![g_a, g_d.clone()]);
        let mut g_g = Graph::new(String::from("G"), vec![g_c.clone(), g_f]);

        assert_eq!(g_g.bfs(String::from("B")), Some(String::from("B")))
    }
    #[test]
    fn test_graph_bfs_none() {
        let g_a = Graph::new(String::from("A"), vec![]);
        let g_b = Graph::new(String::from("B"), vec![g_a.clone()]);
        let g_c = Graph::new(String::from("C"), vec![g_b.clone()]);
        let g_d = Graph::new(String::from("D"), vec![g_b.clone(), g_c.clone()]);
        let g_f = Graph::new(String::from("F"), vec![g_a, g_d.clone()]);
        let mut g_g = Graph::new(String::from("G"), vec![g_c.clone(), g_f]);

        assert_eq!(g_g.bfs(String::from("R")), None)
    }
}
