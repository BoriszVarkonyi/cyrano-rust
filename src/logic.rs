use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::collections::HashMap;

const ORDER_32: [usize; 32] = [
    1, 32, 17, 16, 9, 24, 25, 8, 5, 28, 21, 12, 13, 20, 29, 4, 3, 30, 19, 14, 11, 22, 27, 6,
    7, 26, 23, 10, 15, 18, 31, 2,
];

const ORDER_16: [usize; 16] = [1, 16, 9, 8, 5, 12, 13, 4, 3, 14, 11, 6, 7, 10, 15, 2];
const ORDER_8: [usize; 8] = [1, 8, 5, 4, 3, 6, 7, 2];
const ORDER_4: [usize; 4] = [1, 4, 3, 2];
const ORDER_2: [usize; 2] = [1, 2];

#[derive(Debug, Deserialize)]
struct Fencer {
    name: String,
    nation: String,
    rank: usize,
}

#[derive(Debug, Deserialize)]
struct PisteRef {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct TableMatch {
    pub round: usize,
    pub match_no: usize,
    pub piste: Option<String>,
    pub match_order: Option<usize>,
    pub fencer_1: String,
    pub fencer_1_nation: String,
    pub fencer_1_score: i8,
    pub fencer_2: String,
    pub fencer_2_nation: String,
    pub fencer_2_score: i8,
}

fn bracket_size(fencers: usize) -> Result<usize, Box<dyn Error>> {
    match fencers {
        0 | 1 => Err("At least 2 fencers are required".into()),
        2 => Ok(2),
        3..=4 => Ok(4),
        5..=8 => Ok(8),
        9..=16 => Ok(16),
        17..=32 => Ok(32),
        _ => Err("Maximum supported table is 32".into()),
    }
}

fn seeding_order(size: usize) -> Result<&'static [usize], Box<dyn Error>> {
    match size {
        2 => Ok(&ORDER_2),
        4 => Ok(&ORDER_4),
        8 => Ok(&ORDER_8),
        16 => Ok(&ORDER_16),
        32 => Ok(&ORDER_32),
        _ => Err("Unsupported table size".into()),
    }
}

pub fn generate_table() -> Result<Vec<TableMatch>, Box<dyn Error>> {
    generate_table_from_paths("src/data/fencers.json", "src/data/matches.json")
}

pub fn generate_table_from_paths(
    fencers_path: &str,
    matches_path: &str,
) -> Result<Vec<TableMatch>, Box<dyn Error>> {
    generate_table_from_paths_with_optional_pistes(fencers_path, matches_path, None)
}

pub fn generate_table_with_pistes() -> Result<Vec<TableMatch>, Box<dyn Error>> {
    generate_table_from_paths_with_optional_pistes(
        "src/data/fencers.json",
        "src/data/matches.json",
        Some("src/data/pistes.json"),
    )
}

