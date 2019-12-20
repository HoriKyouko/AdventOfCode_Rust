use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong!");

    let strings : Vec<&str> = contents.split(',').collect();
    let mut counter = 0;

    for i in strings {
        if i.contains("\r\n"){
            let (f, _dc) = i.split_at(4);
            let (_rdc, s) = i.split_at(6);
            let (dir, val) = f.split_at(1);
            let (dir2, val2) = s.split_at(1);
            println!("{}   {}", dir, val.parse::<i32>().unwrap());
            println!("{}   {}", dir2, val2.parse::<i32>().unwrap());
        }
        else{
            let (first, rest) = i.split_at(1);
            print!("{}   ", first);
            let val = rest.parse::<i32>().unwrap();
            println!("{}", val);
        }
    }

    println!("Counter = {}", counter);
    Ok(())
}
