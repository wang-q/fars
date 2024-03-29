extern crate clap;
use clap::*;

mod cmd_fars;

fn main() -> anyhow::Result<()> {
    let app = Command::new("fars")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`fars` is a lightweight tool for operating sequences in the fasta format")
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(cmd_fars::size::make_subcommand())
        .subcommand(cmd_fars::some::make_subcommand())
        .subcommand(cmd_fars::one::make_subcommand())
        .subcommand(cmd_fars::region::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("size", sub_matches)) => cmd_fars::size::execute(sub_matches),
        Some(("some", sub_matches)) => cmd_fars::some::execute(sub_matches),
        Some(("one", sub_matches)) => cmd_fars::one::execute(sub_matches),
        Some(("region", sub_matches)) => cmd_fars::region::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}
