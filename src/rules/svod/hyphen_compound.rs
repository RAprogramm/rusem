// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 86. Пишутся через дефис.
//!
//! Параграф перечисляет, что в русском письме соединяется дефисом, и каждый
//! его пункт — о своём: наречия с приставкой из предлога, повторяющиеся
//! междометия, местоимения с частицами. Пункты не спорят друг с другом и не
//! пересекаются, поэтому каждый живёт своим файлом и цитируется своим
//! номером.
//!
//! | Пункт | Модуль | О чём |
//! | --- | --- | --- |
//! | 86.2 | [`interjections`] | повторяющееся междометие |
//! | 86.3 | [`particles`] | частицы кое-, -то, -либо, -нибудь |
//!
//! Пункт о наречиях с предлогом здесь ещё не написан: он требует знать, из
//! какого предлога и какого слова наречие сложено, а этого движок пока не
//! выводит.

pub mod interjections;
pub mod particles;

use crate::rules::Citation;

/// § 86 as a rule.
///
/// Holds where the paragraph is written and the letter it joins with.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::hyphen_compound::Rule;
///
/// assert_eq!(Rule::CITES.paragraph, 86);
/// assert_eq!(Rule::HYPHEN, '-');
/// ```
pub struct Rule;

impl Rule {
    /// Where the paragraph is written.
    pub const CITES: Citation = Citation::whole(86);

    /// The letter the paragraph joins with.
    pub const HYPHEN: char = '-';
}
