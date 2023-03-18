pub fn command_uptime_parser() -> String {
    let cmd = String::from_utf8(std::process::Command::new("uptime").output().unwrap().stdout).unwrap();
    let clock = cmd.split(',').collect::<Vec<&str>>()[0].split_whitespace().collect::<Vec<&str>>()[2];
    let mut clockc = clock.clone().to_string();
    clockc.retain(|c| c == ':');
    let _clockv = clock.clone().split(':').collect::<Vec<&str>>();
    let mut clockv: Vec<&str> = Vec::with_capacity(3);
    for v in _clockv {
        if v.clone().chars().next().unwrap() == '0' {
            let v = v.split("").collect::<Vec<&str>>();
            clockv.push(v[2]);
            continue;
        }   clockv.push(v);
    }
    let clock: String;
    match clockc.as_str().chars().count() {
        0 => {clock = format!("{} {}", clockv[0], "minute(s)");}
        1 => {clock = format!("{} {} {} {}", clockv[0], "hour(s)", clockv[1], "minute(s)");}
        2 => {clock = format!("{} {} {} {} {} {}", clockv[0], "day(s)", clockv[1], "hour(s)", clockv[2], "minute(s)");}
        _   => panic!("more than 2 :")
    }   clock
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        assert!(super::command_uptime_parser().contains("minute(s)"));
    }
}
