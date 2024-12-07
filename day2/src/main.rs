use std::{fs};

fn parse_input() -> Vec<Vec<i32>> {
    let mut reports = vec![];
    let contents = fs::read_to_string("./input.txt").unwrap();
    for line in contents.lines() {
        let levels: Vec<i32> = line.split(' ').map(|x|(x.parse::<i32>()).unwrap()).collect();
        reports.push(levels);
    }

    reports
}

fn get_dir(a: &i32, b: &i32) -> i32{
    if a < b {
        return 1;
    } else if a > b {
        return -1;
    }
    return 0
}

fn get_safe_report(reports: &mut Vec<Vec<i32>>) -> usize {
    let count = 
        reports.iter()
        .filter(|levels|{
            let mut safe = true;
            let dir = get_dir(&levels[0], &levels[1]);

            for i in 0..levels.len()-1 {
                let diff = (levels[i] - levels[i+1]).abs();
                let cur_dir = get_dir(&levels[i], &levels[i+1]);

                print!("{}: {} {} || diff: {}; ", i, dir, cur_dir, diff);
                if dir != cur_dir {
                    println!("reached dir");
                    safe = false;
                    break;
                }
                
                if !(diff == 1 || diff == 2 || diff == 3) {
                    println!("reached diff");
                    safe = false;
                    break;
                }
            }
            println!(" ");

            
            safe       
        }).count();
    count
}

fn main() {
    let mut reports: Vec<Vec<i32>> = parse_input();
    println!("{:?}", get_safe_report(&mut reports));
}
