// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nalgebra as na;
use slotmap::SlotMap;

use std::hash::RandomState;

use crate::{Element, bond::BondType, element::MassNumber, entities::*, id::*};

/// An extensible arena-like data structure to represent a set of chemical
/// entities and the relationships between them, as a molecular graph.
#[derive(Debug)]
pub struct MolMap<Extension> {
    pub(crate) bonds: SlotMap<BondId, Bond>,
    pub(crate) atoms: SlotMap<AtomId, Atom>,
    pub(crate) pseudoatoms: SlotMap<PseudoatomId, Pseudoatom>,
    pub(crate) fragments: SlotMap<FragmentId, Fragment>,
    pub(crate) molecules: SlotMap<MoleculeId, Molecule>,
    //pub(crate) objects: SlotMap<ObjectId, Object>,
    pub(crate) extension: Extension,
}

// Loading from file involves a lot of insertions and therefore if the initial capacity was 0
// multiple expensive reallocations would occur every time the slotmaps filled up, so try to improve
// performance by pre-allocating a sensible amount of space (say enough for a well-populated scheme
// of A4 size) for each slotmap

impl<E: Default> Default for MolMap<E> {
    fn default() -> Self {
        Self {
            bonds: SlotMap::with_capacity_and_key(500),
            atoms: SlotMap::with_capacity_and_key(500),
            pseudoatoms: SlotMap::with_capacity_and_key(500),
            fragments: SlotMap::with_capacity_and_key(1000),
            molecules: SlotMap::with_capacity_and_key(50),
            //objects: SlotMap::with_capacity_and_key(100),
            extension: E::default(),
        }
    }
}

impl<E: Default> MolMap<E> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E> MolMap<E> {
    // Methods to add entities

    /// Add an `Atom` to the `MolMap`.
    pub fn add_atom(&mut self, element: Element) -> AtomId {
        self.atoms.insert_with_key(|id| Atom::new(id, element))
    }

    /// Add a `Pseudoatom` to the `MolMap`.
    pub fn add_pseudoatom(&mut self, symbol: &str) -> PseudoatomId {
        self.pseudoatoms
            .insert_with_key(|id| Pseudoatom::new(id, symbol.to_owned()))
    }

    /// Add a `Fragment` to the `MolMap` with a single initial atom.
    pub fn add_fragment(&mut self, centre: Atomlike) -> FragmentId {
        self.fragments.insert_with_key(|id| {
            Fragment::new(
                id,
                fragment::FragmentBondingCentre::Single(centre),
                &[centre.into()],
            )
        })
    }

    /// Add an empty `Molecule` to the `MolMap`.
    pub fn add_molecule(&mut self) -> MoleculeId {
        self.molecules.insert_with_key(Molecule::new)
    }

    /// Create a new `Bond` between two bondable entities.
    pub fn create_bond(&mut self, start: Bondable, end: Bondable) -> BondId {
        self.bonds
            .insert_with_key(|id| Bond::new(id, BondType::Covalent, 1.0, start.into(), end.into()))
    }
}
