use std::fs;

pub fn map_seed() {
    let file1 = "docs/5/map.txt";
    // let file1 = "docs/5/short.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");
    let (seeds, maps) = contents.split_once("\n\n").unwrap();

    let (_, seed_str) = seeds.split_once(": ").unwrap();
    let seed_nr = seed_str.split_ascii_whitespace();

    let mut seed_soil_map = vec![];
    let mut soil_fertilizer_map = vec![];
    let mut fertilizer_water_map = vec![];
    let mut water_light_map = vec![];
    let mut light_temperature_map = vec![];
    let mut temperature_humidity_map = vec![];
    let mut humidity_location_map = vec![];
    let mut fake_map = vec![];

    for (i, raw_map_info) in maps.split("\n\n").enumerate() {
        let curr_map = match i {
            0 => &mut seed_soil_map,
            1 => &mut soil_fertilizer_map,
            2 => &mut fertilizer_water_map,
            3 => &mut water_light_map,
            4 => &mut light_temperature_map,
            5 => &mut temperature_humidity_map,
            6 => &mut humidity_location_map,
            _ => {
                println!("out of scope");
                &mut fake_map
            }
        };
        for (j, line) in raw_map_info.lines().enumerate() {
            if j != 0 {
                let values: Vec<i64> = line
                    .split_ascii_whitespace()
                    .map(|str| str.parse::<i64>().unwrap())
                    .collect();
                curr_map.push(values);
            }
        }
    }

    let low_nr = seed_nr
        .map(|str| str.parse::<i64>().unwrap())
        .map(|item| calc_next_val(item, &seed_soil_map))
        .map(|item| calc_next_val(item, &soil_fertilizer_map))
        .map(|item| calc_next_val(item, &fertilizer_water_map))
        .map(|item| calc_next_val(item, &water_light_map))
        .map(|item| calc_next_val(item, &light_temperature_map))
        .map(|item| calc_next_val(item, &temperature_humidity_map))
        .map(|item| calc_next_val(item, &humidity_location_map))
        .min()
        .unwrap();

    println!("low nr {:?} ", low_nr);
}

pub fn map_seed_range() {
    let file1 = "docs/5/map.txt";
    // let file1 = "docs/5/short.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");
    let (seeds, maps) = contents.split_once("\n\n").unwrap();

    let (_, seed_str) = seeds.split_once(": ").unwrap();
    let seed_nr = seed_str.split_ascii_whitespace();
    let mut seed_vec: Vec<&str> = seed_nr.clone().collect();

    let mut seed_tup_vec = vec![];

    while !seed_vec.is_empty() {
        let x = seed_vec.pop().unwrap().parse::<i64>().unwrap();
        let y = seed_vec.pop().unwrap().parse::<i64>().unwrap();
        let x = x + y - 1;
        seed_tup_vec.push((y, x));
    }

    let mut seed_soil_map = vec![];
    let mut soil_fertilizer_map = vec![];
    let mut fertilizer_water_map = vec![];
    let mut water_light_map = vec![];
    let mut light_temperature_map = vec![];
    let mut temperature_humidity_map = vec![];
    let mut humidity_location_map = vec![];
    let mut fake_map = vec![];

    for (i, raw_map_info) in maps.split("\n\n").enumerate() {
        let curr_map = match i {
            0 => &mut seed_soil_map,
            1 => &mut soil_fertilizer_map,
            2 => &mut fertilizer_water_map,
            3 => &mut water_light_map,
            4 => &mut light_temperature_map,
            5 => &mut temperature_humidity_map,
            6 => &mut humidity_location_map,
            _ => {
                println!("out of scope");
                &mut fake_map
            }
        };
        for (j, line) in raw_map_info.lines().enumerate() {
            if j != 0 {
                let values: Vec<i64> = line
                    .split_ascii_whitespace()
                    .map(|str| str.parse::<i64>().unwrap())
                    .collect();
                curr_map.push(values);
            }
        }
    }

    let low_nr = seed_tup_vec
        .next_map(seed_soil_map)
        .next_map(soil_fertilizer_map)
        .next_map(fertilizer_water_map)
        .next_map(water_light_map)
        .next_map(light_temperature_map)
        .next_map(temperature_humidity_map)
        .next_map(humidity_location_map)
        .iter()
        .map(|tup| tup.0)
        .min()
        .unwrap();

    println!("low nr {:?} ", low_nr);
}

trait Maping {
    fn next_map(self, maps: Vec<Vec<i64>>) -> Self;
}

impl Maping for Vec<(i64, i64)> {
    fn next_map(self, maps: Vec<Vec<i64>>) -> Vec<(i64, i64)> {
        cal_next_map_tup(self, maps)
    }
}

fn cal_next_map_tup(tup: Vec<(i64, i64)>, maps: Vec<Vec<i64>>) -> Vec<(i64, i64)> {
    let mut ret_map = vec![];
    let mut work_tuup = tup;
    for map in maps {
        let mut temp_tuup = vec![];
        let start = map[1];
        let end = map[1] + map[2] - 1;
        let dest = map[0];
        for set in &work_tuup {
            let (tup_start, tup_end) = *set;
            if tup_start < end && start < tup_end {
                ret_map.push((
                    dest + tup_start.max(start) - start,
                    dest + tup_end.min(end) - start,
                ));
                if tup_start < start {
                    temp_tuup.push((tup_start, start - 1))
                }
                if end < tup_end {
                    temp_tuup.push((end + 1, tup_end))
                }
            } else {
                temp_tuup.push(*set);
            }
        }
        work_tuup = temp_tuup;
    }
    ret_map.append(&mut work_tuup);
    ret_map
}

fn calc_next_val(input: i64, vec: &Vec<Vec<i64>>) -> i64 {
    for list in vec {
        let x = list[2] + list[1];
        if list[1] <= input && input <= x {
            return list[0] + (input - list[1]);
        }
    }
    input
}
