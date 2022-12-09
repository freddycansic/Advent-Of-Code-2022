use std::{fs, collections::HashMap};

fn main()
{
    let input = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    let lines : Vec<&str> = input.split("\n").collect();
    let lines : Vec<&str> = lines.into_iter().filter(|line| line.split(" ").collect::<Vec<&str>>()[1] != "ls").collect();

    let mut directories = HashMap::<Vec<&str>, (Vec<Vec<&str>>, u32)>::new();
    
    let mut current_directory_path : Vec<&str> = Vec::new();

    for line in lines.iter()
    {
        let tokens : Vec<&str> = line.split(" ").collect();
        let command : &str = tokens[0];
        
        match command
        {
            "$" => // cd
            {
                if tokens[2] == ".."
                {
                    current_directory_path.pop();
                } 
                else 
                {
                    current_directory_path.push(tokens[2]);
                }
            }

            "dir" => 
            {
                let mut current_directory : (Vec<Vec<&str>>, u32) = (Vec::new(), 0);

                if directories.contains_key(&current_directory_path)
                {
                    current_directory = directories.get(&current_directory_path).unwrap().clone();
                }

                let mut subdir_path = current_directory_path.clone();
                subdir_path.push(tokens[1]);

                current_directory.0.push(subdir_path.clone());

                directories.insert(current_directory_path.clone(), current_directory);
            }

            _ => // file
            {
                let mut current_directory : (Vec<Vec<&str>>, u32) = (Vec::new(), 0);

                if directories.contains_key(&current_directory_path)
                {
                    current_directory = directories.get(&current_directory_path).unwrap().clone();
                }

                directories.insert(current_directory_path.clone(), (current_directory.0,  current_directory.1 + tokens[0].parse::<u32>().unwrap()));
            }
        }
    }

    let directories_temp = directories.clone();
    let mut directories_sorted : Vec<(&Vec<&str>, &(Vec<Vec<&str>>, u32))>= directories_temp.iter().map(|(key, val)| (key, val)).collect();

    directories_sorted.sort_by(|dir1, dir2| dir1.0.len().cmp(&dir2.0.len()));
    directories_sorted.reverse();

    for directory in directories_sorted.iter()
    {
        if directory.1.0.len() <= 0 { continue; } // no subdirs
     
        let mut subdirectories_size = 0;

        for subdirectory in directory.1.0.iter()
        {
            subdirectories_size += directories.get(&subdirectory.clone()).unwrap().1;
        }

        directories.insert(directory.0.to_vec(), (directory.1.0.clone(), directory.1.1 + subdirectories_size));
    }

    let part_1 : u32 = directories.iter().filter(|(_key, value)| value.1 <= 100_000 as u32).map(|(_key, value)| value.1).sum::<u32>();

    println!("Part 1 : {part_1}");

    let total_size_used = directories.get(&["/"].to_vec()).unwrap().1;

    let unused_space = 70_000_000 - total_size_used;

    let space_needed = 30_000_000 - unused_space;
    
    let p2 = directories.iter().filter(|(_key, value)| value.1 >= space_needed).map(|(_key, value)| value.1).min().unwrap();

    println!("Part 2 : {p2}");
}