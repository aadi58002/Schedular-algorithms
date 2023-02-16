use super::common::Process;

// Longest burst time first
pub fn ljf(process: &mut Vec<Process>){
    process.sort_by(|val1, val2| val1.arrival_time.cmp(&val2.arrival_time));

    let mut last = None;

    for i in 0..process.len() {
        last = Some({
            let prev = last.unwrap_or(0);
            let current_process = &mut process[i];

            current_process.start_time = prev.max(current_process.arrival_time);
            current_process.completion_time = current_process.start_time + current_process.burst_time;
            current_process.turnaround_time = current_process.completion_time - current_process.arrival_time;
            current_process.waiting_time = current_process.turnaround_time - current_process.burst_time;
            current_process.response_time = current_process.start_time - current_process.arrival_time;
            current_process.completion_time
        });

        let count = process
            .iter()
            .skip(i + 1)
            .take_while(|cp| cp.arrival_time <= last.unwrap())
            .count();
        process[i + 1..i + 1 + count].sort_by(|a, b| b.burst_time.cmp(&a.burst_time));
    }
    Process::print_info(process,false);
}
