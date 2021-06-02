use rustler::{reschedule, Schedule, SchedulerFlags};

#[rustler::nif]
fn scheduled_fac<'a>(input: u32, result: Option<u32>) -> Schedule<scheduled_fac, u32, u32, u32> {
    let result = result.unwrap_or(1);
    if input == 0 {
        Schedule::Return(result)
    } else {
        reschedule!(SchedulerFlags::Normal, input - 1, result * input)
    }
}
