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
    match algo.as_str() {
        "1" => {
            let mut input = input(false);
            fcfs::fcfs(&mut input);
        }
        "2" => {
            let mut input = input(false);
            sjf::sjf(&mut input);
        }
        "3" => {
            let mut input = input(false);
            ljf::ljf(&mut input);
        }
        "4" => {
            let mut input = input(true);
            psnr::psnr(&mut input);
        }
        "5" => {
            let mut input = input(false);
            hrrn::hrrn(&mut input);
        }
        _ => unreachable!(),
    }
}