pub fn generate_table_from_paths_with_optional_pistes(
    fencers_path: &str,
    matches_path: &str,
    pistes_path: Option<&str>,
) -> Result<Vec<TableMatch>, Box<dyn Error>> {
    let raw = fs::read_to_string(fencers_path)?;
    let fencers: Vec<Fencer> = serde_json::from_str(&raw)?;

    let table_size = bracket_size(fencers.len())?;
    let order = seeding_order(table_size)?;

    // Seed index is 1-based: seed #1 is at index 1.
    // We place fencers by their explicit rank, not by array order.
    let mut seed_to_fencer: Vec<Option<&Fencer>> = vec![None; table_size + 1];
    for fencer in &fencers {
        if fencer.rank == 0 {
            return Err("Invalid rank 0; ranks must start from 1".into());
        }
        if fencer.rank > table_size {
            return Err(format!(
                "Fencer rank {} is outside current table size {}",
                fencer.rank, table_size
            )
            .into());
        }
        if seed_to_fencer[fencer.rank].is_some() {
            return Err(format!("Duplicate rank {} in fencers.json", fencer.rank).into());
        }
        seed_to_fencer[fencer.rank] = Some(fencer);
    }

    let mut generated: Vec<TableMatch> = Vec::new();
    let mut match_no = 1usize;

    for pair in order.chunks_exact(2) {
        let seed_a = pair[0];
        let seed_b = pair[1];

        let fencer_a = seed_to_fencer[seed_a];
        let fencer_b = seed_to_fencer[seed_b];

        generated.push(TableMatch {
            round: table_size,
            match_no,
            piste: None,
            match_order: None,
            fencer_1: fencer_a.map(|f| f.name.clone()).unwrap_or_default(),
            fencer_1_nation: fencer_a.map(|f| f.nation.clone()).unwrap_or_default(),
            fencer_1_score: 0,
            fencer_2: fencer_b.map(|f| f.name.clone()).unwrap_or_default(),
            fencer_2_nation: fencer_b.map(|f| f.nation.clone()).unwrap_or_default(),
            fencer_2_score: 0,
        });

        match_no += 1;
    }

    let mut round = table_size / 2;
    while round >= 2 {
        for _ in 0..(round / 2) {
            generated.push(TableMatch {
                round,
                match_no,
                piste: None,
                match_order: None,
                fencer_1: String::new(),
                fencer_1_nation: String::new(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            });

            match_no += 1;
        }

        if round == 2 {
            break;
        }
        round /= 2;
    }

    auto_advance_byes(&mut generated, table_size);

    if let Some(path) = pistes_path {
        let raw_pistes = fs::read_to_string(path)?;
        let pistes: Vec<PisteRef> = serde_json::from_str(&raw_pistes)?;
        assign_pistes_and_orders(&mut generated, &pistes);
    }

    let json = serde_json::to_string_pretty(&generated)?;
    fs::write(matches_path, json)?;

    Ok(generated)
}

fn assign_pistes_and_orders(matches: &mut [TableMatch], pistes: &[PisteRef]) {
    if pistes.is_empty() {
        return;
    }

    let mut per_piste_order: HashMap<String, usize> = HashMap::new();

    for (idx, m) in matches.iter_mut().enumerate() {
        let piste_name = pistes[idx % pistes.len()].name.clone();
        let next_order = per_piste_order.entry(piste_name.clone()).or_insert(0);
        *next_order += 1;

        m.piste = Some(piste_name);
        m.match_order = Some(*next_order);
    }
}

fn auto_advance_byes(matches: &mut [TableMatch], start_round: usize) {
    if start_round < 4 {
        return;
    }

    let mut round_meta: HashMap<usize, (usize, usize)> = HashMap::new();
    for (idx, m) in matches.iter().enumerate() {
        round_meta
            .entry(m.round)
            .and_modify(|(_, count)| *count += 1)
            .or_insert((idx, 1));
    }

    let round = start_round;
    let Some((start_idx, count)) = round_meta.get(&round).copied() else {
        return;
    };
    let Some((next_start_idx, _)) = round_meta.get(&(round / 2)).copied() else {
        return;
    };

    for local_idx in 0..count {
        let curr_idx = start_idx + local_idx;
        let curr = &matches[curr_idx];

        let a_present = !curr.fencer_1.is_empty();
        let b_present = !curr.fencer_2.is_empty();

        // Exactly one present means automatic advance on BYE.
        if a_present == b_present {
            continue;
        }

        let (name, nation) = if a_present {
            (curr.fencer_1.clone(), curr.fencer_1_nation.clone())
        } else {
            (curr.fencer_2.clone(), curr.fencer_2_nation.clone())
        };

        let next_match_local_idx = local_idx / 2;
        let next_idx = next_start_idx + next_match_local_idx;
        let goes_to_slot_1 = local_idx % 2 == 0;

        let next = &mut matches[next_idx];
        if goes_to_slot_1 {
            if next.fencer_1.is_empty() {
                next.fencer_1 = name;
                next.fencer_1_nation = nation;
            }
        } else if next.fencer_2.is_empty() {
            next.fencer_2 = name;
            next.fencer_2_nation = nation;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_size_is_rounded_up() {
        assert_eq!(bracket_size(4).unwrap(), 4);
        assert_eq!(bracket_size(5).unwrap(), 8);
        assert_eq!(bracket_size(9).unwrap(), 16);
        assert_eq!(bracket_size(17).unwrap(), 32);
    }

    #[test]
    fn assigns_piste_and_match_order_per_piste() {
        let mut matches = vec![
            TableMatch {
                round: 8,
                match_no: 1,
                piste: None,
                match_order: None,
                fencer_1: String::new(),
                fencer_1_nation: String::new(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 8,
                match_no: 2,
                piste: None,
                match_order: None,
                fencer_1: String::new(),
                fencer_1_nation: String::new(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 4,
                match_no: 3,
                piste: None,
                match_order: None,
                fencer_1: String::new(),
                fencer_1_nation: String::new(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            },
        ];

        let pistes = vec![
            PisteRef { name: "1".into() },
            PisteRef { name: "2".into() },
        ];

        assign_pistes_and_orders(&mut matches, &pistes);

        assert_eq!(matches[0].piste.as_deref(), Some("1"));
        assert_eq!(matches[0].match_order, Some(1));
        assert_eq!(matches[1].piste.as_deref(), Some("2"));
        assert_eq!(matches[1].match_order, Some(1));
        assert_eq!(matches[2].piste.as_deref(), Some("1"));
        assert_eq!(matches[2].match_order, Some(2));
    }

    #[test]
    fn advances_bye_to_next_round_slot() {
        let mut matches = vec![
            TableMatch {
                round: 4,
                match_no: 1,
                piste: None,
                match_order: None,
                fencer_1: "A".into(),
                fencer_1_nation: "HUN".into(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 4,
                match_no: 2,
                piste: None,
                match_order: None,
                fencer_1: "B".into(),
                fencer_1_nation: "HUN".into(),
                fencer_1_score: 0,
                fencer_2: "C".into(),
                fencer_2_nation: "HUN".into(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 2,
                match_no: 3,
                piste: None,
                match_order: None,
                fencer_1: String::new(),
                fencer_1_nation: String::new(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            },
        ];

        auto_advance_byes(&mut matches, 4);

        assert_eq!(matches[2].fencer_1, "A");
        assert_eq!(matches[2].fencer_1_nation, "HUN");
        assert!(matches[2].fencer_2.is_empty());
    }

    #[test]
    fn does_not_cascade_bye_advances_beyond_one_round() {
        let mut matches = vec![
            TableMatch {
                round: 8,
                match_no: 1,
                piste: None,
                match_order: None,
                fencer_1: "A".into(),
                fencer_1_nation: "HUN".into(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 8,
                match_no: 2,
                piste: None,
                match_order: None,
                fencer_1: "B".into(),
                fencer_1_nation: "HUN".into(),
                fencer_1_score: 0,
                fencer_2: "C".into(),
                fencer_2_nation: "HUN".into(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 8,
                match_no: 3,
                piste: None,
                match_order: None,
                fencer_1: "D".into(),
                fencer_1_nation: "HUN".into(),
                fencer_1_score: 0,
                fencer_2: "E".into(),
                fencer_2_nation: "HUN".into(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 8,
                match_no: 4,
                piste: None,
                match_order: None,
                fencer_1: "F".into(),
                fencer_1_nation: "HUN".into(),
                fencer_1_score: 0,
                fencer_2: "G".into(),
                fencer_2_nation: "HUN".into(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 4,
                match_no: 5,
                piste: None,
                match_order: None,
                fencer_1: String::new(),
                fencer_1_nation: String::new(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 4,
                match_no: 6,
                piste: None,
                match_order: None,
                fencer_1: String::new(),
                fencer_1_nation: String::new(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            },
            TableMatch {
                round: 2,
                match_no: 7,
                piste: None,
                match_order: None,
                fencer_1: String::new(),
                fencer_1_nation: String::new(),
                fencer_1_score: 0,
                fencer_2: String::new(),
                fencer_2_nation: String::new(),
                fencer_2_score: 0,
            },
        ];

        auto_advance_byes(&mut matches, 8);

        assert_eq!(matches[4].fencer_1, "A");
        assert!(matches[6].fencer_1.is_empty());
        assert!(matches[6].fencer_2.is_empty());
    }
}
