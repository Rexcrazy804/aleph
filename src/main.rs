use std::process::Command;

fn main() {
    //println!("Hello, Za WARUDO!");
    //println!("I am totally running on {}", std::env::consts::OS);

    let output = Command::new("pwsh")
        .args(["-c", "echo 'Hello World'"])
        .output()
        .expect("Failed to execute command");

    let Ok(text) = String::from_utf8(output.stdout) else {
        println!("Empty output");
        return;
    };

    println!("{text}");
}
