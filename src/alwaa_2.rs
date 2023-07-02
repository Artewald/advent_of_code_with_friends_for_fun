use std::fs::read_to_string;

/** Read the file */
fn read_file(path: &str) -> String {
    read_to_string(path).unwrap()//.lines().collect::<Vec<String>>()}
}

pub fn parse_best(path: &str) -> i64 {
    let mut top3 = [0,0,0]; // [0; 3]
    let mut current_num = 0;
    read_file(path).lines().for_each(|line| {
        if !line.eq("") {
            let num = line.parse::<i64>().unwrap();
            current_num += num;
        } else {
            if top3[0] < current_num{
                top3[0] = current_num;
                top3.sort();
            }
            current_num = 0;
        }
    });
    return top3.iter().sum();
}

// pub fn parse_better(path: &str) -> i64 {
//     let mut top3 = [0,0,0];
//     let mut current_num = 0;
//     for line in read_file(path).lines(){
//         if !line.eq("") {
//             let num = line.parse::<i64>().unwrap();
//             current_num += num;
//         } else {
//             if top3[0] < current_num{
//                 if top3[1] < current_num{
//                     if top[2] < current_num{

//                     }
//                 }
//                 top3[0] = current_num;                 
//                 if top3[1] < top3[0]{
//                     top3[]
//                 }
//             }
//             current_num = 0;
//         }
//     }
//     return top3.iter().sum();
// }
