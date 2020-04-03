

//const nominaly: [u32; 3] = [2, 5, 10];

fn main() {
    let val = 17;
    let combinations: Vec<Vec<u32>> = vec![];

    let temp: Vec<u32> = vec![];
    loop {
        let combination = vec![];
        let n = val;
        loop {
            let res = step()
        }
    }

    println!("Hello, world!");
}

fn step(nominaly: Vec<u32>, var: Vec<u32>, num: u32) -> Option<(Option<Vec<u32>>, u32)> {
    if num == 0 {
        return None;
    }
    let last_nominal = var.last().or(Some(&std::u32::MAX)).unwrap();
    for n in nominaly.iter() {
        if n > last_nominal { return Some((None, *n)); /* znak aby usunąć ten nominał - dla tego nominału nie ma następnego kroku */ } else
        if n % var.last()? == 0 {
            num = num - n;
            var.push(*n);
        }
        
    }
    Some((Some(var), 2))
}

//10 5 2
//5 5 5 2
//5 2 2 2 2 2 2