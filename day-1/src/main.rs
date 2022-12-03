#![allow(non_snake_case)]

use std::fs;

fn main() 
{
    // get file as string
    let stringFile = fs::read_to_string("calories.txt").expect("Could not find calories.txt!");

    // split by double newlines
    let groupedStrCalories = stringFile.trim().split("\n\n");
    
    // split by newlines and convert to ints
    let groupedIntCalories = groupedStrCalories
        .map(|group| 
            group.split("\n")
                .map(|calorie| 
                    calorie.trim().parse::<u32>().unwrap())
        .into_iter().collect::<Vec<u32>>());

    // sum nested vectors and sort
    let mut summedCalories = groupedIntCalories
        .map(|group| 
            group.into_iter().sum::<u32>())
        .collect::<Vec<u32>>(); 

    summedCalories.sort();

    // let i_SortedSummedCalories = summedCalories.into_iter();

    println!("Part 1 : {}", summedCalories.last().unwrap());

    let lastThreeGroups = &summedCalories[summedCalories.len() - 3..summedCalories.len()];

    println!("Part 2 : {}", lastThreeGroups.into_iter().sum::<u32>());
}
