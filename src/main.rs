use std::io::stdin;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn get_input() -> String {
    let mut ret_v = String::new();
    println!("Enter wich day you want to solve; q to exit");
    let _ = stdin().read_line(&mut ret_v);
    if let Some('\n')=ret_v.chars().next_back() {
        ret_v.pop();
    }
    if let Some('\r')=ret_v.chars().next_back() {
        ret_v.pop();
    }
    ret_v
}

fn main() {
    let mut inp = get_input();
    while !(inp.is_empty()) {
        match inp.as_str() {
            "1" => {
                println!("Lets open nr 1!");
                day_1::cal_val::calc_inp();
                inp = get_input();
            }
            "2" => {
                println!("Lets open nr 2!");
                day_2::val_games::validate_games();
                day_2::val_games::sum_games();
                inp = get_input();
            }
            "3" => {
                println!("Lets open nr 3!");
                day_3::engine::get_engine_nr();
                inp = get_input();
            }
            "4" => {
                println!("Lets open nr 4!");
                day_4::card::calc_games();
                day_4::card::calc_cascade();
                inp = get_input();
            }
            "5" => {
                println!("Lets open nr 5!");
                day_5::map::map_seed();
                day_5::map::map_seed_range();
                inp = get_input();
            }
            "6" => {
                println!("Lets open nr 6!");
                day_6::race::calc_nr();
                day_6::race::calc_one_race();
                inp = get_input();
            }
            "7" => {
                println!("Lets open nr 7!");
                inp = get_input();
            }
            "8" => {
                println!("Lets open nr 8!");
                inp = get_input();
            }
            "9" => {
                println!("Lets open nr 9!");
                inp = get_input();
            }
            "10" => {
                println!("Lets open nr 10!");
                inp = get_input();
            }
            "11" => {
                println!("Lets open nr 11!");
                inp = get_input();
            }
            "12" => {
                println!("Lets open nr 12!");
                inp = get_input();
            }
            "13" => {
                println!("Lets open nr 13!");
                inp = get_input();
            }
            "14" => {
                println!("Lets open nr 14!");
                inp = get_input();
            }
            "15" => {
                println!("Lets open nr 15!");
                inp = get_input();
            }
            "16" => {
                println!("Lets open nr 16!");
                inp = get_input();
            }
            "17" => {
                println!("Lets open nr 17!");
                inp = get_input();
            }
            "18" => {
                println!("Lets open nr 18!");
                inp = get_input();
            }
            "19" => {
                println!("Lets open nr 19!");
                inp = get_input();
            }
            "20" => {
                println!("Lets open nr 20!");
                inp = get_input();
            }
            "23" => {
                println!("Lets open nr 23!");
                inp = get_input();
            }
            "24" => {
                println!("Lets open nr 24!");
                inp = get_input();
            }
            "25" => {
                println!("Lets open nr 25!");
                inp = get_input();
            }
            "q" | "Q" => {
                println!("Thank you come again!");
                break;
            }
            _ => {
                println!("Wrong input!");
                println!("try again");
                inp = get_input();
            }            
        }
    }
    println!("got some!");
}
