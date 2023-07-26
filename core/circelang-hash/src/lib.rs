/*

Copyright (C) 2023 Carlos Kieliszewski

This file is part of the Circe Project.

Circe is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option)
any later version.

Circe is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
Circe. If not, see <https://www.gnu.org/licenses/>. 

*/

pub use circelang_hash_proc::CirceHash;


pub trait CirceHash {
    fn hash(&self) -> u64;
}

macro_rules! hash_is_identity {
    ( $( $t:ty ),* ) => {
        $(
            impl CirceHash for $t {
                fn hash(&self) -> u64 {
                    *self as u64
                }
            }
        )*
    }
}

hash_is_identity![
    u8, u16, u32, u64
];

macro_rules! hash_is_shifted_identity {
    ( $( $t:ty, $s:expr );* $(;)? ) => {
        $(
            impl CirceHash for $t {
                fn hash(&self) -> u64 {
                    (*self + $s) as u64
                }
            }
        )*
    }
}

hash_is_shifted_identity![
    i8, i8::MAX / 2;
    i16, i16::MAX / 2;
    i32, i32::MAX / 2;
    i64, i64::MAX / 2;
];

impl CirceHash for bool {
    fn hash(&self) -> u64 {
        if *self {
            1
        } else {
            0
        }
    }
}

impl CirceHash for char {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

impl CirceHash for String {
    fn hash(&self) -> u64 {
        let mut hash: u64 = 0;

        for (i, c) in self.chars().enumerate() {
            hash ^= c.hash().rotate_right(i as u32 % 64);
        }

        hash
    }
}

impl CirceHash for f32 {
    fn hash(&self) -> u64 {
        self.to_bits().hash()
    }
}

impl CirceHash for f64 {
    fn hash(&self) -> u64 {
        self.to_bits().hash()
    }
}

#[cfg(target_pointer_width = "32")]
hash_is_identity![usize];
#[cfg(target_pointer_width = "64")]
hash_is_identity![usize];

#[cfg(target_pointer_width = "32")]
hash_is_shifted_identity![isize, isize::MAX / 2];
#[cfg(target_pointer_width = "64")]
hash_is_shifted_identity![isize, isize::MAX / 2];

impl<T: CirceHash> CirceHash for Option<T> {
    fn hash(&self) -> u64 {
        match self {
            Some(t) => t.hash(),
            None => 0
        }
    }
}

impl<T: CirceHash> CirceHash for Vec<T> {
    fn hash(&self) -> u64 {
        let mut hash: u64 = 0;

        for (i, t) in self.iter().enumerate() {
            hash ^= t.hash().rotate_right(i as u32 % 64);
        }

        hash
    }
}

impl<T: CirceHash> CirceHash for [T] {
    fn hash(&self) -> u64 {
        let mut hash: u64 = 0;

        for (i, t) in self.iter().enumerate() {
            hash ^= t.hash().rotate_right(i as u32 % 64);
        }

        hash
    }
}

impl<T: CirceHash, const N: usize> CirceHash for [T; N] {
    fn hash(&self) -> u64 {
        let mut hash: u64 = 0;

        for (i, t) in self.iter().enumerate() {
            hash ^= t.hash().rotate_right(i as u32 % 64);
        }

        hash
    }
}

impl<T: CirceHash> CirceHash for Box<T> {
    fn hash(&self) -> u64 {
        self.as_ref().hash()
    }
}

impl<T: CirceHash> CirceHash for &T {
    fn hash(&self) -> u64 {
        (*self).hash()
    }
}
