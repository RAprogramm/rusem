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

/// Where the paragraph is written.
pub const CITES: Citation = Citation::whole(88);

/// The particle the paragraph joins.
pub const PARTICLE: &str = "не";
