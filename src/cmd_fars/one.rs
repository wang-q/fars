use bio::io::fasta;
use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("one")
        .about("Extract one FA record")
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("name")
                .help("The name of the wanted record")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .forbid_empty_values(true)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let reader = reader(args.value_of("infile").unwrap());
    let fa_in = fasta::Reader::new(reader);

    let writer = writer(args.value_of("outfile").unwrap());
    let mut fa_out = fasta::Writer::new(writer);

    let name = args.value_of("name").unwrap();

    for result in fa_in.records() {
        // obtain record or fail with error
        let record = result.unwrap();

        if record.id() == name {
            fa_out.write_record(&record).expect("Error writing record.");
            break;
        }
    }

    Ok(())
}
