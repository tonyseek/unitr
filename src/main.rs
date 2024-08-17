use std::io::Read;

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
    let mut cmd = Cli::command();

    if cli.string2.is_none() && !cli.delete && !cli.squeeze_repeats {
        cmd.error(
            ErrorKind::MissingRequiredArgument,
            "STRING2 is required argument unless -d/--delete or -s/--squeeze-repeats are set",
        )
        .exit()
    }

    let array1: Vec<char> = cli.string1.chars().collect();
    let array2: Vec<char> = cli.string2.unwrap_or_default().chars().collect();

    if cli.truncate_set1 {
        todo!()
    }

    // TODO: Process stdin as a stream
    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        cmd.error(ErrorKind::InvalidUtf8, "STDIN cannot be decoded in UTF-8")
            .exit()
    }

    let mut squeeze_char: Option<char> = None;

    for char in input.chars() {
        if cli.delete && array1.contains(&char) {
            continue;
        }
        if cli.squeeze_repeats && array1.contains(&char) {
            if squeeze_char == Some(char) {
                continue;
            }
            squeeze_char = Some(char);
        } else {
            squeeze_char = None;
        }

        if cli.complement {
            let new_char = if array1.contains(&char) {
                &char
            } else {
                array2.last().unwrap_or(&char)
            };
            print!("{new_char}");
        } else {
            let new_char = if let Some(index) = array1.iter().position(|&c| c == char) {
                array2.get(index).unwrap_or(array2.last().unwrap_or(&char))
            } else {
                &char
            };
            print!("{new_char}");
        }
    }
}
