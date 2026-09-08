// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 88. Пишется слитно частица `не`.
//!
//! Параграф перечисляет, где `не` сливается со словом, к которому относится.
//! Написан один его пункт — о местоимениях; остальные ждут фактов, которых
//! движок пока не выводит.
//!
//! | Пункт | Модуль | О чём |
//! | --- | --- | --- |
//! | 88.5 | [`pronouns`] | `не` при вопросительном слове: `некто`, `нечего` |

pub mod pronouns;

use crate::rules::Citation;

/// § 88 as a rule.
///
/// Holds where the paragraph is written and the particle it joins.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::ne_together::Rule;
///
/// assert_eq!(Rule::CITES.paragraph, 88);
/// assert_eq!(Rule::PARTICLE, "не");
/// ```
pub struct Rule;

impl Rule {
    /// Where the paragraph is written.
    pub const CITES: Citation = Citation::whole(88);

    /// The particle the paragraph joins.
    pub const PARTICLE: &str = "не";
}
