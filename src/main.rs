use std::time;
use std::thread;
use rand::Rng;

// Constants
const SIZE_X: usize = 10; // Width of the canvas
const SIZE_Y: usize = 10; // Height of the canvas
const MS: u64 = 500; // milliseconds between each display

// Displays an array
fn print_array(a: &[i32]) {
    for i in 0..a.len() {
        let k;
        if a[i] == 1 {
            k = '■';
        } else {
            k = '⠀';
        }
        if i < a.len() - 1 {
            print!("{} ", k);
        } else {
            println!("{}", k);
        }
    }
}

// Displays a matrix
fn print_matrix(m: &[[i32; SIZE_X]; SIZE_Y]) {
    for i in 0..m.len() {
        print_array(&m[i]);
    }
    println!(" ");
}

// Updates the canvas
// If a dead cell is surrounded by exactly three live cells it becomes a live cell. Else it stays a dead cell.
// If a live cell is surrounded by exactly two or three live cells it stays alive. Else it becomes a dead cells
fn update(m: &mut [[i32; SIZE_X]; SIZE_Y]) {
    let mut new_m = [[0; SIZE_X]; SIZE_Y];

    for j in 0..m.len() {
        for i in 0..m[j].len() {
            let mut tot = 0;

            if i != 0 {
                tot = tot + m[j][i - 1];
            }
            if i != SIZE_X - 1 {
                tot = tot + m[j][i + 1];
            }
            if j != 0 {
                tot = tot + m[j - 1][i];
            }
            if j != SIZE_Y - 1 {
                tot = tot + m[j + 1][i];
            }
            if i != 0 && j != 0 {
                tot = tot + m[j - 1][i - 1];
            }
            if i != SIZE_X - 1 && j != 0 {
                tot = tot + m[j - 1][i + 1];
            }
            if j != SIZE_Y - 1 && i != 0 {
                tot = tot + m[j + 1][i - 1];
            }
            if j != SIZE_Y - 1 && i != SIZE_X - 1 {
                tot = tot + m[j + 1][i + 1];
            }
            if m[j][i] == 0 {
                if tot == 3 {
                    new_m[j][i] = 1;
                }
            }
            if m[j][i] == 1 {
                if tot == 3 || tot == 2 {
                    new_m[j][i] = 1;
                }
            }
        }
    }
    m.copy_from_slice(&new_m);
}

// Checks if every cell is dead or not
fn check(m: &[[i32; SIZE_X]; SIZE_Y]) -> bool {
    let mut ret = false;
    for j in 0..m.len() {
        for i in 0..m[j].len() {
            if m[j][i] != 0 {
                ret = true;
                break;
            }
        }
    }
    return ret;
}

fn random_config(m: &mut [[i32; SIZE_X]; SIZE_Y]) {
    for j in 0..m.len() {
        for i in 0..m[j].len() {
            m[j][i] = rand::thread_rng().gen_range(0..=1);
        }
    }
}

fn main() {
    print!("\x1b[2J");

    let mut m = [[0; SIZE_X]; SIZE_Y];

    random_config(&mut m);

    let mut i = 0;
    while check(&m) {
        print!("\x1b[H");

        println!("gen : {i}");

        print_matrix(&m);
        update(&mut m);

        let time = time::Duration::from_millis(MS);
        thread::sleep(time);

        i += 1;
    }
}
