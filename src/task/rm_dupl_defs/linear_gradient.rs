/****************************************************************************
**
** SVG Cleaner could help you to clean up your SVG files
** from unnecessary data.
** Copyright (C) 2012-2016 Evgeniy Reizner
**
** This program is free software; you can redistribute it and/or modify
** it under the terms of the GNU General Public License as published by
** the Free Software Foundation; either version 2 of the License, or
** (at your option) any later version.
**
** This program is distributed in the hope that it will be useful,
** but WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
** GNU General Public License for more details.
**
** You should have received a copy of the GNU General Public License along
** with this program; if not, write to the Free Software Foundation, Inc.,
** 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
**
****************************************************************************/

use task::short::{EId, AId};

use svgdom::Document;

pub fn remove_dupl_linear_gradients(doc: &Document) {
    let attrs = [
        AId::X1,
        AId::Y1,
        AId::X2,
        AId::Y2,
        AId::GradientUnits,
        AId::SpreadMethod,
    ];

    super::rm_loop(doc, EId::LinearGradient, &attrs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use svgdom::{Document, WriteToString};
    use task::resolve_attrs;

    macro_rules! test_lg {
        ($name:ident, $in_text:expr, $out_text:expr) => (
            #[test]
            fn $name() {
                let doc = Document::from_data($in_text).unwrap();
                resolve_attrs::linear_gradients(&doc);
                remove_dupl_linear_gradients(&doc);
                assert_eq!(doc_to_str_tests!(doc), $out_text);
            }
        )
    }

    test_lg!(rm_1,
b"<svg>
    <defs>
        <linearGradient id='lg1'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </linearGradient>
        <linearGradient id='lg2'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </linearGradient>
    </defs>
    <rect fill='url(#lg2)'/>
</svg>",
"<svg>
    <defs>
        <linearGradient id='lg1'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </linearGradient>
    </defs>
    <rect fill='url(#lg1)'/>
</svg>
");

    test_lg!(rm_2,
b"<svg>
    <defs>
        <linearGradient id='lg1'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </linearGradient>
        <linearGradient id='lg2'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </linearGradient>
        <linearGradient id='lg3'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </linearGradient>
    </defs>
    <rect fill='url(#lg2)'/>
    <rect fill='url(#lg3)'/>
</svg>",
"<svg>
    <defs>
        <linearGradient id='lg1'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </linearGradient>
    </defs>
    <rect fill='url(#lg1)'/>
    <rect fill='url(#lg1)'/>
</svg>
");

    // different default attributes
    test_lg!(rm_3,
b"<svg>
    <defs>
        <linearGradient id='lg1' x1='0%'/>
        <linearGradient id='lg2' x2='100%'/>
    </defs>
    <rect fill='url(#lg2)'/>
</svg>",
"<svg>
    <defs>
        <linearGradient id='lg1' x1='0%'/>
    </defs>
    <rect fill='url(#lg1)'/>
</svg>
");

    // no 'stop' elements
    test_lg!(rm_4,
b"<svg>
    <defs>
        <linearGradient id='lg1'/>
        <linearGradient id='lg2'/>
    </defs>
    <rect fill='url(#lg2)'/>
</svg>",
"<svg>
    <defs>
        <linearGradient id='lg1'/>
    </defs>
    <rect fill='url(#lg1)'/>
</svg>
");
}