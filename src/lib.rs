//! A tiny library providing support for `Cardinal`, an enum of the four cardinal directions,
//! and `CardinalValues`, which is a struct indexed by `Cardinal` with a value at each direction.

#![deny(rust_2018_idioms)]
#![allow(clippy::bool_comparison)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::print_stdout)]
#![warn(clippy::todo)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(missing_docs)]
#![no_std]

use core::ops;

/// An enumerator for the simple cardinal directions.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Cardinal {
    /// East, or (1, 0)
    East,
    /// North, or (0, 1)
    North,
    /// West, or (-1, 0)
    West,
    /// South, or (0, -1)
    South,
}

impl Cardinal {
    /// Rotates a cardinal.
    #[must_use = "this returns the result of the operation, \
    without modifying the original"]
    pub fn rotate(self, amount: i32) -> Self {
        let mut v = match self {
            Cardinal::East => 0,
            Cardinal::North => 1,
            Cardinal::West => 2,
            Cardinal::South => 3,
        };

        v += amount;
        v = v.rem_euclid(4);

        match v {
            0 => Cardinal::East,
            1 => Cardinal::North,
            2 => Cardinal::West,
            3 => Cardinal::South,
            _ => unreachable!(),
        }
    }

    /// Gives an iterator over the four cardinals
    pub fn iter_values() -> impl Iterator<Item = Self> {
        [
            Cardinal::East,
            Cardinal::North,
            Cardinal::West,
            Cardinal::South,
        ]
        .into_iter()
    }

    /// Converts to a simple tuple int form.
    /// This assumes that north is up.
    pub fn to_ivec2(self) -> (i32, i32) {
        match self {
            Cardinal::East => (1, 0),
            Cardinal::North => (0, 1),
            Cardinal::West => (-1, 0),
            Cardinal::South => (0, -1),
        }
    }

    /// Returns an angle representing the Cardinal
    pub fn to_angle(self) -> f32 {
        match self {
            Cardinal::East => 0.0,
            Cardinal::North => 90.0,
            Cardinal::West => 180.0,
            Cardinal::South => 270.0,
        }
    }

    /// Is either West or East
    pub fn is_horizontal(self) -> bool {
        matches!(self, Self::East | Self::West)
    }

    /// Is either North or South
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::North | Self::South)
    }
}

impl core::fmt::Display for Cardinal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let word = match self {
            Cardinal::East => "east",
            Cardinal::North => "north",
            Cardinal::West => "west",
            Cardinal::South => "south",
        };

        f.pad(word)
    }
}

/// A struct which a value assigned to each cardinal. This can be used as a shorthand for
/// accessing arrays.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CardinalValues<T> {
    /// The value assigned to east.
    pub east: T,
    /// The value assigned to north.
    pub north: T,
    /// The value assigned to west.
    pub west: T,
    /// The value assigned to south.
    pub south: T,
}

impl<T> CardinalValues<T> {
    /// Converts a [CardinalValues] from one type to another.
    pub fn map<B, F>(self, mut f: F) -> CardinalValues<B>
    where
        F: FnMut(T) -> B,
    {
        CardinalValues {
            east: f(self.east),
            north: f(self.north),
            west: f(self.west),
            south: f(self.south),
        }
    }
}

impl<T> ops::Index<Cardinal> for CardinalValues<T> {
    type Output = T;

    fn index(&self, index: Cardinal) -> &Self::Output {
        match index {
            Cardinal::East => &self.east,
            Cardinal::North => &self.north,
            Cardinal::West => &self.west,
            Cardinal::South => &self.south,
        }
    }
}

impl<T: Copy> IntoIterator for CardinalValues<T> {
    type Item = T;

    type IntoIter = CardinalIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        CardinalIterator(self, 0)
    }
}

/// An iterator over a CardinalValues.
pub struct CardinalIterator<T>(CardinalValues<T>, usize);

impl<T> CardinalIterator<T> {
    /// Converts this iterator into an Enumerated one, where each value has its Cardinal given.
    pub fn enumerate(self) -> CardinalEnumeratedIterator<T> {
        CardinalEnumeratedIterator(self.0, self.1)
    }
}

impl<T: Copy> Iterator for CardinalIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let found = match self.1 {
            0 => Some(self.0.east),
            1 => Some(self.0.west),
            2 => Some(self.0.north),
            3 => Some(self.0.south),
            _ => return None,
        };

        self.1 += 1;

        found
    }
}

/// An enumerated iterator for [CardinalValues]. This should be constructed with the `enumerate` method
/// on [CardinalIterator].
pub struct CardinalEnumeratedIterator<T>(CardinalValues<T>, usize);
impl<T: Copy> Iterator for CardinalEnumeratedIterator<T> {
    type Item = (Cardinal, T);

    fn next(&mut self) -> Option<Self::Item> {
        let found = match self.1 {
            0 => Some((Cardinal::North, self.0.north)),
            1 => Some((Cardinal::West, self.0.west)),
            2 => Some((Cardinal::South, self.0.south)),
            3 => Some((Cardinal::East, self.0.east)),
            _ => return None,
        };

        self.1 += 1;

        found
    }
}
