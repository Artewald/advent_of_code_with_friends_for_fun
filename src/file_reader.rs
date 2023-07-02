use std::fs::read_to_string;

/** Read the file */
pub fn read_file(path: &str) -> String {
    read_to_string(path).unwrap()//.lines().collect::<Vec<String>>()}
}

pub fn parse_file(path: &str) -> i64 {
    let mut biggest_num = -0;
    let mut current_num = -0;
    read_file(path).lines().for_each(|line| {
        if !line.eq("") {
            let num = line.parse::<i64>().unwrap();
            current_num += num;
        } else {
            if current_num > biggest_num {
                biggest_num = current_num;
            }
            current_num = 0;
        }
    });
    return biggest_num;
}



