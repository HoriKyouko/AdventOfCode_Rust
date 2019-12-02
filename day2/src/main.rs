use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong!");
    let mut numbers: Vec<i32> = contents.split(',').map(|contents| contents.trim().parse::<i32>().unwrap()).collect();
   
    let mut noun = 1;
    let mut verb = 1;
    let numbers_clone = numbers;

    // Part 2
    loop{
        let mut counter = 0;
        numbers = numbers_clone.clone();
        numbers[1] = noun;
        numbers[2] = verb;
        // Part 1
        while counter <= numbers.len() {
            let op = numbers[counter];
            let num1 = numbers[counter + 1];
            let num2 = numbers[counter + 2];
            let pos = numbers[ counter + 3];
    
            if op == 1 {
                numbers[pos as usize] = add_numbers(numbers[num1 as usize], numbers[num2 as usize]);
            }
            else if op == 2 {
                numbers[pos as usize] = mult_numbers(numbers[num1 as usize], numbers[num2 as usize]);
            }
            else if op == 99 {
                break;
            }
            else{
                println!("Something went terribly wrong!");
                break;
            }
            counter += 4;
        }

        if numbers[0] == 19690720 {
            break;
        }
        
        else if noun != 99 {
            noun += 1;
        }
        else if verb != 99 {
            noun = 1;
            verb += 1;
        }
        else {
            println!("ERROR OCCURED BREAKING OUT");
            break;
        }
    }

    println!("\nThe answer should be: {} \nNoun = {} & Verb = {}\nAns = {}", numbers[0], noun, verb, 100 * noun + verb);

    Ok(())
}

fn add_numbers(num1:i32, num2:i32) -> i32 {
    num1 + num2
}

fn mult_numbers(num1:i32, num2:i32) -> i32 {
    num1 * num2
}