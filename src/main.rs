use core::panic;
use std::{env, fs::File, io::Write, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {}
        _ => panic!("only one arg accepted"),
    };

    let write_buff;

    match args[1].as_str() {
        "-i" => write_buff = b"1",
        "-a" => write_buff = b"0",
        _ => panic!("invalid arg"),
    };

    let path = Path::new("/sys/devices/platform/i8042/serio0/input/input2/inhibited");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!(
            "couldn't open {}: {}, make sure you run this program with sudo",
            display, why
        ),
        Ok(file) => file,
    };
    let result = file.write_all(write_buff);
    match result {
        Err(why) => panic!(
            "couldn't write {}: {}, make sure you run this program with sudo",
            display, why
        ),
        Ok(..) => println!(
            "Keyboard {}",
            if write_buff == b"0" {
                "activated"
            } else {
                "inhibited"
            }
        ),
    }
}
