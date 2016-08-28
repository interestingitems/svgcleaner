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

use svgdom::Document;

pub fn remove_default_attributes(doc: &Document) {
    let mut rm_list = Vec::with_capacity(16);

    for node in doc.descendants() {
        {
            let attrs = node.attributes();

            // TODO: units-per-em and other non-presentation attributes

            for attr in attrs.values() {
                if attr.is_presentation() {
                    if attr.check_is_default() {
                        match node.parent_attribute(attr.id) {
                            Some(pattr) => {
                                if !pattr.visible {
                                    rm_list.push(attr.id);
                                }
                            }
                            None => {
                                rm_list.push(attr.id)
                            }
                        }
                    }
                }
            }
        }

        {
            let mut attrs_mut = node.attributes_mut();
            for aid in &rm_list {
                // we only hide default attributes, because they still can be useful
                attrs_mut.get_mut(*aid).unwrap().visible = false;
                // attrs.remove(aid.clone());
            }
        }

        rm_list.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svgdom::{Document, WriteToString};

    macro_rules! test {
        ($name:ident, $in_text:expr, $out_text:expr) => (
            base_test!($name, remove_default_attributes, $in_text, $out_text);
        )
    }

    test!(rm_1,
b"<svg>
    <rect fill='#000000'/>
</svg>",
"<svg>
    <rect/>
</svg>
");

    test!(keep_1,
b"<svg fill='#ff0000'>
    <rect fill='#000000'/>
</svg>",
"<svg fill='#ff0000'>
    <rect fill='#000000'/>
</svg>
");

    test!(all,
b"<svg>
    <rect fill='#000000'/>
</svg>",
"<svg>
    <rect/>
</svg>
");
}