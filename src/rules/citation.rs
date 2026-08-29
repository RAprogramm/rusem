// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Where a rule is written down.
//!
//! A finding that cannot name its paragraph is an assertion; one that can is a
//! citation. The difference is what a reader can do with it: § 40 can be
//! looked up, argued with, and found to have been misapplied.
//!
//! The code of 1956 states two hundred and three paragraphs, several of them
//! in numbered points. A rule names both.

/// The paragraph and point of the code a rule is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Citation {
    /// The paragraph, from 1 to 203.
    pub paragraph: u16,
    /// The point within the paragraph, or zero when the paragraph is whole.
    pub point:     u16
}

/// The last paragraph the code of 1956 states.
pub const LAST: u16 = 203;

impl Citation {
    /// A whole paragraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::rules::Citation;
    ///
    /// assert_eq!(Citation::whole(40).to_string(), "§ 40");
    /// ```
    #[must_use]
    pub const fn whole(paragraph: u16) -> Self {
        Self {
            paragraph,
            point: 0
        }
    }

    /// One point of a paragraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::rules::Citation;
    ///
    /// assert_eq!(Citation::point(70, 3).to_string(), "§ 70, п. 3");
    /// ```
    #[must_use]
    pub const fn point(paragraph: u16, point: u16) -> Self {
        Self {
            paragraph,
            point
        }
    }

    /// Reports whether the citation names a paragraph the code states.
    ///
    /// A rule citing § 0 or § 300 is citing nothing, and the registry refuses
    /// it rather than carrying a reference a reader cannot follow.
    #[must_use]
    pub const fn is_stated(self) -> bool {
        self.paragraph >= 1 && self.paragraph <= LAST
    }
}

impl core::fmt::Display for Citation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.point == 0 {
            return write!(formatter, "§ {}", self.paragraph);
        }

        write!(formatter, "§ {}, п. {}", self.paragraph, self.point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_whole_paragraph_names_only_itself() {
        assert_eq!(Citation::whole(40).to_string(), "§ 40");
    }

    #[test]
    fn a_point_names_both() {
        assert_eq!(Citation::point(70, 3).to_string(), "§ 70, п. 3");
    }

    #[test]
    fn a_paragraph_outside_the_code_is_not_stated() {
        assert!(Citation::whole(1).is_stated());
        assert!(Citation::whole(LAST).is_stated());
        assert!(!Citation::whole(0).is_stated());
        assert!(!Citation::whole(LAST + 1).is_stated());
    }

    #[test]
    fn citations_order_by_paragraph_then_point() {
        let mut held = std::vec![
            Citation::point(40, 2),
            Citation::whole(7),
            Citation::point(40, 1)
        ];
        held.sort_unstable();

        assert_eq!(
            held,
            std::vec![
                Citation::whole(7),
                Citation::point(40, 1),
                Citation::point(40, 2)
            ]
        );
    }
}
