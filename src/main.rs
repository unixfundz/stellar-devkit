// Copyright 2025 stellar-devkit contributors
// SPDX-License-Identifier: Apache-2.0

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "stellar-devkit",
    about = "A local Stellar/Soroban developer toolkit",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Simulate a Soroban contract invocation locally
    Simulate {
        #[arg(long)]
        wasm: String,
        #[arg(long)]
        function: String,
        #[arg(long, default_value = "[]")]
        args: String,
        #[arg(long)]
        ledger: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Decode and display diagnostic events from base64 XDR
    Events {
        #[arg(long)]
        tx: String,
    },
    /// Inspect a WASM contract file
    Inspect {
        #[arg(long)]
        wasm: String,
    },
    /// Capture ledger state for a contract to a JSON snapshot
    Snapshot {
        #[arg(long)]
        rpc: String,
        #[arg(long)]
        contract: String,
        #[arg(long, default_value = "./snapshot.json")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Simulate {
            wasm,
            function,
            args,
            ledger,
            json,
        } => {
            println!("Simulating {} in {}", function, wasm);
            let _ = (args, ledger, json);
        }
        Commands::Events { tx } => {
            println!("Decoding events from tx: {}", tx);
        }
        Commands::Inspect { wasm } => {
            println!("Inspecting WASM: {}", wasm);
        }
        Commands::Snapshot {
            rpc,
            contract,
            output,
        } => {
            println!(
                "Snapshotting contract {} from {} to {}",
                contract, rpc, output
            );
        }
    }
}
