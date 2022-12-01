#![allow(non_snake_case)]

use std::fs;

fn main() 
{
    let maxCalories = fs::read_to_string("calories.txt").expect("Could not find calories.txt!").trim().split("\n\n")
        .map(|group| group.split("\n")
            .map(|calorie| calorie.trim().parse::<u32>().unwrap())
        .into_iter().collect::<Vec<u32>>())
        
        .map(|group| group.into_iter().sum::<u32>()).max().unwrap();

    println!("{maxCalories}");
}
