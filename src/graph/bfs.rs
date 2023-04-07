use std::collections::HashMap;

fn aux_bfs(
    graph: &HashMap<String, Vec<String>>,
    node: &String,
    finish_node: &String,
    visited_param: Option<&Vec<&String>>, /* = "!" */
) -> Vec<String> {
    let start_neighbors = graph.get(node);
    let new_vec = vec![];
    let mut result: Vec<String> = vec![];

    let mut visited: Vec<&String> = match visited_param {
        Some(value) => {
            let mut new_visited = vec![node];
            new_visited.extend(value);
            new_visited
        }
        None => new_vec,
    };

    if start_neighbors.is_none() {
        return vec![];
    }

    if start_neighbors.unwrap().is_empty() {
        return vec![];
    }

    if start_neighbors.unwrap().contains(finish_node) {
        return vec![node.to_string(), finish_node.to_string()];
    }

    for neighbor in start_neighbors.unwrap() {
        if !visited.contains(&neighbor) {
            let sub_bfs = aux_bfs(&graph, neighbor, finish_node, Some(&mut visited));
            if !sub_bfs.is_empty() {
                result.push(node.to_string());
                result.extend(sub_bfs);
                break;
            }
        }
    }

    result
}

pub fn bfs(
    graph: &HashMap<String, Vec<String>>,
    node: &String,
    finish_node: &String,
) -> Vec<String> {
    self::aux_bfs(graph, node, finish_node, None)
}
