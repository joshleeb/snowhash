use clap::{App, Arg};

pub fn create<'a, 'b>() -> App<'a, 'b> {
    App::new("Snowhash")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Procedurally generate a unique snowflake from a hash")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Output path for the generated PNG")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("HASH")
                .help("Hash to use for generating the snowflake")
                .required(true)
                .index(1),
        )
}
