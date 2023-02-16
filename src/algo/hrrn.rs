use super::common::Process;

// Sort according to response-ratio
pub fn hrrn(process: &mut Vec<Process>){
    process.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));

    let mut last_completion = None;

    for i in 0..process.len() {
        last_completion = Some({
            let prev_completion = last_completion.unwrap_or(0);
            let p = &mut process[i];

            p.start_time = prev_completion.max(p.arrival_time);
            p.completion_time = p.start_time + p.burst_time;
            p.turnaround_time = p.completion_time - p.arrival_time;
            p.waiting_time = p.turnaround_time - p.burst_time;
            p.response_time = p.start_time - p.arrival_time;
            p.completion_time
        });

        let count = process.iter().skip(i+1).take_while(|cp| cp.arrival_time <= last_completion.unwrap()).count();
        process[i+1..i+1+count].sort_by(|a, b| {
            let response_ratio_a = ((last_completion.unwrap() - a.arrival_time) + a.burst_time) / a.burst_time;
            let response_ratio_b = ((last_completion.unwrap() - b.arrival_time) + b.burst_time) / b.burst_time;
            response_ratio_b.cmp(&response_ratio_a)
        });
    }
   Process::print_info(process,false);
}
