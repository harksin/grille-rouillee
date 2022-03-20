mod utils;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "equilibrium")]
#[clap(bin_name = "equilibrium")]
struct Equilibrium {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::ArgEnum, Clone, Debug)]
pub enum Mode {
    Spread,
    Size,
}
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Balance {
        /// kafka broker
        #[clap(short, long, required = true)]
        bootstrap_server: String,

        /// is balance incremental ?
        #[clap(arg_enum, short, long, required = true)]
        mode: Mode,

        /// is balance incremental ?
        #[clap(short, long,takes_value = false)]
        incremental: bool,

        /// dry run with consequence, ask to continue
        #[clap(short,long,takes_value = false)]
        plan: bool,
    },
    #[clap(arg_required_else_help = true)]
    Supervise {
        /// kafka broker
        #[clap(short, long, required = true)]
        bootstrap_server: String,

        /// is supervisor move incremental ?
        #[clap(short, long,takes_value = false)]
        incremental: bool,
    },
    #[clap(arg_required_else_help = true)]
    DecommissionBroker {
        /// kafka broker
        #[clap(short, long, required = true)]
        bootstrap_server: String,

        /// is supervisor move incremental ?
        #[clap(short, long, takes_value = false)]
        incremental: bool,

        /// dry run with consequence, ask to continue
        #[clap(short,long,takes_value = false)]
        plan: bool,
    },
}

fn main() {
    let args = Equilibrium::parse();

    match &args.command {
        Commands::Balance {
            bootstrap_server,
            mode,
            incremental,
            plan,
        } => {
            println!("Balance Started\n bootstrap-server [{}]\n mode : [{:?}]\n incremental : [{}]\n plan : [{}]", bootstrap_server,mode,incremental,plan);

            //todo
        }
        Commands::Supervise {
            bootstrap_server,
            incremental,
        } => {
            println!(
                "Supervision Started\n bootstrap-server [{}]\n incremental : [{}]",
                bootstrap_server, incremental
            );

            //todo
        }
        Commands::DecommissionBroker {
            bootstrap_server,
            incremental,
            plan,
        } => {
            println!(
                "Decommission Started\n bootstrap-server [{}]\n incremental : [{}]\n plan : [{}]",
                bootstrap_server, incremental, plan
            );

            //todo
        }
    }
}
