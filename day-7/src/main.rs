use std::{fs, collections::HashMap, fmt::Display};

fn print_list<T>(iter : impl Iterator<Item=T>)
where T : Display
{
    for item in iter
    {
        println!("{item}");
    }
}

fn main() 
{
    let input = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    let lines : Vec<&str> = input.split("\n").collect();
    let lines : Vec<&str> = lines.into_iter().filter(|line| line.split(" ").collect::<Vec<&str>>()[1] != "ls").collect();
    let lines : Vec<Vec<&str>> = lines.iter().map(|line| line.split(" ").collect::<Vec<&str>>()).collect();

    let lines : Vec<Vec<String>> = lines.iter().map(|line| line.iter().map(|token| token.to_string()).collect()).collect();

    let mut directory_sizes = HashMap::<String, u32>::new();
    
    let mut current_directory : String = String::new(); 

    for line in lines.iter()
    {
        if line[0] == "cd".to_string()
        {
            current_directory += &("/".to_string() + &line[1].to_string());
        }
        else
        {
            let current_size = directory_sizes.get(&current_directory).unwrap();
            directory_sizes.insert(current_directory, current_size + line[0].parse::<u32>().unwrap());
        }
    }
}
