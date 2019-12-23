use super::Msg;
use crate::generated::css_classes::C;

use crate::use_spring::{use_spring, AnimationGroupAccessTrait};
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

pub fn view() -> Node<Msg> {
    let anim_group = use_spring(20.0, 200.0, "margin-left", "px", 0.1, 0.0);
    anim_group.add_spring(20.0, 200.0, "padding-top", "px", 0.1, 0.0);
    anim_group.add_spring(20.0, 200.0, "padding-bottom", "px", 0.1, 0.0);
    anim_group.add_spring(20.0, 200.0, "margin-right", "px", 0.1, 0.0);

    div![
        class![C.p_3, C.bg_gray_8, C.rounded, C.shadow_lg, C.m_2],
        id!(anim_group.dom_id()),
        button!(
            "This will shrink on the left...",
            on_click(move |_| anim_group.trigger())
        )
    ]
}
