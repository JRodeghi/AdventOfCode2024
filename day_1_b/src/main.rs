use std::collections::HashMap;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let contents = fs::read_to_string(file_path)
                            .expect("Unable to read the files");

    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for lines in contents.lines(){
        let mut split_line = lines.split("   ");
        let val1 = split_line.next().expect("Failed to get get the first value");
        let val2 = split_line.next().expect("Failed to get get the first value");
        
        list_one.push(val1.parse::<i32>().expect("value is not a valid number"));
        list_two.push(val2.parse::<i32>().expect("value is not a valid number"));
    }

    list_one.sort();
    list_two.sort();

    let mut sim_score = 0;
    let mut scores:HashMap<i32, i32>= HashMap::new();

    for val in list_one
    {
        match scores.get(&val){
            Some(&score) => {
                sim_score = sim_score + score;
            },
            _ => {
                let mut count = 0;
                for j in &list_two{
                    if *j == val
                    {
                        count += 1;
                    }

                    if *j != val && count != 0
                    {
                        break;
                    }
                }

                sim_score = sim_score + (val*count);
                
                scores.insert(val, val*count);

            },
        }
    }

    println!("Simularity Score: {sim_score}");
}
