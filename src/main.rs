use std::{thread, time};

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
fn is_collosion{



}

fn main() {

    let field = [
        [1,1,1,0,0,0,0,0,0,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1],
    ];

    // let mut field_buf = field;

    // for y in 0..4 {
    //     for x in 0..4 {
    //          field_buf[y+ 2][x+2] |= minos[0][y][x];  // I
    //          field_buf[y+ 2][x+7] |= minos[1][y][x];  // O
    //          field_buf[y+ 6][x+2] |= minos[2][y][x];  // S
    //          field_buf[y+ 6][x+7] |= minos[3][y][x];  // Z
    //          field_buf[y+10][x+2] |= minos[4][y][x];  // J
    //          field_buf[y+10][x+7] |= minos[5][y][x];  // L
    //          field_buf[y+14][x+2] |= minos[6][y][x];  // T
    //     }
    // }
    let mut pos = Position {x:4, y:0};

     println!("\x1b[2J\x1b[H\x1b[?25l");


    for _ in 0..5 {

        // Generate Field for draw.
        let mut field_buf = field;

        // Write info of tetri mino to Field
        for y in 0..4 {
            for x in 0..4{
                if BLOCKS[BlockKind::I as usize][y][x] == 1 {
                    field_buf[y+pos.y][x+pos.x] = 1;
                }
            }
        }

        // Update y of pos
        pos.y += 1;

 println!("\x1b[H");

        // Field for draw
        for y in 0..22 {
            for x in 0..12 {
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

    }



     println!("\x1b[?25h");



}
