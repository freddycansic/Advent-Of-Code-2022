use std::fs;

struct TreeVisibility
{
    distance : u32,
    visible : bool
}

#[derive(Clone, Copy, Debug)]
struct Tree
{
    height : u32,
    visible : bool,
    left_score : u32,
    right_score : u32,
    up_score : u32,
    down_score : u32,
    scenic_score : u32
}

impl Default for Tree
{
    fn default() -> Self {
        Tree 
        { 
            height: 0, 
            visible: false,
            left_score : 0,
            right_score : 0,
            up_score : 0,
            down_score : 0,
            scenic_score : 0
        }
    }
}

fn get_first_tree_visibility(trees: &Vec<Tree>) -> TreeVisibility
{
    let mut distance = 1;

    for i in 1..trees.len()
    {
        if !(trees[0].height > trees[i].height) 
        {
            return TreeVisibility{distance : distance, visible : false};
        }

        distance += 1;
    }

    return TreeVisibility{distance : distance - 1, visible : true};
}

fn scenic_score(tree : &Tree) -> u32
{
    return tree.left_score * tree.right_score * tree.up_score * tree.down_score
}

fn main() 
{
    let input = fs::read_to_string("input.txt").expect("Could not find input.txt!");

    let mut trees : Vec<Vec<Tree>> = input.split("\n").map(|row| row.chars().map(|tree| Tree{height : tree.to_digit(10).unwrap() as u32, ..Default::default()} ).collect()).collect();
    
    let trees_2 = trees.clone(); // how do i get around this?

    let bottom_row_idx = &trees.len() - 1;
    let right_row_idx = &trees[0].len() - 1;

    for (i, tree_row) in trees.iter_mut().enumerate()
    {
        for (j, tree) in tree_row.iter_mut().enumerate()
        {
            if i == 0 || j == 0 || i == bottom_row_idx || j == right_row_idx
            {
                tree.visible = true;
                tree.scenic_score = 0;
                continue;
            }

            let current_row = &trees_2[i];
            
            // right
            let row_to_right = &current_row[j..right_row_idx + 1].to_vec();

            let right_visibility = get_first_tree_visibility(&row_to_right);
            tree.right_score = right_visibility.distance;
            
            if right_visibility.visible
            {
                tree.visible = true
            }
           
            // left
            let row_to_left = &mut current_row[0..j+1].to_vec();
            
            row_to_left.reverse();

            let left_visibility = get_first_tree_visibility(&row_to_left);
            tree.left_score = left_visibility.distance;
            
            if left_visibility.visible
            {
                tree.visible = true
            }

            // above
            let mut column_above = Vec::<Tree>::new();

            for column_idx in 0..i+1
            {
                column_above.push(trees_2[column_idx][j]);
            }

            column_above.reverse();

            let up_visibility = get_first_tree_visibility(&column_above);
            tree.up_score = up_visibility.distance;
            
            if up_visibility.visible
            {
                tree.visible = true
            }

            // down
            let mut column_below = Vec::<Tree>::new();

            for column_idx in i..bottom_row_idx + 1
            {
                column_below.push(trees_2[column_idx][j]);
            }

            let down_visibility = get_first_tree_visibility(&column_below);
            tree.down_score = down_visibility.distance;
            
            if down_visibility.visible
            {
                tree.visible = true
            }

            tree.scenic_score = scenic_score(tree);
        }
    }
    
    let part_1 = trees.iter().fold(0u32, |acc, tree_row| acc + tree_row.iter().fold(0u32, |mut acc, tree| {acc += tree.visible as u32; acc}));
    
    println!("Part 1 : {part_1}");

    let part_2 = trees.iter().map(|tree_row| tree_row.iter().map(|tree| tree.scenic_score).max().unwrap()).max().unwrap();

    println!("Part 2 : {part_2}");
}