use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "gear", about = "A developer QOL tool")]
enum Gear {
    #[structopt(name = "run", about = "Run user-defined commands")]
    Run {
        #[structopt(help = "Name of command to run")]
        command: String,
        #[structopt(help = "Optional arguments to pass to the command")]
        args: Vec<String>,
    },
}

fn main() {
    let program = Gear::from_args();
    match program {
        Gear::Run { command, args } => println!("Running {} with args {:?}", command, args),
    }
}
