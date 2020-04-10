use std::collections::HashMap;

fn main() {
    println!("{}", are_squares(vec!(1, 2, 3), vec!(1, 4, 9)));
    println!("{}", are_squares(vec!(1, 2, 3, 5), vec!(1, 4, 9)));
    println!("{}", are_squares(vec!(1, 2, 3, 5), vec!(25, 1, 4, 9)));
}

fn are_squares(a: Vec<i32>, b: Vec<i32>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut freq_a: HashMap<i32, i32> = HashMap::new();
    let mut freq_b: HashMap<i32, i32> = HashMap::new();

    for n in a.iter() {
        *freq_a.entry(*n).or_insert(0) += 1;
    }

    for n in b.iter() {
        *freq_b.entry(*n).or_insert(0) += 1;
    }

    println!("freq_a = {:?}", freq_a);
    println!("freq_b = {:?}", freq_b);

    for (k, v) in freq_a.iter() {
        println!("f = {}, {}", k, v);

        let squared_key = k.pow(2);

        println!("squared_key = {}", squared_key);
        println!("key {:?}", freq_b.entry(squared_key).key());
        println!("value {:?}", freq_b.entry(squared_key).or_default());

        if !freq_b.contains_key(&squared_key) && freq_b.entry(squared_key).or_default() != v {
            return false;
        }
    }

    true
}
