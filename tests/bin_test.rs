use aleph::manifest::{bin::Binary, Manifest};
use std::{fmt::Binary as FUCKOFF, fs};

#[test]
fn apngasm_bin() {
    let data = fs::read_to_string("./tests/sample_data/apngasm.json")
        .expect("Failed to retreive sample data");
    let data: Manifest = serde_json::from_str(&data).expect("Failed to parse data\n");

    //"bin": [
    //    "bin\\apngasm.exe",
    //    [
    //        "bin\\apngasm.exe",
    //        "apngasm-cli"
    //        "--arg1",
    //        "--arg2",
    //        "--arg3"
    //    ]
    //]
    assert_eq!(
        Binary::AliasedExecutables(vec![
            Binary::Executable(String::from("bin\\apngasm.exe")),
            Binary::Executables(vec![
                String::from("bin\\apngasm.exe"),
                String::from("apngasm-cli"),
                String::from("--arg1"),
                String::from("--arg2"),
                String::from("--arg3")
            ]),
        ]),
        data.bin.clone().unwrap()
    );

    let data_bin = data.bin.unwrap();

    if let Binary::AliasedExecutables(bins) = data_bin {
        for bin in bins {
            if let Binary::Executables(shim) = bin {
                println!(
                    "command: {}\nalias: {}\nargs: {}",
                    shim[0],
                    shim[1],
                    shim.iter()
                        .skip(2)
                        .fold(String::new(), |acc, x| acc + &x + " ")
                );
            }
        }
    }
}
