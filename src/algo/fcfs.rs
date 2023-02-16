use super::common::Process;

pub fn fcfs(process: &mut Vec<Process>) {
    process.sort_by(|val1, val2| val1.arrival_time.cmp(&val2.arrival_time));

    let mut last: Option<&Process> = None;

    for ele in process.iter_mut(){
        let prev = last.map(|prev| prev.completion_time).unwrap_or(0);
        ele.start_time = std::cmp::max(prev, ele.arrival_time);
        ele.completion_time = ele.start_time + ele.burst_time;
        ele.turnaround_time = ele.completion_time - ele.arrival_time;
        ele.waiting_time = ele.turnaround_time - ele.burst_time;
        ele.response_time = ele.start_time - ele.arrival_time;
        last = Some(ele);
    };
    Process::print_info(process,false);
}
