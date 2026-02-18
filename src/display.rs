use colored::Colorize;

use crate::config::SlotConfig;
use crate::git;

pub fn slot_label(slot: &SlotConfig) -> String {
    let name = format!("{:<10}", slot.name);
    let stype = format!("{:<6}", slot.slot_type);

    if let Some(status) = git::get_status(&slot.path) {
        let branch = format!("{:<20}", status.branch);
        let git_text = if status.is_clean() {
            "clean".green().to_string()
        } else {
            status.status_text().yellow().to_string()
        };

        if status.is_available() {
            format!("{} {} {} {}", name, stype.dimmed(), branch.dimmed(), git_text)
        } else {
            format!("{} {} {} {}", name.bold(), stype, branch.cyan(), git_text)
        }
    } else {
        format!("{} {} {}", name, stype.dimmed(), "introuvable".red())
    }
}
