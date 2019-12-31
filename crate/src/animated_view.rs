use super::Msg;
use crate::generated::css_classes::C;

use crate::use_spring::{animated_id, use_spring, AnimPropertyAccessTrait};
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

pub fn view() -> Node<Msg> {
    let margin_left = use_spring("margin-left", "344px");
    let margin_top = use_spring("margin-top", "344px");
    let color = use_spring("background-color", "rgb(0, 100, 200)");
    div![
        class![C.w_20, C.p_3, C.bg_gray_8, C.rounded, C.shadow_lg, C.m_2],
        animated_id(
            "animated_div",
            &[margin_left.clone(), margin_top.clone(), color.clone()]
        ),
        button!(
            class![C.focus__outline_none],
            "Click me",
            on_click(move |_| {
                margin_left
                    .set_property(format!("{}px", js_sys::Math::random() * 500.0))
                    .start();
                margin_top
                    .set_property(format!("{}px ", js_sys::Math::random() * 500.0))
                    .start();
                color
                    .set_property(format!(
                        "rgb({}, {}, {})",
                        js_sys::Math::random() * 256.0,
                        js_sys::Math::random() * 256.0,
                        js_sys::Math::random() * 256.0
                    ))
                    .start();
            })
        )
    ]
}
