use std::{fs, collections::HashSet};

struct Movement
{
    direction : char,
    count : u32
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Default)]
struct Vec2
{
    x : i32,
    y : i32
}

fn adjacent(p1 : &Vec2, p2 : &Vec2) -> bool
{
    return (p1.x - p2.x).abs() <= 1 && (p1.y - p2.y).abs() <= 1;
}

fn resolve_tail_position(tail : &Vec2, head : &Vec2) -> Vec2
{
    if adjacent(&tail, &head) { return tail.clone(); }

    let mut result = tail.clone();

    if tail.x != head.x 
    {
        if tail.x < head.x 
        {
            result.x += 1;
        } 
        else 
        {
            result.x -= 1;
        }
    }
    
    if tail.y != head.y 
    {
        if tail.y < head.y 
        {
            result.y += 1;
        } 
        else 
        {
            result.y -= 1;
        }
    }

    return result;
}

fn solution(num_knots : usize, movements : &Vec<Movement>) -> usize
{
    let mut knots = vec![Vec2::default(); num_knots];
    let mut last_positions = vec![Vec2::default(); num_knots];
    
    let mut visited_squares = HashSet::<Vec2>::new();

    for movement in movements.iter()
    {
        for _ in 0..movement.count
        {
            visited_squares.insert(knots[knots.len() - 1].clone());

            last_positions[0] = knots[0];

            match movement.direction
            {
                'L' => knots[0].x -= 1,
                'R' => knots[0].x += 1,
                'U' => knots[0].y += 1,
                'D' => knots[0].y -= 1,
                _ => {}
            }

            for i in 1..knots.len()
            {
                last_positions[i] = knots[i];
                knots[i] = resolve_tail_position(&knots[i], &knots[i - 1]); 
            }
        }
    }

    return visited_squares.len();
}

fn main()
{
    let input = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    let movements : Vec<Movement> = input.trim().split("\n").map(|movement| movement.split(" ").collect()).map(|movement : Vec<&str>| Movement{direction : movement[0].chars().next().unwrap(), count : movement[1].parse::<u32>().unwrap()}).collect();

    println!("Part 1 : {}", solution(2, &movements));
    println!("Part 2 : {}", solution(10, &movements));    
}