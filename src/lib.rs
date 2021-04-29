// //! # My Crate
// //!
// //! `my_crate` is a collection of utilities to make performing certain
// //! calculations more convenient.

// /// Adds one to the number given.
// ///
// /// # Examples
// ///
// /// ```
// /// let arg = 5;
// /// let answer = pub_crates::add_one(arg);
// ///
// /// assert_eq!(6, answer);
// /// ```
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(add_one(3), 4);
//     }
// }

//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    // --snip--
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
