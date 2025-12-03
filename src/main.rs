mod day1;
mod day2;

mod utils;

fn main() {
    let start = std::time::Instant::now();
    day2::part2::run();
    let duration = std::time::Instant::now() - start;
    println!("finished in {duration:?}");
}
