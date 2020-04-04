


fn main() {
    let val = 17;
    let combinations: Vec<Vec<u32>> = vec![];

    let nominals = [10, 5, 2];


    let temp: Vec<u32> = vec![];
    'main: loop {
        let mut combination = vec![];
        let mut n = val;
        //let mut res = step(10, combination, n);
        let mut current_nominal = nominals.iter().peekable();
        loop {
            match step(**current_nominal.peek().unwrap(), &mut combination, n) {
                Step::Continue((Some(combination), remainder)) => {
                    n = remainder;
                    println!("{}: {:?}", remainder, combination);
                },
                Step::Continue((None, remainder)) => {

                },
                Step::DowngradeNominal(nominal) => {
                    current_nominal.next();
                    println!("lol");
                },
                Step::Impossible => {
                    break 'main;
                }
            }
        }
    }

    println!("Hello, world!");
}

fn step(nom: u32, sumof: &mut Vec<u32>, mut num: u32) -> Step {
    if num == 0 {
        return Step::Impossible;
    }

    if num.checked_sub(nom).is_some() {
        num = num - nom;
        sumof.push(nom);
    } else {
        return Step::DowngradeNominal(nom);
    }
    
    Step::Continue((Some(sumof.to_vec()), num))
}

//10 5 2
//5 5 5 2
//5 2 2 2 2 2 2
#[derive(Debug)]
enum Step {
    Continue((Option<Vec<u32>>, u32)),
    DowngradeNominal(u32),
    //CannotProceed,
    //BranchLower((Option<Vec<u32>>, u32)),
    Impossible,
}

struct Node<T> {
    value: T,
    nodes: Vec<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            nodes: vec![],
        }
    }
    fn add_node(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }
}