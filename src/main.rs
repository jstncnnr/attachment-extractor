use clap::Parser;
use mail_parser::{mailbox::mbox::MessageIterator, Message, MimeHeaders};
use std::{fs::File, path::Path};

#[derive(Parser, Default, Debug)]
#[command(name = "Justin Conner", version = "1.0.0", about = "A simple program to take in an MBOX mailbox and extract all of the attachments", long_about = None)]
struct Arguments {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Arguments::parse();

    let outdir = Path::new(&args.output);
    if !outdir.is_dir() {
        panic!("Output is not set to a directory")
    }

    let path = Path::new(&args.input);
    if path.is_dir() {
        panic!("Input must be set to a single file")
    }

    match path.extension() {
        Some(ext) => {
            if ext.eq_ignore_ascii_case("mbox") {
                Some(ext)
            } else {
                None
            }
        }
        None => None,
    }
    .expect("Input file must end in .mbox");

    let input = match File::open(&args.input) {
        Ok(file) => file,
        Err(_) => panic!("Unable to open input file"),
    };

    for raw_message in MessageIterator::new(input) {
        let raw_message = match raw_message {
            Ok(message) => message,
            Err(_) => continue,
        };

        let message = match Message::parse(raw_message.contents()) {
            Some(message) => message,
            None => continue,
        };

        write_attachments(&message, &outdir);
    }
}

fn write_attachments(message: &Message, output: &Path) {
    for attachment in message.attachments() {
        if !attachment.is_message() {
            let path = output.join(attachment.attachment_name().unwrap_or("Untitled"));
            std::fs::write(path.as_path(), attachment.contents()).unwrap();
        } else {
            write_attachments(attachment.message().unwrap(), output);
        }
    }
}
