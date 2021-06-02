use rustler::{reschedule, Schedule, SchedulerFlags};

#[rustler::nif]
fn scheduled_fac<'a>(
    input: u8,
    result: Option<u32>,
) -> Schedule<scheduled_fac, u32, u8, Option<u32>> {
    let result = result.unwrap_or(1);
    if input == 0 {
        Schedule::Return(result)
    } else {
        reschedule!(
            SchedulerFlags::Normal,
            input - 1,
            Some(result * input as u32)
        )
    }
}
