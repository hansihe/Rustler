use rustler::schedule::Schedule;

#[rustler::nif]
fn scheduled_fac(input: u32, result: Option<u32>) -> Schedule<scheduled_fac, u32, u32, u32> {
    let result = result.unwrap_or(1);
    if input == 0 {
        Schedule::Result(result)
    } else {
        (input - 1, result * input).into()
    }
}
