use aoc_utils::read_lines_buffered;

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
                let length: i32 = length_str.parse().expect("Invalid number {}");
                if direction == "L" {
                    start -= length;
                    while start < 0 { start += 100 }
                }
                if direction == "R" {
                    start += length;
                    while start > 99 { start -= 100 }
                }
                println!("start {start}");
                if start == 0 { count += 1 }
            },
            Err(e) => eprintln!("Error reading lines: {e}"),
        }
    }
    println!("{count}");
}
