use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let contents = fs::read_to_string(file_path)
                            .expect("Unable to read the files");

    let mut listOne: Vec<i32> = Vec::new();
    let mut listTwo: Vec<i32> = Vec::new();

    for lines in contents.lines(){
        let mut split_line = lines.split("   ");
        let val1 = split_line.next().expect("Failed to get get the first value");
        let val2 = split_line.next().expect("Failed to get get the first value");
        
        listOne.push(val1.parse::<i32>().expect("value is not a valid number"));
        listTwo.push(val2.parse::<i32>().expect("value is not a valid number"));
    }

    listOne.sort();
    listTwo.sort();

    let mut total = 0;
    
    for i in 0..listOne.len()
    {
        let val1 = listOne[i];
        let val2 = listTwo[i];
        let mut distance = val1-val2;
        if distance < 0
        {
            distance = distance*-1;
        }

        total = total + distance;
        println!("Val1: {val1} val2: {val2} Distance: {distance}")
    }

    println!("Total: {total}")
}
