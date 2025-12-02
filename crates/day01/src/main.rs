use aoc_utils::read_lines_buffered;

const N: i32 = 100;

fn apply_move(start: i32, delta: i32) -> i32 {
    (start + delta).rem_euclid(N) 
}

fn zeros_on_right_move(start: i32, len: i32) -> i32 {
    if len <= 0 { return 0; }
    let steps_to_first_zero = if start == 0 { N } else { N - start };
    if len < steps_to_first_zero { 0 }
    else { 1 + (len - steps_to_first_zero) / N }
}

fn zeros_on_left_move(start: i32, len: i32) -> i32 {
    if len <= 0 { return 0; }
    let steps_to_first_zero = if start == 0 { N } else { start };
    if len < steps_to_first_zero { 0 }
    else { 1 + (len - steps_to_first_zero) / N }
}

fn main() {
    let lines = match read_lines_buffered("crates/day01/input.txt") {
        Ok(it) => it,
        Err(e) => { eprintln!("Failed to open file: {e}"); return; }
    };

    let mut start = 50;
    let mut count_part1 = 0;
    let mut count_part2 = 0;
    for line_result in lines {
        match line_result {
            Ok(line) => {
                let (direction, length_str) = line.split_at(1);
                let length: i32 = length_str.parse().expect("Invalid number {length_str}");
                match direction {
                    "L" => { 
                        count_part2 += zeros_on_left_move(start, length);
                        start = apply_move(start, -length);
                    }
                    "R" => { 
                        count_part2 += zeros_on_right_move(start, length);
                        start = apply_move(start, length);
                    }
                    other => panic!("unexpected direction {other}"),
                }
                if start == 0 { count_part1 += 1; }
            },
            Err(e) => eprintln!("Error reading lines: {e}"),
        }
    }
    println!("part 1: {count_part1}");
    println!("part 2: {count_part2}");
}
