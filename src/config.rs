use clap::{App, Arg};

pub struct Config {
    pub input_file_path: String,
    pub output_file_path: String,
    pub is_simple: bool,
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("Movie Barcode")
                        .version("1.0")
                        .author("William Main <william.c.main@gmail.com>")
                        .about("Creates movie barcodes from video files")
                        .arg(Arg::with_name("input_path")
                            .short("i")
                            .long("input_path")
                            .required(true)
                            .help("Sets the input file")
                            .takes_value(true))
                        .arg(Arg::with_name("output_path")
                            .short("o")
                            .long("output_path")
                            .required(true)
                            .help("Sets the output file")
                            .takes_value(true))
                        .arg(Arg::with_name("is_simple")
                            .short("s")
                            .long("simple")
                            .help("Sets barcode mode. Simple creates barcodes where frame columns have one color averaged from the frame"))
                        .get_matches();

        let input_file_path = matches.value_of("input_path").unwrap().to_owned();

        let output_file_path = matches.value_of("output_path").unwrap().to_owned();

        let is_simple = matches.is_present("is_simple");

        Self {
            input_file_path,
            is_simple,
            output_file_path,
        }
    }
}
