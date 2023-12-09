pub fn get_top_num() {
    let contents = std::fs::read_to_string("docs/9/chain.txt").expect("whoops");

    let data: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|str| str.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let rec_sum: i32 = data.iter().map(|vec| vec.as_slice()).map(call_rec).sum();
    println!("recursive calc: {}", rec_sum);
    let rev: i32 = data.iter().map(|vec| vec.as_slice()).map(calc_rev).sum();
    println!("the reverse is {}", rev);
}

fn calc_rev(slice: &[i32]) -> i32 {
    let mut rev = slice.to_owned();
    rev.reverse();
    call_rec(rev.as_slice())
}

fn call_rec(slice: &[i32]) -> i32 {
    slice.last().unwrap() + rec_calc(slice)
}

fn rec_calc(slice: &[i32]) -> i32 {
    let into: Vec<i32> = slice.windows(2).map(|tup| tup[1] - tup[0]).collect();
    if into.iter().all(|i| i == &0) {
        0
    } else {
        into.last().unwrap() + rec_calc(into.as_slice())
    }
}
