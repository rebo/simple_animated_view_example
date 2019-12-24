use super::Msg;
use crate::generated::css_classes::C;

use crate::use_spring::{use_prop_spring, AnimationGroupAccessTrait};
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

pub fn view() -> Node<Msg> {
    // the property values are needed incase there is a re-render mid animation
    let (margin_left, anim_group) = use_prop_spring(20.0, 200.0, "margin-left", "px", 0.1, 0.0);
    let padding_top = anim_group.add_prop_spring(20.0, 200.0, "padding-top", "px", 0.1, 0.0);
    let padding_bottom = anim_group.add_prop_spring(20.0, 200.0, "padding-bottom", "px", 0.1, 0.0);
    let margin_right = anim_group.add_prop_spring(20.0, 200.0, "margin-right", "px", 0.1, 0.0);

    anim_group.on_complete_msg::<Msg, super::Model>(Msg::AnimationComplete);

    div![
        class![C.p_3, C.bg_gray_8, C.rounded, C.shadow_lg, C.m_2],
        style![St::MarginLeft => margin_left, St::PaddingTop => padding_top, St::PaddingBottom => padding_bottom, St::MarginRight => margin_right],
        id!(anim_group.dom_id()),
        button!(
            "This will shrink on the left...",
            on_click(move |_| anim_group.trigger())
        )
    ]
}
