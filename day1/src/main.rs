use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

/* source: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn extract_puzzle_input(input_path: &str, a: &mut Vec<i32>, map_b: &mut HashMap<i32, i32>) {
    if let Ok(lines) = read_lines(input_path) {
		for line in lines.flatten() {
			let tokens : Vec<&str> = line.split("   ").collect();
            let num_a : i32 = tokens[0].parse::<i32>().unwrap();
            let num_b : i32 = tokens[1].parse::<i32>().unwrap();

            
            if map_b.get(&num_b).is_some() {
                map_b.entry(num_b).and_modify(|value| *value += 1);
            } else { 
                map_b.insert(num_b, 1);
            }
            a.push(num_a);
		}
	}
}

fn sort_vec(arr: &mut Vec<i32>) {
	for i in 1..arr.len() {
		let elem = arr[i];
		let mut j = i as isize - 1;
	
		while j >= 0 && arr[j as usize] > elem {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
		arr[(j + 1) as usize] = elem;
	}
} // usize nonsense is dumb in Rust

fn similarity_score(a: &mut Vec<i32>, map_b: &mut HashMap<i32, i32>) -> i32 {
	let mut score: i32 = 0;


	for n in 0..a.len() {
		score += (a[n] * map_b.get(&a[n]).unwrap_or(&0)).abs();
	} // Improvement use: .iter().for_each(|v| { ... });

	score
}

fn main() {
	let (mut a, mut b): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());
	let (mut map_a, mut map_b): (HashMap<i32, i32>, HashMap<i32, i32>) = (HashMap::new(), HashMap::new());
    extract_puzzle_input("./input.txt", &mut a, &mut map_b);
    print!("{}", similarity_score(&mut a, &mut map_b))

}
