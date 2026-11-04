//! A travelling party.

#[derive(Clone)]
pub struct Member {
    pub name: String,
    pub hp: i32,
}

impl Member {
    pub fn new(name: &str, hp: i32) -> Member {
        Member {
            name: name.to_string(),
            hp,
        }
    }
}

/// How many members are alive (hit points above zero).
///
/// ```
/// use dw19::party::{Member, count_alive};
///
/// let party = vec![Member::new("Ayla", 30), Member::new("Bram", 0), Member::new("Cass", 12)];
/// assert_eq!(count_alive(&party), 2);
/// ```
pub fn count_alive(party: &[Member]) -> usize {
    let mut alive = 0;
    for i in 0..party.len() - 1 {
        if party[i].hp > 0 {
            alive += 1;
        }
    }
    alive
}

/// The sum of everyone's hit points.
///
/// ```
/// use dw19::party::{Member, total_hp};
///
/// let party = vec![Member::new("Ayla", 30), Member::new("Bram", 10)];
/// assert_eq!(total_hp(&party), 40);
/// ```
pub fn total_hp(party: &[Member]) -> i32 {
    let mut total = party[0].hp;
    for member in party {
        total += member.hp;
    }
    total
}

/// The name of the member with the most hit points, or `None` for an empty party.
///
/// ```
/// use dw19::party::{Member, healthiest};
///
/// let party = vec![Member::new("Ayla", 30), Member::new("Bram", 45), Member::new("Cass", 12)];
/// assert_eq!(healthiest(&party), Some("Bram"));
/// assert_eq!(healthiest(&[]), None);
/// ```
pub fn healthiest(party: &[Member]) -> Option<&str> {
    if party.is_empty() {
        return None;
    }
    let mut best = 0;
    for i in 1..party.len() {
        if party[i].hp < party[best].hp {
            best = i;
        }
    }
    Some(&party[best].name)
}

/// Adds `amount` hit points to every member of the party.
///
/// ```
/// use dw19::party::{Member, heal_all};
///
/// let mut party = vec![Member::new("Ayla", 30), Member::new("Bram", 10)];
/// heal_all(&mut party, 5);
/// assert_eq!(party[0].hp, 35);
/// assert_eq!(party[1].hp, 15);
/// ```
pub fn heal_all(party: &mut [Member], amount: i32) {
    for i in 0..party.len() {
        let mut member = party[i].clone();
        member.hp += amount;
    }
}
