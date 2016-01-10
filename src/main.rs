fn total_distance(speed: u32, sprint: u32, rest: u32, seconds: u32) -> u32 {
    let mut time_to_sprint = sprint;
    let mut time_to_rest = rest;
    let mut distance = 0;

    for _ in 0..seconds {
        if time_to_sprint > 0 {
            distance += speed;
            time_to_sprint -= 1;
            time_to_rest = rest;
        } else {
            if time_to_rest > 0 {
                time_to_rest -= 1;
            } else {
                time_to_sprint = sprint;
            }
        }
    }
    distance
}

fn main() {
    println!("Comet distance: {}", total_distance(14, 10, 127, 1000));
    println!("Dancer distance: {}", total_distance(16, 11, 162, 1000));
}
