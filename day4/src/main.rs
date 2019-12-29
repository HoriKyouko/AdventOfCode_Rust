/**
 * --- Day 4: Secure Container ---
 * You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.
 * 
 * However, they do remember a few key facts about the password:
 * 
 * It is a six-digit number.
 * The value is within the range given in your puzzle input.
 * Two adjacent digits are the same (like 22 in 122345).
 * Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
 * Other than the range rule, the following are true:
 * 
 * 111111 meets these criteria (double 11, never decreases).
 * 223450 does not meet these criteria (decreasing pair of digits 50).
 * 123789 does not meet these criteria (no double).
 * How many different passwords within the range given in your puzzle input meet these criteria?
 * 
 * Your puzzle input is 382345-843167.
 * 
 * --- Part Two ---
 * An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.
 * 
 * Given this additional criterion, but still ignoring the range rule, the following are now true:
 * 
 * 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
 * 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
 * 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
 * How many different passwords within the range given in your puzzle input meet all of the criteria?
 * 
 * Your puzzle input is still 382345-843167.
 */

use std::char;
use std::io::Error;

fn main() -> Result<(), Error> {
    let min = 382345;
    let max = 843167;
    let mut count = 0;
    let mut val = min;

    while val < max {
        
        let number = convert_number_to_vec(val);

        if check_double_digits(number.clone()) && check_decreasing_digits(number.clone()) && not_part_of_larger_group(number.clone()) {
            count += 1;
        }

        val += 1;
    }

    println!("Count: {}", count);
    Ok(())
}

// Check for double digits
fn check_double_digits(number : Vec<char>) -> bool {
    let mut i = 0;
    let mut j = 1;
    while j < 6 {
        if number[i] == number[j] {
            return true
        }
        i = j;
        j += 1;
    }
    false
}
// Check for decreasing digits
fn check_decreasing_digits(number : Vec<char>) -> bool {
    let mut i = 0;
    let mut j = 1;
    while j < 6 {
        if number[i] > number[j] {
            return false
        }
        i = j;
        j += 1;
    }
    true
}

fn convert_number_to_vec(mut val: u32) -> Vec<char> {
    let mut number : Vec<char> = vec!['0'; 6];
    let mut temp = 5;
    loop {
        if val == 0 {
            break;
        }
        let rem = val % 10;
        number[temp] = char::from_digit(rem, 10).unwrap();
        val /= 10;
        if temp != 0{
            temp -= 1;
        }   
    }
    number
}

fn not_part_of_larger_group(number : Vec<char>) -> bool {
    (0..5).any(|i| match i {
        0 => (number[0] == number[1]) && (number[0] != number[2]),
        4 => (number[4] == number[5]) && (number[4] != number[3]),
        n => (number[n] == number[n + 1]) && (number[n] != number[n - 1]) && (number[n] != number[n + 2]),
    })
}