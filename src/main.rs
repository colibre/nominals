

const NOMINALS: [u32;3] = [10, 5, 2];


fn main() {
    let value = 100;

    if let Some(main_node) = Node::new(std::u32::MAX, value) {
        main_node.print();
    } else {
        println!("Nie da się podzielić tej liczby na nominały");
    }


}


#[derive(Debug)]
struct Node {
    value: u32,
    nodes: Vec<Option<Node>>,
}

impl Node {
    fn new(value: u32, remainder: u32) -> Option<Self> {
        let mut vec: Vec<Option<Node>> = Vec::with_capacity(NOMINALS.len());
        if remainder == 0 {
            Some(Node {
                value,
                nodes: vec![],
            })
        } else {
            let mut iter = NOMINALS.iter().cloned();
            for nom in iter.by_ref() {
                if nom > value {
                    vec.push(None);                                      
                } else if *NOMINALS.last()? == nom &&
                           remainder % nom != 0 {
                    vec.push(None);
                } else if let Some(subtraction) = remainder.checked_sub(nom) {
                    vec.push(Node::new(nom, subtraction));
                } else {
                    vec.push(None);
                }
            }
            if !vec.iter().filter(|option| option.is_some()).next().is_some() {
                return None
            }
            Some(Node {
                value: value,
                nodes: vec,
            })
        }
    }
    fn print(&self) {
        let vec = Vec::new();
        self.print_recursively(vec, 0);
    }
    fn print_recursively(&self, mut path: Vec<u32>, mut path_len: usize) {
        path.resize_with(path_len+1, || 0 );
        if self.value != std::u32::MAX {
            path[path_len] = self.value;
            path_len += 1;
        }
        if self.nodes.is_empty() {
            path.iter().take(path_len).for_each(|i| print!("{} ", i));
            println!("");
        } else {
            for node in self.nodes.iter().filter(|a| a.is_some()).map(|a| a.as_ref().unwrap()) {
                node.print_recursively(path.clone(), path_len);
            }
        }

    }
}