use bio::io::fasta;
use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("size")
        .about("Count total bases in FA file(s)")
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Sets the input file to use"),
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
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = reader(infile);
        let fa_in = fasta::Reader::new(reader);

        for result in fa_in.records() {
            // obtain record or fail with error
            let record = result.unwrap();

            writer.write_fmt(format_args!("{}\t{}\n", record.id(), record.seq().len()))?;
        }
    }

    Ok(())
}
