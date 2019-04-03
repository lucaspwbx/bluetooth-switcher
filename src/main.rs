use pipers::Pipe;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let out = Pipe::new("pacmd list-cards")
        .then("grep bluez_card -B1")
        .then("grep index")
        .finally()
        .expect("Commands dit not pipe")
        .wait_with_output()
        .expect("failed to wait on child");

    let output = &String::from_utf8(out.stdout).unwrap();
    let mut indexes: Vec<u32> = vec![];

    let re = Regex::new(r"\d+").unwrap();

    for cap in re.captures_iter(output) {
        println!("{}", &cap[0]);
        indexes.push(FromStr::from_str(&cap[0]).unwrap());
    }
}
