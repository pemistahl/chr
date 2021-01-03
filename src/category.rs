/*
 * Copyright © 2021 Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use strum_macros::EnumString;

#[derive(EnumString)]
pub enum Category {
    Lu,
    Ll,
    Lt,
    Lm,
    Lo,
    Mn,
    Mc,
    Me,
    Nd,
    Nl,
    No,
    Pc,
    Pd,
    Ps,
    Pe,
    Pi,
    Pf,
    Po,
    Sm,
    Sc,
    Sk,
    So,
    Zs,
    Zl,
    Zp,
    Cc,
    Cf,
    Cs,
    Co,
    Cn,
}

impl Category {
    pub fn description(&self) -> &'static str {
        match self {
            Category::Lu => "Uppercase Letter",
            Category::Ll => "Lowercase Letter",
            Category::Lt => "Titlecase Letter",
            Category::Lm => "Modifier Letter",
            Category::Lo => "Other Letter",
            Category::Mn => "Non-spacing Mark",
            Category::Mc => "Spacing Mark",
            Category::Me => "Enclosing Mark",
            Category::Nd => "Decimal Number",
            Category::Nl => "Letter Number",
            Category::No => "Other Number",
            Category::Pc => "Connector Punctuation",
            Category::Pd => "Dash Punctuation",
            Category::Ps => "Opening Punctuation",
            Category::Pe => "Closing Punctuation",
            Category::Pi => "Initial Quotation Mark",
            Category::Pf => "Final Quotation Mark",
            Category::Po => "Other Punctuation",
            Category::Sm => "Mathematical Symbol",
            Category::Sc => "Currency Sign",
            Category::Sk => "Non-letter Modifier Symbol",
            Category::So => "Other Symbol",
            Category::Zs => "Space Separator",
            Category::Zl => "Line Separator",
            Category::Zp => "Paragraph Separator",
            Category::Cc => "Control Character",
            Category::Cf => "Format Control Character",
            Category::Cs => "Surrogate Code Point",
            Category::Co => "Private-use Character",
            Category::Cn => "Reserved Unassigned Code Point",
        }
    }
}
