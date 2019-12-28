use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong!");
    let strings : Vec<&str> = contents.split(',').collect();

    let origin_x = 0;
    let origin_y = 0;
    let mut cur_x = origin_x;
    let mut cur_y = origin_y;
    let mut min_dist = i32::max_value();
    let mut steps_done = i32::max_value();
    let mut steps = 0;
    let mut line = 1;
    let mut map : HashMap<(i32, i32), i32> = HashMap::new();
    let mut point : (i32, i32);
    
    for i in strings {
        let mut dir_x = 0;
        let mut dir_y = 0;

        if i.contains("\r\n") {
            let (f, _dont_care) = i.split_at(4);
            let (_really_dont_care, s) = i.split_at(6);
            let (first, first_next) = f.split_at(1);
            let (second, second_next) = s.split_at(1);

            let mut val = first_next.parse::<i32>().unwrap();

            if first == "L" {
                dir_x = -1;
                dir_y = 0;
            }
            else if first == "R" {
                dir_x = 1;
                dir_y = 0;
            }
            else if first == "U" {
                dir_x = 0;
                dir_y = 1;
            }
            else if first == "D" {
                dir_x = 0;
                dir_y = -1;
            }

            while val > 0 {
                cur_x += dir_x;
                cur_y += dir_y;
                steps += 1;
                point = (cur_x, cur_y);

                if line == 2 && map.contains_key(&point) {
                    let dist = manhattan_distance(origin_x, origin_y, cur_x, cur_y);
                    let tally = steps + map[&point];
                    if dist != 0 && tally < steps_done{
                        steps_done = tally;
                    }
                    if dist != 0 && dist < min_dist {
                        min_dist = dist;
                    }
                } 
                else if line == 1 {
                    map.insert(point, steps);
                }
                val -= 1;
            }

            line += 1;
            val = second_next.parse::<i32>().unwrap();
            cur_x = 0;
            cur_y = 0;
            steps = 0;

            if second == "L" {
                dir_x = -1;
                dir_y = 0;
            }
            else if second == "R" {
                dir_x = 1;
                dir_y = 0;
            }
            else if second == "U" {
                dir_x = 0;
                dir_y = 1;
            }
            else if second == "D" {
                dir_x = 0;
                dir_y = -1;
            }

            while val > 0 {
                cur_x += dir_x;
                cur_y += dir_y;
                steps += 1;
                point = (cur_x, cur_y);

                if line == 2 && map.contains_key(&point) {
                    let dist = manhattan_distance(origin_x, origin_y, cur_x, cur_y);
                    let tally = steps + map[&point];
                    if dist != 0 && tally < steps_done{
                        steps_done = tally;
                    }
                    if dist != 0 && dist < min_dist {
                        min_dist = dist;
                    }
                }
                else if line == 1 {
                    map.insert(point, steps);
                }
                val -= 1;
            }

        }
        else{
            let (first, rest) = i.split_at(1);
            let mut val = rest.parse::<i32>().unwrap();

            if first == "L" {
                dir_x = -1;
                dir_y = 0;
            }
            else if first == "R" {
                dir_x = 1;
                dir_y = 0;
            }
            else if first == "U" {
                dir_x = 0;
                dir_y = 1;
            }
            else if first == "D" {
                dir_x = 0;
                dir_y = -1;
            }

            while val > 0 {
                cur_x += dir_x;
                cur_y += dir_y;
                steps += 1;
                point = (cur_x, cur_y);

                if line == 2 && map.contains_key(&point) {
                    let dist = manhattan_distance(origin_x, origin_y, cur_x, cur_y);
                    let tally = steps + map[&point];
                    if dist != 0 && tally < steps_done{
                        steps_done = tally;
                    }
                    if dist != 0 && dist < min_dist {
                        min_dist = dist;
                    }
                }
                else if line == 1 {
                    map.insert(point, steps);
                }
                val -= 1;
            }
        }
    }

    println!("Min Distance: {} Steps_done: {}", min_dist, steps_done);
    Ok(())
}

fn manhattan_distance(x1:i32, y1:i32, x2:i32, y2:i32) -> i32 {
    i32::abs(x1-x2) + i32::abs(y1-y2)
}