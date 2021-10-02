//! # Honey tree result
//!
//! Module that contains data used to represent honey tree calculation results.

use super::calculator;
use super::tree::HoneyTree;
use std::iter::IntoIterator;

///
/// Struct that holds the four existing Munchlax honey trees.
///
pub struct HoneyTreeResult<'a> {
    pub tree1: &'a HoneyTree<'a>,
    pub tree2: &'a HoneyTree<'a>,
    pub tree3: &'a HoneyTree<'a>,
    pub tree4: &'a HoneyTree<'a>,
}

impl<'a> IntoIterator for HoneyTreeResult<'a> {
    type Item = &'a HoneyTree<'a>;
    type IntoIter = std::array::IntoIter<&'a HoneyTree<'a>, 4>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([self.tree1, self.tree2, self.tree3, self.tree4])
    }
}

///
/// Struct that holds the trainer's ID and SID.
///
pub struct TrainerData {
    trainer_id: u16,
    secret_id: u16,
}

impl TrainerData {
    ///
    /// Initializes a new TrainerData struct with the trainer ID and secret ID.
    ///
    pub fn new(trainer_id: u16, secret_id: u16) -> TrainerData {
        TrainerData {
            trainer_id,
            secret_id,
        }
    }

    ///
    /// Calculates the honey trees for a TrainerData.
    ///
    /// # Examples
    /// ```
    /// use honeytree_calc::htree::result::TrainerData;
    /// let my_data = TrainerData::new(12345, 54321);
    /// my_data.get_honey_trees().into_iter().for_each(|tree| println!("{}", tree.location));
    /// ```
    ///
    pub fn get_honey_trees(&self) -> HoneyTreeResult<'static> {
        calculator::calculate_honey_trees(self.trainer_id, self.secret_id)
    }
}
