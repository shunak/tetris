use std::{thread, time};
use getch_rs::{Getch, Key};

const FIELD_WIDTH: usize = 11+2; // num of field + num of walls
const FIELD_HEIGHT: usize = 20+1; // num of field + num of bottom
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT]; //[V;N] V: Value, N: size of array

// kind of blocks
#[derive(Clone, Copy)]
enum BlockKind{
    I,O,S,Z,J,L,T,
}

type BlockShape = [[usize; 4]; 4];

const BLOCKS: [BlockShape; 7] = [

        // I block
        [
            [0,0,0,0],
            [0,0,0,0],
            [1,1,1,1],
            [0,0,0,0],
        ],
        // O block
        [
            [0,0,0,0],
            [0,1,1,0],
            [0,1,1,0],
            [0,0,0,0],
        ],
        // S block
        [
            [0,0,0,0],
            [0,1,1,0],
            [1,1,0,0],
            [0,0,0,0],
        ],
        // Z block
        [
            [0,0,0,0],
            [1,1,0,0],
            [0,1,1,0],
            [0,0,0,0],
        ],
        // J block
        [
            [0,0,0,0],
            [1,0,0,0],
            [1,1,1,0],
            [0,0,0,0],
        ],
        // L block
        [
            [0,0,0,0],
            [0,0,1,0],
            [1,1,1,0],
            [0,0,0,0],
        ],
        // T block
        [
            [0,0,0,0],
            [0,1,0,0],
            [1,1,1,0],
            [0,0,0,0],
        ],

];

struct Position {
    x: usize,
    y: usize,
}

// collision checker
fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4{
        for x in 0..4 {
            if field[y+pos.y][x+pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn main() {

    let field = [
        [1,1,1,0,0,0,0,0,0,0,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1],
    ];

    let mut pos = Position {x:4, y:0};
    let g = Getch::new();

    // Clear screen
     println!("\x1b[2J\x1b[H\x1b[?25l");


     loop{

        // Generate Field for draw.
        let mut field_buf = field;

        // Judge collision
        if !is_collision(&field, &pos, BlockKind::I){
            // upate pos y
            pos.y += 1;
        }


        // Write info of tetri mino to Field
        for y in 0..4 {
            for x in 0..4{
                if BLOCKS[BlockKind::J as usize][y][x] == 1 {
                    field_buf[y+pos.y][x+pos.x] = 1;
                }
            }
        }

// Move cursor to Top
 println!("\x1b[H");

        // Field for draw
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                if field_buf[y][x] == 1{
                    print!("[]");
                }else{
                    print!(" .");
                }
            }
            println!();
        }


    // Sleep for 1 sec
    thread::sleep(time::Duration::from_millis(1000));

    // Break out loop by key 'q'
        match g.getch(){
            Ok(Key::Char('q')) => break,
            _ => (), // do nothing
        }

    }

    // Re-display cursor
     println!("\x1b[?25h");



}
