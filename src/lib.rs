//! # DW19 — Party Report
//!
//! ```text
//! 4 alive, 87 total HP, healthiest: Ayla
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

pub mod party;

use party::{Member, count_alive, heal_all, healthiest, total_hp};

/// Builds a party, heals everyone by 5, and reports on the result.
///
/// ```
/// assert_eq!(dw19::party_report(), "4 alive, 87 total HP, healthiest: Ayla");
/// ```
pub fn party_report() -> String {
    let mut party = vec![
        Member::new("Ayla", 30),
        Member::new("Bram", 0),
        Member::new("Cass", 12),
        Member::new("Dov", 25),
    ];
    heal_all(&mut party, 5);

    let name = match healthiest(&party) {
        Some(name) => name,
        None => "nobody",
    };
    format!(
        "{} alive, {} total HP, healthiest: {}",
        count_alive(&party),
        total_hp(&party),
        name
    )
}
