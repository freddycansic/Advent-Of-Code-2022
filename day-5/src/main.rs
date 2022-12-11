use std::fs;

fn transpose<T>(matrix : &Vec<Vec<T>>) -> Vec<Vec<T>>
where T : Clone + Default
{
    assert!(!matrix.is_empty());

    let mut transposed : Vec<Vec<T>> = vec![vec![T::default(); matrix.len()]; matrix[0].len()];    

    for (i, row) in matrix.iter().enumerate()
    {
        for (j, el) in row.iter().enumerate()
        {
            transposed[j][i] = el.clone();
        }
    }

    return transposed;
}

struct Movement
{
    count : usize,
    from : usize,
    to : usize,
}

fn main() 
{
    // parse input
    let input = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    let (stacks, moves) = input.trim_end().split_once("\n\n").unwrap();

    let stacks = stacks.replace("[", " ");

    let stacks = stacks.replace("]", " ");

    // removes space at the front of each row
    let stacks : Vec<&str> = stacks.split("\n").map(|row| &row[1..row.len()]).collect();

    let stacks : Vec<Vec<char>> = stacks.iter().map(|row| row.chars().step_by(4).collect()).collect();

    let mut stacks = transpose(&stacks);

    // remove spaces
    stacks.iter_mut().for_each(|row| row.retain(|el| el != &' '));

    stacks.iter_mut().for_each(|row| row.reverse());

    // parse moves
    let moves : Vec<&str> = moves.split("\n").collect();
    let moves : Vec<Vec<&str>> = moves.iter().map(|movement| movement.split(" ").collect()).collect();

    let moves : Vec<Movement> = moves.iter().map(|movement| 
        Movement
        {
            count: movement[1].parse::<usize>().unwrap(),
            from: movement[3].parse::<usize>().unwrap() - 1,
            to: movement[5].parse::<usize>().unwrap() - 1
        }).collect();

    let mut part_1_stacks = stacks.clone();

    for movement in moves.iter()
    {
        for _ in 0..movement.count
        {
            let elf_crate = part_1_stacks[movement.from].pop().unwrap().clone();
            
            part_1_stacks [movement.to].push(elf_crate);   
        }
    }

    print!("Part 1 : ");
    part_1_stacks.iter().for_each(|row| print!("{}", row[row.len()-1]));
    println!();

    for movement in moves.iter()
    {
        let current_stack_len = stacks[movement.from].len();
        
        let elf_crates = stacks[movement.from][current_stack_len - movement.count..current_stack_len].to_vec();

        stacks[movement.from].truncate(current_stack_len - movement.count);

        stacks[movement.to].append(&mut elf_crates.clone());
    }

    print!("Part 2 : ");
    stacks.iter().for_each(|row| print!("{}", row[row.len()-1]));
    println!();
}
