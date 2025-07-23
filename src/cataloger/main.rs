// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-maildir-cataloger requires the 'std' feature");

use asimov_maildir_module::MaildirReader;
use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use dogma::{Uri, UriScheme::File, UriValueParser};
use std::{error::Error, io::stdout};

/// asimov-maildir-cataloger
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// Limit the number of messages to catalog.
    #[arg(value_name = "COUNT", short = 'n', long)]
    limit: Option<usize>,

    /// Set the output format [default: cli] [possible values: cli, json, jsonld, jsonl]
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    /// A `file:/path/to/maildir/` URL to the folder to catalog.
    #[arg(value_name = "MAILDIR-FOLDER-URL", value_parser = UriValueParser::new(&[File]))]
    maildir_url: Uri<'static>,
}

fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    // Open the maildir directory:
    let maildir = MaildirReader::open(options.maildir_url.path())?;

    // Scan the maildir messages:
    let messages = maildir.iter();
    let messages = messages.take(options.limit.unwrap_or(usize::MAX));
    match options
        .output
        .as_ref()
        .unwrap_or(&String::default())
        .as_str()
    {
        "jsonl" => {
            use know::traits::ToJsonLd;
            for message in messages {
                let message = message?;
                let json = message.headers.to_jsonld()?;
                serde_json::to_writer(stdout(), &json)?;
                println!();
            }
        },
        "jsonld" | "json" => {
            use know::traits::ToJsonLd;
            let mut output = Vec::new();
            for message in messages {
                let message = message?;
                output.push(message.headers.to_jsonld()?);
            }
            if cfg!(feature = "pretty") {
                colored_json::write_colored_json(&output, &mut stdout())?;
            } else {
                serde_json::to_writer_pretty(stdout(), &output)?;
            }
            println!();
        },
        "cli" | _ => {
            for (index, message) in messages.enumerate() {
                let message = message?;
                if index > 0 && options.flags.verbose > 0 {
                    println!();
                }
                match options.flags.verbose {
                    0 => print!("{}", message.headers.oneliner()),
                    1 => print!("{}", message.headers.concise()),
                    _ => print!("{}", message.headers.detailed()),
                }
            }
        },
    }

    Ok(EX_OK)
}
