macro_rules! assert {
    ($x:expr) => {
        if !$x {
            panic!("Assertion failed: {}", stringify!($x));
        }
    };
}

pub mod inifile;

pub mod parsing;
use parsing::*;

fn main() {
    let filename = std::env::args().nth(1).expect("no filename given to parse");
    let mut file = parse_ini_file(filename).unwrap();
    println!("{:#?}", file);

    file.get_section("<default>")
        .unwrap()
        .create_entry("from program", "has value");
    file.get_section("<default>")
        .unwrap()
        .create_entry("from program2", "has value2");

    let (section, value) = file.peek("from program").unwrap();

    println!(
        "Value for 'from program' = {} in section {}",
        value, section.name
    );

    match file.peek_value("a setting that does not exist") {
        Some(value) => println!("Value for 'a setting that does not exist' = {}", value),
        None => println!("No value for 'a setting that does not exist'"),
    }

    file.write_to_file("output.ini", true).unwrap();
}
