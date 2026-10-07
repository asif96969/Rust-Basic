use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::str::FromStr;
 
/// Prints `prompt`, reads a line from standard input,
/// and returns it parsed as `T`. Panics on errors.
pub fn input<T: FromStr>(prompt: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_end().parse().unwrap()
}
 
/// Opens the given file and returns an object that iterates over its lines.
/// Panics on any I/O error.
pub fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path)
        .unwrap_or_else(|e| panic!("Failed to open file '{}': {}", path, e));
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_else(|e| panic!("Error while reading line: {}", e)))
}

pub fn type_of<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}