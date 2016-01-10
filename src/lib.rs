pub fn total_distance(speed: u32, fly: u32, rest: u32, seconds: u32) -> u32 {
    let mut time_to_fly = fly;
    let mut time_to_rest = rest;
    let mut distance = 0;

    for _ in 0..seconds {
        if time_to_fly > 0 {
            distance += speed;
            time_to_fly -= 1;
            time_to_rest = rest;
        } else {
            if time_to_rest > 0 {
                time_to_rest -= 1;
            } else {
                time_to_fly = fly;
            }
        }
    }
    distance
}

pub fn parse_string(s: &String) -> (u32, u32, u32) {
    let can_fly = s.find("can fly").expect("Uh oh!") + 8;
    let mut km_s = s.find("km/s").expect("Uh oh!") - 1;
    let speed = s[can_fly..km_s].parse::<u32>().expect("Error parsing speed");

    km_s = s.find("km/s").expect("Uh oh!") + 9;
    let first_seconds = s.find("seconds,").expect("Uh oh!") - 1;
    let fly = s[km_s..first_seconds].parse::<u32>().expect("Error parsing fly time");

    let rest_for = s.find("rest for").expect("Uh oh!") + 9;
    let second_seconds = s.find("seconds.").expect("Uh oh!") - 1;
    let rest = s[rest_for..second_seconds].parse::<u32>().expect("Error parsing rest time");

    (speed, fly, rest)
}

#[test]
fn comet_distance() {
    assert_eq!(1120, total_distance(14, 10, 127, 1000));
}

#[test]
fn dancer_distance() {
    assert_eq!(1056, total_distance(16, 11, 162, 1000));
}

#[test]
fn comet_parse() {
    let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.".to_string();
    let (speed, fly, rest) = parse_string(&input);
    assert_eq!(14, speed);
    assert_eq!(10, fly);
    assert_eq!(127, rest);
}

#[test]
fn dancer_parse() {
    let input = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.".to_string();
    let (speed, fly, rest) = parse_string(&input);
    assert_eq!(16, speed);
    assert_eq!(11, fly);
    assert_eq!(162, rest);
}
