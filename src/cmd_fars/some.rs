use bio::io::fasta;
use clap::*;
use intspan::*;
use std::collections::HashSet;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("some")
        .about("Extract some FA records")
        .arg(
            Arg::new("infile")
                .required(true)
                .index(1)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("lst")
                .required(true)
                .index(2)
                .help("One name per line"),
        )
        .arg(
            Arg::new("invert")
                .long("invert")
                .short('i')
                .action(ArgAction::SetTrue)
                .help("Output sequences not in the list"),
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
    let is_invert = args.get_flag("invert");

    let reader = reader(args.get_one::<String>("infile").unwrap());
    let fa_in = fasta::Reader::new(reader);

    let writer = writer(args.get_one::<String>("outfile").unwrap());
    let mut fa_out = fasta::Writer::new(writer);

    let set_lst: HashSet<String> = read_first_column(args.get_one::<String>("lst").unwrap())
        .into_iter()
        .collect();

    for result in fa_in.records() {
        // obtain record or fail with error
        let record = result.unwrap();

        if set_lst.contains(record.id()) != is_invert {
            fa_out.write_record(&record).expect("Error writing record.");
        }
    }

    Ok(())
}
