use std::fs;
use std::collections::HashSet;
use std::hash::Hash;

fn is_all_unique<T>(iter: impl Iterator<Item=T>) -> bool
where T : Hash + Eq
{
    let vec : Vec<T> = iter.into_iter().collect();

    if vec.len() == 0 { return true; }

    let mut unique_values = HashSet::new();

    for item in vec.iter()
    {
        if unique_values.contains(&item) { return false; }

        unique_values.insert(item);
    }

    return true;
}

fn find_unique_n<T>(iter : impl Iterator<Item=T>, count : usize) -> Option<usize>
where T : Hash + Eq
{
    let vec : Vec<T> = iter.into_iter().collect();

    for i in 0..vec.len() - count + 1
    {
        let current_seq = &vec[i..i+count];

        if is_all_unique(current_seq.into_iter())
        {
            return Some(i);
        }
    }

    return None;
}

fn main()
{
    let signal = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    println!("Part 1 : {}", find_unique_n(signal.chars(), 4).unwrap() + 4);
    println!("Part 2 : {}", find_unique_n(signal.chars(), 14).unwrap() + 14);
}
