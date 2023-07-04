use bio::io::fasta;
use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("one")
        .about("Extract one FA record")
        .arg(
            Arg::new("infile")
                .required(true)
                .num_args(1)
                .index(1)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("name")
                .required(true)
                .num_args(1)
                .index(2)
                .help("The name of the wanted record"),
        )
        .arg(
            Arg::new("outfile")
                .long("outfile")
                .short('o')
                .num_args(1)
                .default_value("stdout")
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let reader = reader(args.get_one::<String>("infile").unwrap());
    let fa_in = fasta::Reader::new(reader);

    let writer = writer(args.get_one::<String>("outfile").unwrap());
    let mut fa_out = fasta::Writer::new(writer);

    let name = args.get_one::<String>("name").unwrap();

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
