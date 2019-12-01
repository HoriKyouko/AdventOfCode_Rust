use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut sum :i32 = 0;

    let contents = fs::read_to_string(filename).expect("Something went wrong!");

    // Part 1
    /*
    for line in contents.lines() {
        let mass : i32 = line.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        sum += get_new_mass(mass);
    }
    */

    // Part 2
    for line in contents.lines() {
        let mass :i32 = line.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        sum += mass_of_fuel(mass);
    }
    
    println!("{}", sum);

    Ok(())
}

fn mass_of_fuel(mass :i32) -> i32 {
    let mut fuel_mass = 0;
    let mut new_mass = mass;

    loop {
        new_mass = get_new_mass(new_mass);
        if new_mass > 0 {
            fuel_mass += new_mass;
        }
        else{
            break;
        }
    }
    fuel_mass
}

fn get_new_mass(mass: i32) -> i32{
    (mass / 3) - 2
}