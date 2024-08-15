use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

#[derive(Parser)]
#[command(version, about)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Specify arrays of characters ARRAY1 that control the action
    string1: String,
    /// Specify arrays of characters ARRAY2 that control the action
    string2: Option<String>,

    /// Use the complement of ARRAY1
    #[arg(short, long)]
    complement: bool,

    /// Delete characters in ARRAY1, do not translate
    #[arg(short, long)]
    delete: bool,

    /// Replace each sequence of a repeated character that is listed in the last
    /// specified ARRAY, with a single occurrence of that character
    #[arg(short, long)]
    squeeze_repeats: bool,

    /// First truncate ARRAY1 to length of ARRAY2
    #[arg(short, long)]
    truncate_set1: bool,
}

fn main() {
    let cli = Cli::parse();
    if cli.string2.is_none() && !cli.delete && !cli.squeeze_repeats {
        let mut cmd = Cli::command();
        cmd.error(
            ErrorKind::MissingRequiredArgument,
            "STRING2 is required argument unless -d/--delete or -s/--squeeze-repeats are set",
        )
        .exit()
    }
}
