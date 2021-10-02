//! # Honey tree data
//!
//! Module that contains structs and data related to honey trees.

///
/// Constant that holds all existing honey trees.
///
pub const HONEY_TREES: [HoneyTree; 21] = [
    HoneyTree {
        location: "Route 205 south",
    },
    HoneyTree {
        location: "Route 205 north",
    },
    HoneyTree {
        location: "Route 206",
    },
    HoneyTree {
        location: "Route 207",
    },
    HoneyTree {
        location: "Route 208",
    },
    HoneyTree {
        location: "Route 209",
    },
    HoneyTree {
        location: "Route 210 south",
    },
    HoneyTree {
        location: "Route 210 north",
    },
    HoneyTree {
        location: "Route 211 east",
    },
    HoneyTree {
        location: "Route 212 north",
    },
    HoneyTree {
        location: "Route 212 south",
    },
    HoneyTree {
        location: "Route 213",
    },
    HoneyTree {
        location: "Route 214",
    },
    HoneyTree {
        location: "Route 215",
    },
    HoneyTree {
        location: "Route 218",
    },
    HoneyTree {
        location: "Route 221",
    },
    HoneyTree {
        location: "Route 222",
    },
    HoneyTree {
        location: "Valley Windworks",
    },
    HoneyTree {
        location: "Eterna Forest (exterior)",
    },
    HoneyTree {
        location: "Fuego Ironworks",
    },
    HoneyTree {
        location: "Floaroma Meadow",
    },
];

///
/// Struct that holds a honey tree.
/// A honey tree is defined by is in-game location.
///
/// # Examples
/// ```
/// let my_tree = HoneyTree {location: "Route 201"};
/// println!(my_tree.location);
/// ```
pub struct HoneyTree<'a> {
    pub location: &'a str,
}
