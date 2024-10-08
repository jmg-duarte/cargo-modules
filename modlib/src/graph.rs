// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

use petgraph::stable_graph::StableGraph;

use crate::item::Item;

mod builder;
mod walker;

pub use self::{builder::GraphBuilder, walker::GraphWalker};

pub type Graph<N, E> = StableGraph<N, E>;

pub type Node = Item;
pub type Edge = Relationship;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Relationship {
    Uses,
    Owns,
}

impl Relationship {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Uses => "uses",
            Self::Owns => "owns",
        }
    }
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Uses => "Uses",
            Self::Owns => "Owns",
        };
        write!(f, "{name}")
    }
}
