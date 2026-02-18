use colored::Colorize;

use crate::config::SlotConfig;
use crate::git;

pub fn slot_label(slot: &SlotConfig) -> String {
    let name = format!("{:<10}", slot.name);
    let stype = format!("{:<6}", slot.slot_type);

    if let Some(status) = git::get_status(&slot.path) {
        if status.is_available() {
            format!("{} {}  {}", name.dimmed(), stype.dimmed(), "libre".green())
        } else {
            let detail = if status.is_clean() {
                status.branch.cyan().to_string()
            } else {
                format!("{}  {}", status.branch.cyan(), status.status_text().yellow())
            };
            format!("{} {}  {}", name.bold(), stype, detail)
        }
    } else {
        format!("{} {}  {}", name, stype.dimmed(), "introuvable".red())
    }
}
