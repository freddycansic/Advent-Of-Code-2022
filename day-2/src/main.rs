#![allow(non_snake_case)]

use std::fs;

fn gameResult(game : (i32, i32)) -> i32 
{
    let result : i32 = (game.0 - game.1).rem_euclid(3);

    match result
    {
        0 => return 1,
        1 => return 0,
        2 => return 2,
        _ => return 0
    }
}

fn responseFromGame(game : (i32, i32)) -> i32
{
    (game.0 + game.1 - 1).rem_euclid(3)
}

fn scoreFromGame(game : (i32, i32)) -> i32
{
    gameResult(game) * 3
}

fn main()
{
    let stringFile = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    let stringGamesVec : Vec<String> = stringFile.trim().split("\n").map(|game| game.to_string()).collect::<Vec<String>>();

    let games : Vec<(i32, i32)> = stringGamesVec.iter()
        .map(|game| 
            (
                (game.trim().chars().next().unwrap() as i32 - 'A' as i32) % 3, 
                (game.trim().chars().nth(2).unwrap() as i32 - 'X' as i32) % 3
            )
        ).collect::<Vec<_>>();
    
    // games.iter().for_each(|game| println!("{:?}", game));

    let totalScore : i32 = games.iter().map(|game| scoreFromGame(*game) + game.1 + 1).sum();

    println!("Part 1 : {totalScore}");

    let totalScore : i32 = games.iter().map(|game| scoreFromGame((game.0, responseFromGame(*game))) + responseFromGame(*game) + 1).sum();

    println!("Part 2 : {totalScore}");
}
