//! # Honey tree calculator
//!
//! Module that contains functions used to calculate the honey tree locations.

use super::result::HoneyTreeResult;
use super::tree::HONEY_TREES;

///
/// Calculates the honey trees for a given trainer ID. and secret ID.
///
/// # Examples
/// ```
/// calculate_honey_trees(12345, 54321).into_iter().for_each(|tree| println!("{}", tree.location));
/// ```
pub fn calculate_honey_trees(trainer_id: u16, secret_id: u16) -> HoneyTreeResult<'static> {
    let tid1 = trainer_id % 256;
    let tid2 = trainer_id / 256;

    let sid1 = secret_id % 256;
    let sid2 = secret_id / 256;

    let a = sid1 % 21;
    let mut b = sid2 % 21;
    let mut c = tid1 % 21;
    let mut d = tid2 % 21;

    if a == b {
        b += 1;
        reset_if_overflows(&mut b);
    }

    if a == c {
        c += 1;
        reset_if_overflows(&mut c);
    }

    if b == c {
        c += 1;
        reset_if_overflows(&mut c);
    }

    if a == d {
        d += 1;
        reset_if_overflows(&mut d);
    }

    if b == d {
        d += 1;
        reset_if_overflows(&mut d);
    }

    if c == d {
        d += 1;
        reset_if_overflows(&mut d);
    }

    HoneyTreeResult {
        tree1: &HONEY_TREES[a as usize],
        tree2: &HONEY_TREES[b as usize],
        tree3: &HONEY_TREES[c as usize],
        tree4: &HONEY_TREES[d as usize],
    }
}

fn reset_if_overflows(val: &mut u16) {
    if *val == 21 {
        *val = 0;
    }
}
