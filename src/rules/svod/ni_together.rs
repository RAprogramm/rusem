// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 90. Пишется слитно частица `ни`.
//!
//! Как и § 88, параграф перечисляет случаи слияния, и оба его пункта
//! написаны.
//!
//! | Пункт | Модуль | О чём |
//! | --- | --- | --- |
//! | 90.1 | [`pronouns`] | `ни` при местоимении: `никто`, `ничей` |
//! | 90.2 | [`adverbs`] | `ни` в перечисленных наречиях: `никогда`, `нигде` |

pub mod adverbs;
pub mod pronouns;

use crate::rules::Citation;

/// Where the paragraph is written.
pub const CITES: Citation = Citation::whole(90);

/// The particle the paragraph joins.
pub const PARTICLE: &str = "ни";
