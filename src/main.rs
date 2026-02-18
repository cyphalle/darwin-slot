mod config;
mod display;
mod git;

use clap::{Parser, Subcommand};
use dialoguer::Select;
use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::process::Command;

#[derive(Parser)]
#[command(name = "dslot", about = "Gérer les slots Darwin")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Libérer un slot (checkout develop + pull)
    Free {
        /// Nom du slot (ex: local-1, proto-2)
        slot: Option<String>,
    },
    /// Rejoindre un slot et lancer claude
    #[command(external_subcommand)]
    Go(Vec<OsString>),
}

fn main() {
    let cli = Cli::parse();
    let config = config::load_config();

    match cli.command {
        None => {
            go_to_slot(&config, None);
        }
        Some(Commands::Go(args)) => {
            let slot_name = args.into_iter().next().map(|s| s.into_string().unwrap());
            go_to_slot(&config, slot_name);
        }
        Some(Commands::Free { slot }) => {
            free_slot(&config, slot);
        }
    }
}

fn go_to_slot(config: &config::Config, slot_name: Option<String>) {
    let slot = match slot_name {
        Some(name) => {
            config
                .slots
                .iter()
                .find(|s| s.name == name)
                .unwrap_or_else(|| {
                    eprintln!("Slot '{}' introuvable", name);
                    std::process::exit(1);
                })
        }
        None => {
            let labels: Vec<String> = config
                .slots
                .iter()
                .map(|s| display::slot_label(s))
                .collect();

            let selection = Select::new()
                .with_prompt("Slot")
                .items(&labels)
                .default(0)
                .interact()
                .unwrap_or_else(|_| std::process::exit(0));

            &config.slots[selection]
        }
    };

    println!("→ {} ({})", slot.name, slot.path);

    let err = Command::new("claude")
        .arg("--dangerously-skip-permissions")
        .current_dir(&slot.path)
        .exec();
    eprintln!("Erreur au lancement de claude: {}", err);
    std::process::exit(1);
}

fn free_slot(config: &config::Config, slot_name: Option<String>) {
    let slot = match slot_name {
        Some(name) => {
            config
                .slots
                .iter()
                .find(|s| s.name == name)
                .unwrap_or_else(|| {
                    eprintln!("Slot '{}' introuvable", name);
                    std::process::exit(1);
                })
        }
        None => {
            let occupied: Vec<&config::SlotConfig> = config
                .slots
                .iter()
                .filter(|s| {
                    git::get_status(&s.path)
                        .map(|st| !st.is_available())
                        .unwrap_or(false)
                })
                .collect();

            if occupied.is_empty() {
                eprintln!("Tous les slots sont déjà libres");
                std::process::exit(0);
            }

            let labels: Vec<String> = occupied
                .iter()
                .map(|s| {
                    let status = git::get_status(&s.path).unwrap();
                    format!("{} ({}) — {}", s.name, s.slot_type, status.branch)
                })
                .collect();

            let selection = Select::new()
                .with_prompt("Quel slot libérer ?")
                .items(&labels)
                .default(0)
                .interact()
                .unwrap_or_else(|_| std::process::exit(0));

            occupied[selection]
        }
    };

    let status = git::get_status(&slot.path);
    if let Some(st) = &status {
        if !st.is_clean() {
            eprintln!(
                "⚠ {} n'est pas clean ({}). Commit ou stash d'abord.",
                slot.name,
                st.status_text()
            );
            std::process::exit(1);
        }
        if st.branch == "develop" {
            println!("{} est déjà sur develop", slot.name);
            return;
        }
    }

    println!("Libération de {} ...", slot.name);
    match git::checkout_develop(&slot.path) {
        Ok(()) => println!("✓ {} libéré (develop, à jour)", slot.name),
        Err(e) => {
            eprintln!("Erreur: {}", e);
            std::process::exit(1);
        }
    }
}
