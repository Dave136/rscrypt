use clap::{App, Arg};
use rpassword::read_password;

fn main() {
    let matches = App::new("rcrypt")
        .version("0.0.1")
        .author("David A. <davejs136@gmail.com>")
        .about("Encrypt and decrypt files with credentials")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE_PATH")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("encrypt")
                .short("e")
                .long("encrypt")
                .value_name("FILE_TO_ENCRYPT")
                .help("Encrypt specified files")
                .takes_value(true),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap_or("null");
    println!("Config value: {}", config);

    if matches.is_present("encrypt") {
        let target = matches.value_of("encrypt").unwrap();
        println!("Set a password: ");
        let password = read_password().unwrap();
        println!("Encrypting: {}, with password: {}", target, password);
    }
    std::process::exit(0);
}
