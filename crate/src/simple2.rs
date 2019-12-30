use super::Msg;
use crate::generated::css_classes::C;

use crate::use_spring2::{animated_id, use_spring, AnimPropertyAccessTrait};
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

pub fn view() -> Node<Msg> {
    let padding_anim = use_spring("padding-top", "344px");

    div![
        class![C.pt_1, C.p_3, C.bg_gray_8, C.rounded, C.shadow_lg, C.m_2],
        animated_id("animated_div", &[padding_anim.clone()]),
        button!("Click me", on_click(move |_| padding_anim.start()))
    ]
}
