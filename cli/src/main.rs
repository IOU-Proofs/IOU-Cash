use clap::{Args, Parser, Subcommand};
use core::str;
use files::{Creds, FileMan};

pub(crate) mod files;

// pub(crate) type Field = <Concrete as IVC>::Field;
// pub(crate) type VerifyingKey = <Concrete as IVC>::Field;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create(CreateArgs),
    Register(RegisterArgs),
    GetUser(GetUserArgs),
    GetNotes(GetUserArgs),
    VerifyNullifier(VerifyNullifierArgs),
    Info,
    Reset,
    RegisterAuto,
}

#[derive(Args)]
struct GetUserArgs {
    username: String,
}

#[derive(Args)]
struct VerifyNullifierArgs {
    nullifier: String,
    state: String,
}

#[derive(Args)]
struct CreateArgs {
    #[arg(short, long = "user")]
    username: String,
    #[arg(short, long, default_value = "")]
    pass: String,
}
#[derive(Args)]
struct RegisterArgs {
    #[arg(short, long = "user")]
    username: String,
    #[arg(short, long)]
    address: String,
    #[arg(short, long)]
    pubkey: String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create(args) => Creds::generate(args).unwrap(),
        Commands::Info => FileMan::list_accounts(),
        Commands::RegisterAuto => Creds::register_auto().unwrap(),
        // TODO: ISSUE & TRANSFER NOTE includes IVCNOTES & SERVICE
        // TODO: SPLIT & TRANSFER NOTE includes IVCNOTES & SERVICE
        // SPLIT TODO: store nullifier
        Commands::GetUser(args) => {
            if let Err(e) = Creds::get_user(args.username.clone()) {
                eprintln!("Failed to get user: {:?}", e);
            }
        }
        Commands::GetNotes(args) => {
            if let Err(e) = Creds::get_notes(args.username.clone()) {
                eprintln!("Failed to get notes: {:?}", e);
            }
        }
        Commands::VerifyNullifier(args) => {
            if let Err(e) = Creds::verify_nullifier(args.nullifier.clone(), args.state.clone()) {
                eprintln!("Failed to verify Nullifier: {:?}", e);
            }
        }
        Commands::Register(args) => {
            if let Err(e) = Creds::register(args.username.clone(), args.address.clone()) {
                eprintln!("Failed to register user: {:?}", e);
            }
        }
        Commands::Reset => FileMan::clear_contents().unwrap(),
    }
}
