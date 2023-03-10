use std::io::{self, Write};

#[derive(Clone, Debug, Default)]
pub struct Process {
    pub pid: usize,
    pub priority: usize,
    pub arrival_time: usize,
    pub burst_time: usize,
    pub start_time: usize,
    pub completion_time: usize,
    pub turnaround_time: usize,
    pub waiting_time: usize,
    pub response_time: usize,
}

pub fn input(priority: bool) -> Vec<Process> {
    let mut buf = String::new();
    print!("Enter the no of proceesses: ");
    io::stdout().flush().expect("unable to write to stdout");
    io::stdin()
        .read_line(&mut buf)
        .expect("unable to read from stdin");
    let n = buf.trim().parse::<usize>().unwrap();
    let mut p = vec![Process::default(); n];

    for i in 0..n {
        if priority {
            print!("Enter priority of process {}: ", i + 1);
            io::stdout().flush().expect("unable to write to stdout");
            buf.clear();
            io::stdin()
                .read_line(&mut buf)
                .expect("unable to read from stdin");
            p[i].priority = buf.trim().parse().unwrap();

            p[i].pid = i + 1;
        }
        print!("Enter arrival time of process {}: ", i + 1);
        io::stdout().flush().expect("unable to write to stdout");
        buf.clear();
        io::stdin()
            .read_line(&mut buf)
            .expect("unable to read from stdin");
        p[i].arrival_time = buf.trim().parse().unwrap();

        print!("Enter burst time of process {}: ", i + 1);
        io::stdout().flush().expect("unable to write to stdout");
        buf.clear();
        io::stdin()
            .read_line(&mut buf)
            .expect("unable to read from stdin");
        p[i].burst_time = buf.trim().parse().unwrap();

        p[i].pid = i + 1;
    }
    p
}

impl Process {
    pub fn print_info(process: &mut Vec<Process>, priority: bool) {
        let mut idle_time = process[0].arrival_time;
        for ele in process.windows(2) {
            idle_time += ele[1].start_time - ele[0].completion_time;
        }
        let (mut csum, mut wsum, mut tsum, mut rsum) = (0, 0, 0, 0);
        for val in process.iter() {
            csum += val.completion_time;
            wsum += val.waiting_time;
            tsum += val.turnaround_time;
            rsum += val.response_time;
        }
        println!(
            "\nAvg Completion time: {:.2}",
            csum as f64 / process.len() as f64
        );
        println!(
            "Avg Waiting time: {:.2}",
            wsum as f64 / process.len() as f64
        );
        println!(
            "Avg turnaround time: {:.2}",
            tsum as f64 / process.len() as f64
        );
        println!(
            "CPU utilitzation: {:.2}%",
            ((process[process.len() - 1].completion_time - idle_time) as f64
                / process[process.len() - 1].completion_time as f64)
                * 100f64
        );
        println!(
            "Avg Reponse time: {:.2}",
            rsum as f64 / process.len() as f64
        );
        println!(
            "CPU Idle time: {}",
            idle_time
        );
        println!(
            "ThroughPut: {:.2} process/per_unit_time",
            (process.len() as f64)
                / ((process[process.len() - 1].completion_time - process[0].arrival_time) as f64)
        );

        process.sort_by_key(|process| process.pid);
        println!("");
        if !priority {
            println!("#P\tAT\tBT\tST\tCT\tTAT\tWT\tRT\n");
            for ele in process.iter() {
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
        } else {
            println!();
            println!("#P\tPri\tAT\tBT\tST\tCT\tTAT\tWT\tRT\n");

            for i in 0..process.len() {
                let process = &process[i];
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    process.pid,
                    process.priority,
                    process.arrival_time,
                    process.burst_time,
                    process.start_time,
                    process.completion_time,
                    process.turnaround_time,
                    process.waiting_time,
                    process.response_time
                );
            }
        }
    }
}
