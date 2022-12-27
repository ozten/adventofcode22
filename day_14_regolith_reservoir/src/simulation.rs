use crate::{grid::Grid, parser::Coordinate};

enum State {
    Spawn,
    Move
}

pub fn simulate(grid: &mut Grid) {
    let mut state = State::Spawn;
    let mut sand = Coordinate { x: 500, y: 0 };
    let mut i = 0;
    let mut sand_count = 0;
    loop {
        match &state {
            State::Spawn => {
                sand_count += 1;
                sand = Coordinate { x: 500, y: 0 };
                state = State::Move;
                grid.record_sand(sand.x as usize, sand.y as usize);
            },
            Move => {
                let outside = grid.is_ouside_grid(sand.x - 1, sand.y + 1);
                let can_down_left = grid.can_move_down_left(&sand);

                println!("{}, {} is outside? {} can down left? {}", sand.x - 1, sand.y + 1, outside, can_down_left);
                if grid.is_ouside_grid(sand.x, sand.y + 1) {
                    println!("Outsideof grid to move down {:?}", sand);
                    break
                } else 
                if grid.can_move_down(&sand) {
                    println!("Moving down");
                    grid.clear_sand(&sand);
                    sand.move_to(sand.x, sand.y + 1);
                    grid.record_sand(sand.x as usize, sand.y as usize);
                } else if ! grid.is_ouside_grid(sand.x - 1, sand.y + 1) && 
                grid.can_move_down_left(&sand) {
                    println!("Moving left");
                    grid.clear_sand(&sand);
                    sand.move_to(sand.x - 1, sand.y + 1);
                    grid.record_sand(sand.x as usize, sand.y as usize);
                } else 
                if ! grid.is_ouside_grid(sand.x + 1, sand.y + 1) &&
                 grid.can_move_down_right(&sand)  {
                    println!("Moving right");
                    grid.clear_sand(&sand);
                    sand.move_to(sand.x + 1, sand.y + 1);
                    grid.record_sand(sand.x as usize, sand.y as usize);
                } else {
                    state = State::Spawn;
                }

            }
        }
        println!("{i}");
        i += 1;
        if i > 203 {
            println!("Sand count = {sand_count}");
            break;
        }
        // if no progress... finish
    }
}