use std::{
    fs::File,
    io::{BufRead, BufReader,Result},
    path::Path,
};

pub fn read_lines_buffered<P>(
    filename: P
) -> Result<impl Iterator<Item = Result<String>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
// }
