use aoc_utils::read_lines_buffered;

fn main() {
    let lines = match read_lines_buffered("crates/day01/test.txt") {
        Ok(it) => it,
        Err(e) => { eprintln!("Failed to open file: {e}"); return; }
    };

    for line_result in lines {
        match line_result {
            Ok(line) => println!("{line}"),
            Err(e) => eprintln!("Error reading lines: {e}"),
        }
    }
}
