use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn priority(item : char) -> u8
{
    if item.is_lowercase() 
    {
        return item as u8 - 'a' as u8 + 1;
    } else 
    {
        return item as u8 - 'A' as u8 + 27;
    }
}

fn main() 
{
    let input = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    let rucksacks_one_compartment : Vec<&str> = input.trim().split("\n").collect();

    let rucksacks_two_compartments : Vec<(HashSet<char>, HashSet<char>)> = rucksacks_one_compartment.iter().map(|rucksack| 
        (
            HashSet::from_iter((&rucksack[0..rucksack.len()/2]).chars()), 
            HashSet::from_iter((&rucksack[rucksack.len()/2..rucksack.len()]).chars())
        )).collect();
    
    let rucksacks_compartments_intersection : Vec<char> = rucksacks_two_compartments.iter().map(|rucksack|
        *(rucksack.0.intersection(&rucksack.1).next().unwrap())
        ).collect();

    let priority_intersection_sum : u32 = rucksacks_compartments_intersection.iter().map(|intersection|
        priority(*intersection) as u32
        ).sum();

    println!("Part 1 : {priority_intersection_sum}");

    let mut grouped_rucksacks : Vec<[HashSet<char>; 3]> = Vec::new();

    for i in (0..rucksacks_one_compartment.len() - 2).step_by(3)
    {
        grouped_rucksacks.push
        (
            [   
                HashSet::from_iter(rucksacks_one_compartment[i].chars()),
                HashSet::from_iter(rucksacks_one_compartment[i+1].chars()),
                HashSet::from_iter(rucksacks_one_compartment[i+2].chars())
            ]
        );
    }

    let mut total_priority : u32 = 0;

    for grouped_rucksack in grouped_rucksacks
    {
        let [rucksack1, rucksack2, rucksack3] = grouped_rucksack;
        
        let first_intersection = rucksack1.intersection(&rucksack2).cloned().collect::<HashSet<char>>();
        let second_intersection = first_intersection.intersection(&rucksack3).cloned().collect::<HashSet<char>>();

        total_priority += priority(*second_intersection.iter().next().unwrap()) as u32;
    }

    println!("Part 2 : {total_priority}");
}