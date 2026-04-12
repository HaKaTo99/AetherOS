//! AetherOS Resource Packager (pkgar)
//! Tactical PQC Binary Signer for Sovereign Deployment.

use std::fs::{File, read, write};
use std::io::{Write, Read};
use hmac::{Hmac, Mac};
use sha2::Sha512;
use clap::{Parser, Subcommand};

type HmacSha512 = Hmac<Sha512>;

#[derive(Parser)]
#[command(name = "pkgar")]
#[command(about = "AetherOS Resource Packager & PQC Signer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sign a binary and package it as .arm
    Sign {
        /// Input binary file
        input: String,
        /// Output .arm file
        output: String,
        /// Secret key for HMAC-SHA512 (Tactical PQC Baseline)
        #[arg(short, long)]
        key: String,
    },
    /// Verify an .arm package
    Verify {
        /// .arm file to verify
        input: String,
        /// Public key for HMAC-SHA512 (Tactical PQC Baseline)
        #[arg(short, long)]
        key: String,
    },
}

const MAGIC: &[u8; 8] = b"AETHEROS";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sign { input, output, key } => {
            let payload = read(input)?;
            let key_bytes = key.as_bytes();
            
            let mut mac = HmacSha512::new_from_slice(key_bytes)?;
            mac.update(&payload);
            let signature = mac.finalize().into_bytes();

            let mut out = File::create(output)?;
            out.write_all(MAGIC)?;
            out.write_all(&(payload.len() as u64).to_le_bytes())?;
            out.write_all(&signature)?;
            out.write_all(&payload)?;

            println!("[Sovereign] Binary signed successfully: {}", output);
        }
        Commands::Verify { input, key } => {
            let mut file = File::open(input)?;
            let mut magic = [0u8; 8];
            file.read_exact(&mut magic)?;
            if &magic != MAGIC {
                return Err("Invalid file format: Magic mismatch".into());
            }

            let mut size_bytes = [0u8; 8];
            file.read_exact(&mut size_bytes)?;
            let size = u64::from_le_bytes(size_bytes) as usize;

            let mut signature = [0u8; 64];
            file.read_exact(&mut signature)?;

            let mut payload = vec![0u8; size];
            file.read_exact(&mut payload)?;

            let mut mac = HmacSha512::new_from_slice(key.as_bytes())?;
            mac.update(&payload);
            
            if mac.verify_slice(&signature).is_ok() {
                println!("[Sovereign] Signature VERIFIED: Integrasi biner paska-kuantum terjamin.");
            } else {
                println!("[CRITICAL] SIGNATURE VIOLATION: Biner tidak sah atau telah dimodifikasi!");
            }
        }
    }

    Ok(())
}
