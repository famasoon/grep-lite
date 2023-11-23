use clap::{App, Arg};
use regex::Regex;

fn main() {
    let args = App::new("grep-lite")
        .version("1.0.0")
        .about("Search for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let quote = "\
    Every face,
    every shop,
    bedroom window,
    public-house,
    and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books.
    What do we seek through millions of pages?";

    for line in quote.lines() {
        let containes_substring = re.find(line);
        match containes_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
