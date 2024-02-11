#[allow(unused)]

mod configparser;

fn main() {
    use crate::configparser::ConfigParser;
    let mut parser = ConfigParser::new().parse("config.txt");

    println!("{:?}", parser);
}