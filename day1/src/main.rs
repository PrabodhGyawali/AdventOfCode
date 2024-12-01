
// TODO: Binary Sort Algorithm in Rust
// TODO: Request to get data and parse through it

fn extract_puzzle_input() {
	
}

fn similarity_score(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
	let mut score: i32 = 0;
	
	// Sort list

	// Each list index find difference
	for n in 0..a.len() {
		score += (a[n] - b[n]).abs();
	}

	score
}

fn main() {
    let a = vec![1, 2, 3];
	let b = vec![2, 2, 10];
	let result = similarity_score(&a, &b);
	println!("result: {:?}", result);
}
