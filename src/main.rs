use cool::{alwaa_2::parse_best, tim_part2::parse_file_tim, calorie_counter::{find_n_largest_calories, sum_calories}};

use crate::file_reader::parse_file;

pub mod file_reader;

const FILE_PATH: &str = "./text.txt";

fn main() {
    // println!("{}", parse_file(FILE_PATH));
    println!("{:?}", parse_best(FILE_PATH));
    println!("{:?}", parse_file_tim(FILE_PATH));
    println!("{:?}", sum_calories(FILE_PATH, 3));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testfile.txt");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "5\n4\n\n2\n1").unwrap();

        let biggest_sum = parse_file(file_path.to_str().unwrap());

        assert_eq!(biggest_sum, 9); // 5 + 4 = 9 is the largest sum of consecutive numbers in the file
    }
}

// .\execute "virus.exe" --force // SOurce trust me bro nothing shady at all