use std::io::{self,Write};

#[derive(Clone, Debug, Default)]
pub struct Process {
    pub pid: usize,
    pub arrival_time: usize,
    pub burst_time: usize,
    pub start_time: usize,
    pub completion_time: usize,
    pub turnaround_time: usize,
    pub waiting_time: usize,
    pub response_time: usize,
}

pub fn input() -> Vec<Process>{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("unable to read from stdin");
    let n = buf.trim().parse::<usize>().unwrap();
    let mut p = vec![Process::default(); n];

    for i in 0..n {
        print!("Enter arrival time of process {}: ", i + 1);
        io::stdout().flush().expect("unable to write to stdout");
        buf.clear();
        io::stdin().read_line(&mut buf).expect("unable to read from stdin");
        p[i].arrival_time = buf.trim().parse().unwrap();

        print!("Enter burst time of process {}: ", i + 1);
        io::stdout().flush().expect("unable to write to stdout");
        buf.clear();
        io::stdin().read_line(&mut buf).expect("unable to read from stdin");
        p[i].burst_time = buf.trim().parse().unwrap();

        p[i].pid = i + 1;
    }
    p
}


impl Process{
    pub fn print_table(process: Vec<Process>){
        println!("");
        println!("#P\tAT\tBT\tST\tCT\tTAT\tWT\tRT\n");

        for ele in process {
            println!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                ele.pid,
                ele.arrival_time,
                ele.burst_time,
                ele.start_time,
                ele.completion_time,
                ele.turnaround_time,
                ele.waiting_time,
                ele.response_time
            );
        }
    }
    pub fn print_info(process: Vec<Process>){
        let (mut csum,mut wsum,mut tsum,mut rsum) = (0,0,0,0);
        for val in process.iter(){
            csum += val.completion_time;
            wsum += val.waiting_time;
            tsum += val.turnaround_time;
            rsum += val.response_time;
        }
        println!("Avg Completion time: {}",csum as f64 / process.len() as f64);
        println!("Avg Waiting time: {}",wsum as f64 / process.len() as f64);
        println!("Avg turnaround time: {}",tsum as f64 / process.len() as f64);
        println!("Avg Reponse time: {}",rsum as f64 / process.len() as f64);
        
    }
    pub fn print_throughput(process_size: usize,time: usize){
        println!("Through Put: {:.2}",(process_size as f64 / time as f64)*100f64);
    }
}
