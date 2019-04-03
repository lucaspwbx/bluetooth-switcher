//use std::process::{Command, Stdio};
//use std::error::Error;
//use std::io::prelude::*;
use std::fmt::Write as FmtWrite;
//static TESTE: &'static str = "grep bluez_card -B1 | grep index";
use pipers::Pipe;

fn main() {
    //Command::new("sh")
        //.arg("-c")
        //.arg("echo hello")
        //.spawn()
        //.expect("failed to execute process");

    //let output = Command::new("pacmd")
        //.arg("list-cards")
        //.output()
        //.expect("failed to execute process");

    //println!("status: {}", output.status);
    //io::stdout().write_all(&output.stdout).unwrap();
    //io::stdout().write_all(&output.stderr).unwrap();
    //assert!(output.status.success());
    //let process = match Command::new("pacmd")
                                 //.arg("list-cards")
                                 //.stdin(Stdio::piped())
                                 //.stdout(Stdio::piped())
                                 //.spawn() {
                                     //Err(why) => panic!("couldnt spawn: {}", why.description()),
                                     //Ok(process) => process,
                                 //};
    //match process.stdin.unwrap().write_all(TESTE.as_bytes()) {
        //Err(why) => panic!("foo: {}", why.description()),
        //Ok(_) => println!("sent foo to"),
    //}

    //let mut s = String::new();
    //match process.stdout.unwrap().read_to_string(&mut s) {
        //Err(why) => panic!("could read stdout: {}", why.description()),
        //Ok(_) => println!("responded with: \n{}", s),
    //};

 //   println!("{}", '\''.escape_default());
    let mut awk_cmd = String::new();
    awk_cmd.push_str("awk ");
    write!(awk_cmd, "{}", '\''.escape_default()).unwrap();
    awk_cmd.push_str("{print $2}");
    write!(awk_cmd, "{}", '\''.escape_default()).unwrap();
    println!("{}", awk_cmd);

   // let foo = r#"awk '{print $2}'"#;

    let out = Pipe::new("pacmd list-cards")
        .then("grep bluez_card -B1")
        .then("grep index")
 //       .then(&awk_cmd)
 //       .then(foo)
        .finally()
        .expect("Commands dit not pipe")
        .wait_with_output()
        .expect("failed to wait on child");

    println!("{}", &String::from_utf8(out.stdout).unwrap());
}
