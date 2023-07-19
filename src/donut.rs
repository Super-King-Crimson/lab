#![allow(unused)]

const HEIGHT: u16  = 15;
const WIDTH: u16 = 15;

const BRIGHTNESS: [char; 8] = [
    '.',
    ',',
    ':',
    ';',
    '*',
    '&',
    '$',
    '@',
];

fn test() {
    let mut column = 0;
    let mut row = 0;
    
    let mut bs = 2;

    while row < HEIGHT {
        let mut current_row = String::from("");
        while column < WIDTH {
            if bs >= BRIGHTNESS.len() { bs = usize::from(row + column) % 8; }

            current_row.push(BRIGHTNESS[bs]);

            bs += 1;

            column += 1;
        }
        println!("{current_row}");
        row += 1;
        column = 0;
    }
}