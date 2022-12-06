use std::fs;
use std::collections::HashSet;

fn is_all_unique(string : &str) -> bool
{
    if string.chars().count() == 0 { return true; }

    let mut unique_values = HashSet::new();
    
    for letter in string.chars()
    {
        if unique_values.contains(&letter) { return false; }

        unique_values.insert(letter);
    }

    return true;
}

fn find_unique_n(string : &str, count : usize) -> Option<usize>
{
    for i in 0..string.len() - count - 1
    {
        let current_seq = &string[i..i+count];

        if is_all_unique(&current_seq)
        {
            return Some(i);
        }
    }

    return None;
}

fn main()
{
    let signal = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    println!("Part 1 : {}", find_unique_n(&signal, 4).unwrap() + 4);
    println!("Part 2 : {}", find_unique_n(&signal, 14).unwrap() + 14);
}
