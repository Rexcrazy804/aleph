//use std::fs::File;
//use std::io;
use std::process::Command;

fn main() {
    println!("Hello, Za WARUDO!");
    println!("I am totally running on {}", std::env::consts::OS);

    let output = Command::new("cmd")
        .args(["/C", "echo 'Hello World'"])
        .output()
        .expect("Failed to execute command");

    let Ok(text) = String::from_utf8(output.stdout) else {
        println!("Empty output");
        return;
    };

    println!("{text}");

    //let response = reqwest::blocking::get(
    //    "https://github.com/Clozure/ccl/releases/download/v1.12.2/ccl-1.12.2-windowsx86.zip",
    //)
    //.expect("failed to fetch url");
    //
    //let body = response.text().expect("No data");
    //
    //let mut out = File::create("rustup-init.sh").expect("failed to create file");
    //io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}
