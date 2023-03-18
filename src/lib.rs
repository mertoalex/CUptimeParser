pub fn command_uptime_parser() -> String {
    let cmd = String::from_utf8(std::process::Command::new("uptime").output().unwrap().stdout).unwrap();
    let saat = cmd.split(',').collect::<Vec<&str>>()[0].split_whitespace().collect::<Vec<&str>>()[2];
    let mut saatc = saat.clone().to_string();
    saatc.retain(|c| c == ':');
    let _saatv = saat.clone().split(':').collect::<Vec<&str>>();
    let mut saatv: Vec<&str> = Vec::with_capacity(3);
    for v in _saatv {
        if v.clone().chars().next().unwrap() == '0' {
            let v = v.split("").collect::<Vec<&str>>();
            saatv.push(v[2]);
            continue;
        }   saatv.push(v);
    }
    let saat: String;
    match saatc.as_str().chars().count() {
        0 => {saat = format!("{} {}", saatv[0], "minute(s)");}
        1 => {saat = format!("{} {} {} {}", saatv[0], "hour(s)", saatv[1], "minute(s)");}
        2 => {saat = format!("{} {} {} {} {} {}", saatv[0], "day(s)", saatv[1], "hour(s)", saatv[2], "minute(s)");}
        _   => panic!("more than 2 :")
    }   saat
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        assert!(super::command_uptime_parser().contains("minute(s)"));
    }
}
