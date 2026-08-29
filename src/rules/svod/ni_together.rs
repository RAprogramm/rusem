// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 90. Пишется слитно частица `ни`.
//!
//! Как и § 88, параграф перечисляет случаи слияния; написан пункт о
//! местоимениях, а пункт о наречиях ждёт своей очереди.
//!
//! | Пункт | Модуль | О чём |
//! | --- | --- | --- |
//! | 90.1 | [`pronouns`] | `ни` при вопросительном слове: `никто`, `ничей` |

pub mod pronouns;

use crate::rules::Citation;

/// Where the paragraph is written.
pub const CITES: Citation = Citation::whole(90);

/// The particle the paragraph joins.
pub const PARTICLE: &str = "ни";
