use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let contents = fs::read_to_string(file_path)
                            .expect("Unable to read the files");

    let mut safe = 0;
    
    for lines in contents.lines()
    {
        let mut is_decreasing = false;
        let mut is_safe = true;
        let mut level:Vec<i32> = Vec::new();

        for vals in lines.split(" ")
        {
            level.push(vals.parse::<i32>().expect("value is not a valid number"));
        }

        for i in 1..level.len()
        {
            if (level[i-1]-level[i]).abs() < 1 || (level[i-1]-level[i]).abs() > 3
            {
                is_safe = false;
            }
            else 
            {
                if level[i-1] > level[i]
                {
                    if i == 1 
                    {
                        is_decreasing = true;
                    }
                    else {
                        if !is_decreasing
                        {
                            is_safe = false;
                        }
                    }
                }
                else {
                    if i != 1 && is_decreasing
                    {
                        is_safe = false;
                    }
                }
                
            }
        }

        if is_safe
        {
            safe = safe + 1;
        }
    }

    println!("Safe: {safe}")
}
