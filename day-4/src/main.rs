use std::fs;
use std::collections::HashSet;

fn main() 
{
    let input = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    let task_pairs : Vec<&str> = input.trim().split("\n").collect();

    let task_pairs_split : Vec<(&str, &str)> = task_pairs.iter().map(|pair| pair.split_once(",").unwrap()).collect();

    let task_pairs_bounds : Vec<((&str, &str), (&str, &str))> = task_pairs_split.iter().map(|(task1, task2)| (
        task1.split_once("-").unwrap(), 
        task2.split_once("-").unwrap()
    )).collect();

    let task_pairs_bounds : Vec<((u8, u8), (u8, u8))> = task_pairs_bounds.iter().map(|(bound1, bound2)| (
        (bound1.0.parse::<u8>().unwrap(), bound1.1.parse::<u8>().unwrap()),
        (bound2.0.parse::<u8>().unwrap(), bound2.1.parse::<u8>().unwrap()),
    )).collect();

    let task_pairs_sets : Vec<(HashSet<u8>, HashSet<u8>)> = task_pairs_bounds.iter().map(|(bound1, bound2)| (
        HashSet::from_iter(bound1.0..bound1.1+1),
        HashSet::from_iter(bound2.0..bound2.1+1),
    )).collect();

    let total_supersets : u16 = task_pairs_sets.iter().fold(0u16, |mut total, (task1, task2)| {total += (task1.is_superset(&task2) || task2.is_superset(&task1)) as u16; total});

    println!("Part 1 : {total_supersets}");

    let total_intersections : u16 = task_pairs_sets.iter().fold(0u16, |mut total, (task1, task2)| {total += (!task1.intersection(&task2).next().is_none()) as u16; total});

    println!("Part 2 : {total_intersections}");
}
