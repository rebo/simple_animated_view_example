use super::Msg;
use crate::generated::css_classes::C;

use crate::{
    use_spring::{use_spring, AnimationGroupAccessTrait},
    use_spring2,
};
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

pub fn view() -> Node<Msg> {
    // the property values are needed incase there is a re-render mid animation
    //
    // ideally would like:
    // use_spring("margin-left: 300px");
    //
    //
    let (margin_left, anim_group) =
        use_spring(20.0, 200.0, "margin-left", "px", 0.1, 0.0);
    let padding_top =
        anim_group.add_spring(20.0, 200.0, "padding-top", "px", 0.1, 0.0);
    let padding_bottom =
        anim_group.add_spring(20.0, 200.0, "padding-bottom", "px", 0.1, 0.0);
    let margin_right =
        anim_group.add_spring(20.0, 200.0, "margin-right", "px", 0.1, 0.0);

    anim_group.on_complete_msg::<Msg, super::Model>(Msg::AnimationComplete);

    div![
        div![
            class![C.p_3, C.bg_gray_8, C.rounded, C.shadow_lg, C.m_2],
            style![St::MarginLeft => margin_left, St::PaddingTop => padding_top, St::PaddingBottom => padding_bottom, St::MarginRight => margin_right],
            id!(anim_group.dom_id()),
            button!(
                "This will shrink on the left...",
                on_click(move |_| anim_group.trigger())
            )
        ],
        div![
            {
                // let unique_div_id = comp_state::topo::call!(comp_state::topo::Id::current());
                use_spring2::register_div_for_updates("foo");
                id!("foo")
            },
            class![C.p_3, C.bg_gray_8, C.rounded, C.shadow_lg, C.m_2],
            button!(
                "click 2...",
                // on_click(move |_| .trigger())
            )
        ]
    ]
}
