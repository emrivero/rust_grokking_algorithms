struct Queue {
    elements: Vec<Graph>,
}

impl Queue {
    pub fn new(elements: Vec<Graph>) -> Queue {
        Queue { elements }
    }

    pub fn has_next(&self) -> bool {
        self.elements.first().is_some()
    }

    // pub fn enqueue(&mut self, element: Graph) {
    //     self.elements.push(element);
    // }

    pub fn enqueue_vec(&mut self, elements: &mut Vec<Graph>) {
        self.elements.append(elements);
    }

    pub fn dequeue(&mut self) -> Option<Graph> {
        let head_ref = self.elements.first();

        let head = match head_ref {
            Some(value) => Some(value.clone()),
            None => None,
        };

        if head.is_some() {
            self.elements = self.elements[1..].to_vec();
        }

        match &head {
            None => None,
            Some(value) => Some(value.clone()),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    data: String,
    neighbors: Vec<Graph>,
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for Graph {}

impl Clone for Graph {
    fn clone(&self) -> Self {
        Graph {
            data: String::from(&self.data),
            neighbors: self.neighbors.clone(),
        }
    }
}

impl Graph {
    pub fn new(data: String, neighbors: Vec<Graph>) -> Graph {
        Graph { data, neighbors }
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }

    pub fn get_neighbors(&self) -> &Vec<Graph> {
        &self.neighbors
    }

    pub fn bfs(&mut self, nearest: String) -> Option<String> {
        let mut queue = Queue::new(vec![]);
        queue.enqueue_vec(&mut self.neighbors);

        while queue.has_next() {
            let next = queue.dequeue().unwrap();
            let mut neighbors = &mut next.get_neighbors().clone();
            if next.get_data() == &nearest {
                return Some(String::from(nearest));
            } else {
                queue.enqueue_vec(&mut neighbors);
            }
        }
        None
    }
}

#[test]
fn test_get_data() {
    let graph = Graph::new(String::from("Emilio"), vec![]);
    assert_eq!(graph.get_data(), "Emilio");
}

#[test]
fn test_get_neighbors() {
    let g1 = Graph::new(String::from("g1"), vec![]);
    let g2 = Graph::new(String::from("g2"), vec![]);
    let neighbors = vec![g1, g2];
    let graph = Graph::new(String::from("Emilio"), neighbors.clone());
    assert_eq!(graph.get_neighbors(), &neighbors);
}
