use colored::Colorize;
use tabled::{Table, Tabled};

use crate::config::SlotConfig;
use crate::git;

#[derive(Tabled)]
struct SlotRow {
    #[tabled(rename = "Slot")]
    name: String,
    #[tabled(rename = "Type")]
    slot_type: String,
    #[tabled(rename = "Branche")]
    branch: String,
    #[tabled(rename = "Git")]
    git_status: String,
    #[tabled(rename = "Dispo")]
    availability: String,
}

pub fn print_status(slots: &[SlotConfig]) {
    let rows: Vec<SlotRow> = slots
        .iter()
        .map(|slot| {
            if let Some(status) = git::get_status(&slot.path) {
                let availability = if status.is_available() {
                    "libre".green().to_string()
                } else {
                    "occupé".red().to_string()
                };

                let git_text = if status.is_clean() {
                    "clean".green().to_string()
                } else {
                    status.status_text().yellow().to_string()
                };

                SlotRow {
                    name: slot.name.clone(),
                    slot_type: slot.slot_type.clone(),
                    branch: status.branch,
                    git_status: git_text,
                    availability,
                }
            } else {
                SlotRow {
                    name: slot.name.clone(),
                    slot_type: slot.slot_type.clone(),
                    branch: "—".dimmed().to_string(),
                    git_status: "—".dimmed().to_string(),
                    availability: "introuvable".red().to_string(),
                }
            }
        })
        .collect();

    let table = Table::new(rows).to_string();
    println!("{table}");
}
