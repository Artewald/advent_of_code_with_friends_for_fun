use std::fs::read_to_string;

/** Read the file */
fn read_file(path: &str) -> String {
    read_to_string(path).unwrap()
}

pub fn parse_file_tim(path: &str) -> i32 {
    let mut top_three = Vec::with_capacity(4);
    top_three.push(0);
    top_three.push(0);
    top_three.push(0);
    let mut current_num = 0;
    read_file(path).lines().for_each(|line| {
        if !line.eq("") {
            let num = line.parse::<i32>().unwrap();
            current_num += num;
        } else {
            top_three.push(current_num);
            top_three.sort();
            top_three.remove(0);
            current_num = 0;
        }
    });
    return top_three.iter().sum();
}