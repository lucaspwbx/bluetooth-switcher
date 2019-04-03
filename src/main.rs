use pipers::Pipe;
use regex::Regex;
use std::str::FromStr;
use std::process::Command;

fn main() {
 //   let bla = "pacmd set-card-profile $index off";
   // let bla2 = "pacmd set-card-profile $index a2dp_sink_ldac";
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
    let _foo = format!("{} off", indexes[0]);
    let foo2 = format!("{} a2dp_sink_ldac", indexes[0]);
    Command::new("pacmd")
        .args(&["set-card-profile", &foo2])
        .spawn()
        .expect("failed");
}
