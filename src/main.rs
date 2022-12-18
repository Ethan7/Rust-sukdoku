/* By Ethan Hughes */
/* Written 12/10/2022 */
use rand::prelude::*;
use std::time::Instant;

const EMPTY: i32 = -1;

//Guess checker
fn sudokuhelper(x: usize, y: usize, guess: i32, undergrid: [[i32; 9]; 9]) -> i32{
	for i in 0..9 {
		if undergrid[i][y] == guess && x != i {
			return 0;
		}
		if undergrid[x][i] == guess && y != i {
			return 0;
		}
	}
	let x0 = x - (x % 3);
	let y0 = y - (y % 3);
	let x1 = x0+3;
	let y1 = y0+3;
	for i in x0..x1 {
		for j in y0..y1 {
			if undergrid[i][j] == guess && (x != i || y != j) {
				return 0;
			}
		}
	}
	return 1;
}

//Recursive backtracking while counting solutions to generate sudoku grid
fn sudokusolver(stopper:&mut i32, undergrid: &mut [[i32; 9]; 9]) -> i32{
	let mut solutions = 0;
	for i in 0..9 {
		for j in 0..9 {
			if (*undergrid)[i][j] == EMPTY {
				for k in 0..9 {
					if sudokuhelper(i, j, k, *undergrid) == 1{
						*stopper += 1;
						if *stopper == 10000 {
							return 2;
						}
						(*undergrid)[i][j] = k;
						//Recursive call until the sudoku is solved
						let test = sudokusolver(stopper, undergrid);
						if test > 0 {
							if test == 2 {
								return 2;
							}
							solutions += 1;
							if solutions == 2 {
								return 1;
							}
						}
						(*undergrid)[i][j] = EMPTY;
					}
				}
				if solutions == 1 {
					return 1;
				}
				return 0;
			}
		}
	}
	return 1;
}

//Generate new undergrid and display it on the main grid
fn sudokugenerate(undergrid: &mut [[i32; 9]; 9]){
	//Allocate grid and set random seed
	let mut rng = rand::thread_rng();
	let mut stopper;
	while {
		//Clear the grid
		for i in 0..9 {
			for j in 0..9 {
				(*undergrid)[i][j] = EMPTY;
			}
		}
		//Add 17 initial random number placements
		let mut x:usize;
		let mut y:usize;
		let mut guess:i32;
		for _i in 0..17 {
			while{
				x = rng.gen_range(0..9);
				y = rng.gen_range(0..9);
				guess = rng.gen_range(0..9);
				sudokuhelper(x, y, guess, *undergrid) == 0
			} {};
			(*undergrid)[x][y] = guess;
		}
		/*
		let mut placements = 0;
		while placements < 17 { // loop terminates on monotonic incr
			let x = rng.gen_range(0..9);
			let y = rng.gen_range(0..9);
			let guess = rng.gen_range(0..9);
			if sudokuhelper(x, y, guess, *undergrid) != 0 {
				(*undergrid)[x][y] = guess;
				placements += 1;
			}
		}
		*/
		stopper = 0;
		sudokusolver(&mut stopper, undergrid) == 0 || stopper == 10000 
	} { };
}

fn main(){
	
	//Generate Sudoku
	let mut undergrid = [[EMPTY; 9]; 9];
	let start = Instant::now();
	sudokugenerate(&mut undergrid);
	let elapsed = start.elapsed();

	println!("Time elapsed: {} microseconds", elapsed.as_micros());

	//print sudoku
	for i in 0..9 {
		for j in 0..9 {
			print!("{}", undergrid[i][j]+1);
		}
		println!("");
	}
}