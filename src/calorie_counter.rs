use crate::file_reader::read_file;

fn calorie_list(path: &str) -> Vec<i64>{
    let file = read_file(path);
    let mut calories = Vec::new();
    let mut current_elf_calories = 0;
    for line in file.lines() {
        if (line == "") {
            calories.push(current_elf_calories);
            current_elf_calories = 0;
        }
        else {
            current_elf_calories += line.parse::<i64>().unwrap();
        }
    }
    return calories;
}

/** Find the n largest calories in the file at the given path.
 *  Returns a vector of the n largest calories.
 */
pub fn find_n_largest_calories(path: &str, n: i64) -> Vec<i64> {
    let mut calories = calorie_list(path);
    let mut largest_calories = Vec::new();
    calories.sort();
    for i in 0..n {
        largest_calories.push(calories[calories.len() - 1 - i as usize]);
    }
    return largest_calories;
}

/** Find the sum of the n largest calories in the file at the given path.*/
pub fn sum_calories(path: &str, n: i64) -> i64 {
    let calories = find_n_largest_calories(path, n);
    let mut sum = 0;
    for calorie in calories {
        sum += calorie;
    }
    return sum;
}

