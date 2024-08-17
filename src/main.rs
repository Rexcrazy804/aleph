fn main() {
    println!("Hello, Za WARUDO!");
    println!("I am totally running on {}", std::env::consts::OS);

    //match download_url("https://github.com/lukesampson/cowsay-psh/archive/master.zip") {
    //    Ok(()) => println!("Download succesfull"),
    //    Err(error) => println!("Download Failed with: {error}"),
    //};
    
    const URL: &str = "https://github.com/lukesampson/cowsay-psh/archive/master.zip";

    let mut response = reqwest::blocking::get(URL)
        .expect("Invalid Url");

    let content =  response.text()
        .expect("Failed to retrieve Body");

    let fname = "master.zip";
    let mut dest = std::fs::File::create(fname).expect("asd");

    std::io::copy(&mut content.as_bytes(), &mut dest);
}
