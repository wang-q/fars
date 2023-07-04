use bio::io::fasta;
use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("region")
        .about("Extract regions from a FA file")
        .after_help(
            r###"
* region.txt contains fake runlists
  There might be overlaps, e.g. I:17221-25234,21428-25459

"###,
        )
        .arg(
            Arg::new("infile")
                .required(true)
                .num_args(1)
                .index(1)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("region.txt")
                .required(false)
                .num_args(1)
                .index(2)
                .help("seq_name:begin-end[,begin-end]"),
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

    let mut n = 0;
    let mut sum = 0;
    for result in fa_in.records() {
        // obtain record or fail with error
        let record = result.unwrap();
        // obtain sequence
        let seq = record.seq();
        sum += seq.len();
        n += 1;
    }
    println!(
        "mean sequence length of {} records: {:.1} bp",
        n,
        sum as f32 / n as f32
    );

    Ok(())
}
