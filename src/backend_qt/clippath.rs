// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use qt;
use usvg;
use usvg::prelude::*;

// self
use super::prelude::*;
use super::{
    path,
    text,
};


pub fn apply(
    node: &usvg::Node,
    cp: &usvg::ClipPath,
    opt: &Options,
    bbox: Rect,
    layers: &mut QtLayers,
    p: &qt::Painter,
) {
    // a-clip-path-001.svg
    // e-clipPath-001.svg

    let clip_img = try_opt!(layers.get(), ());
    let mut clip_img = clip_img.borrow_mut();
    clip_img.fill(0, 0, 0, 255);

    let clip_p = qt::Painter::new(&clip_img);
    // e-clipPath-006.svg
    // e-clipPath-007.svg
    clip_p.set_transform(&p.get_transform());
    // e-clipPath-008.svg
    clip_p.apply_transform(&cp.transform.to_native());

    // e-clipPath-005.svg
    if cp.units == usvg::Units::ObjectBoundingBox {
        clip_p.apply_transform(&qt::Transform::from_bbox(bbox));
    }

    clip_p.set_composition_mode(qt::CompositionMode::CompositionMode_Clear);

    let ts = clip_p.get_transform();
    // e-clipPath-015.svg
    // e-clipPath-017.svg
    for node in node.children() {
        clip_p.apply_transform(&node.transform().to_native());

        match *node.borrow() {
            usvg::NodeKind::Path(ref path_node) => {
                path::draw(&node.tree(), path_node, opt, &clip_p);
            }
            usvg::NodeKind::Text(ref text) => {
                text::draw(&node.tree(), text, opt, &clip_p);
            }
            _ => {}
        }

        clip_p.set_transform(&ts);
    }

    clip_p.end();

    p.set_transform(&qt::Transform::default());
    p.set_composition_mode(qt::CompositionMode::CompositionMode_DestinationOut);
    p.draw_image(0.0, 0.0, &clip_img);
}
