mod utility;
use std::collections::{HashMap, HashSet};
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

fn part_1_and_2() {
    let input = load_file("resources/day_10_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut rows = split_by_new_line(input);
    let height = rows.len();
    let width = rows[0].chars().count();

    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut y = 0;
    for row in rows {
        let mut x = 0;
        for character in row.chars() {
            if character == '#' {
                asteroids.push(Asteroid { x, y });
            }
            x += 1;
        }
        y += 1;
    }

    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_count = 0;

    // Calculate ratios on x and y, store only 1 ratio per
    for asteroid1 in &asteroids {
        let mut ratios: HashSet<i64> = HashSet::new();

        for asteroid2 in &asteroids {
            if asteroid1.x == asteroid2.x && asteroid1.y == asteroid2.y {
                continue;
            }
            let angle = get_angle(asteroid1.x, asteroid1.y, asteroid2.x, asteroid2.y);
            ratios.insert(angle);
        }
        let count = ratios.len();
        if count > best_count {
            best_x = asteroid1.x;
            best_y = asteroid1.y;
            best_count = count;
        }
    }
    stopwatch.print_elapsed();
    println!("Best X {}", best_x);
    println!("Best Y {}", best_y);
    println!("Best count {}", best_count);

    // Part 2
    let mut ratio_asteroids: HashMap<i64, Vec<Asteroid>> = HashMap::new();

    for asteroid in &asteroids {
        if asteroid.x == best_x && asteroid.y == best_y {
            continue;
        }
        let angle = get_angle(best_x, best_y, asteroid.x, asteroid.y);
        let vec = ratio_asteroids.entry(angle).or_insert(Default::default());
        vec.push(Asteroid {
            x: asteroid.x,
            y: asteroid.y,
        });
    }
    let mut keys: Vec<i64> = ratio_asteroids.keys().map(|k| *k).collect();
    keys.sort();
    // Sort asteroids at each degree by how far they are away from the target asteroid
    for key in &keys {
        let my_vec = ratio_asteroids.get_mut(&key).unwrap();
        my_vec.sort_by(|asteroid1, asteroid2| {
            let asteroid_distance1 =
                (best_x as f64 - asteroid1.x as f64).hypot(best_y as f64 - asteroid1.y as f64);
            let asteroid_distance2 =
                (best_x as f64 - asteroid2.x as f64).hypot(best_y as f64 - asteroid2.y as f64);
            asteroid_distance1.partial_cmp(&asteroid_distance2).unwrap()
        });
    }
    let mut asteroid_x = 0;
    let mut asteroid_y = 0;
    let mut current_key_index = 0;
    for i in (0..200) {
        let mut vaporized = false;
        while vaporized == false {
            let current_key = keys[current_key_index];
            let asteroid_list_option = ratio_asteroids.get_mut(&current_key);
            if let Some(asteroid_list) = asteroid_list_option {
                if asteroid_list.len() > 0 {
                    let vaporized_asteroid = asteroid_list.remove(0);
                    println!(
                        "{}: removed asteroid at {},{} angle {}",
                        i,
                        vaporized_asteroid.x,
                        vaporized_asteroid.y,
                        current_key / 10000
                    );
                    asteroid_x = vaporized_asteroid.x;
                    asteroid_y = vaporized_asteroid.y;
                    vaporized = true;
                }
            }
            current_key_index += 1;
            if current_key_index >= keys.len() {
                current_key_index = 0;
            }
        }
    }
    println!("Final coord: {}, {}", asteroid_x, asteroid_y);
    println!("Result: {}", asteroid_x * 100 + asteroid_y);
}

fn get_angle(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    let mut angle = (y2 as f64 - y1 as f64)
        .atan2(x2 as f64 - x1 as f64)
        .to_degrees()
        + 90.0;
    if angle < 0.0 {
        angle += 360.0;
    }
    (angle * 10000.0) as i64
}

struct Asteroid {
    pub x: i64,
    pub y: i64,
}

fn main() {
    println!("Part 1");
    part_1_and_2();
}
