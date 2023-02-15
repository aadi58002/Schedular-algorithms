use super::common::Process;

pub fn fcfs(process: &mut Vec<Process>) {
    process.sort_by(|v1, v2| v1.arrival_time.cmp(&v2.arrival_time));
    let (mut idle_time, mut time) = (process[0].arrival_time, process[0].arrival_time);
    for index in 0..process.len() {
        if process[index].arrival_time > time {
            time += process[index].arrival_time - time;
        }
        process[index].start_time = time;
        if index > 0 {
            if let Some(val) = process[index]
                .start_time
                .checked_sub(process[index - 1].completion_time)
            {
                idle_time += val;
            }
        }
        if let Some(val) = process[index]
            .start_time
            .checked_sub(process[index].arrival_time)
        {
            process[index].waiting_time = val;
        }
        process[index].completion_time = process[index].start_time + process[index].burst_time;
        if let Some(val) = process[index]
            .completion_time
            .checked_sub(process[index].arrival_time)
        {
            process[index].turnaround_time = val;
        };
        if let Some(val) = process[index]
            .start_time
            .checked_sub(process[index].arrival_time)
        {
            process[index].response_time = val;
        };
        time += process[index].burst_time;
    }
    process.sort_by_key(|process| process.pid);
    Process::print_table(process.to_vec());
    Process::print_info(process.to_vec());
    println!("Idle CPU time: {idle_time}");
    println!("CPU time: {:.2}%",((time-idle_time) as f64/time as f64)*100f64);
    Process::print_throughput(process.len(), time);
}
