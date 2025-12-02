use aoc_utils::read_lines_buffered;

const N: i32 = 100;

fn main() {
    let lines = match read_lines_buffered("crates/day01/input.txt") {
        Ok(it) => it,
        Err(e) => { eprintln!("Failed to open file: {e}"); return; }
    };

    let mut start = 50;
    let mut count = 0;
    for line_result in lines {
        match line_result {
            Ok(line) => {
                let (direction, length_str) = line.split_at(1);
                let length: i32 = length_str.parse().expect("Invalid number {length_str}");
                match direction {
                    "L" => { start = (start - length).rem_euclid(N); }
                    "R" => { start = (start + length).rem_euclid(N); }
                    other => panic!("unexpected direction {other}"),
                }
                if start == 0 { count += 1; }
            },
            Err(e) => eprintln!("Error reading lines: {e}"),
        }
    }
    println!("{count}");
}
