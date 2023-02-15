use std::io::{stdout, Write};

use non_primitive::algo::{common::input, *};

fn main() {
    let stdin = std::io::stdin();
    let algos = ["FCFS", "SJF", "LJF", "PSNP", "HRRN"];
    println!("Choose one of the algo below");
    for (index, ele) in algos.iter().enumerate() {
        println!("[{}]   {ele}", index + 1);
    }
    print!("Enter the algo which you would like to choose: ");
    stdout().flush().expect("Unable to write to stdout");
    let mut algo = String::new();
    stdin.read_line(&mut algo).expect("Unable to read stdin");
    algo = algo.trim().to_string();
    let mut input = input();
    match algo.as_str() {
        "1" => fcfs::fcfs(&mut input),
        "2" => sjf::sjf(),
        "3" => ljf::ljf(),
        "4" => psnr::psnr(),
        "5" => hrrn::hrrn(),
        _ => unreachable!(),
    }
}
