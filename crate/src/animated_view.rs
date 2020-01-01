#![allow(clippy::too_many_lines)]
use super::Msg;
use crate::{
    generated::css_classes::C,
    use_spring::{
        animated_id, use_spring, AnimPropertyAccessTrait, UpdateElLocal,
    },
};
use comp_state::use_istate;
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

pub fn view() -> Node<Msg> {
    let (flipped, flipped_access) = use_istate(|| false);

    let opacity_val = if flipped {
        "0.0"
    } else {
        "1.0"
    };
    let opacity_val_opp = if flipped {
        "1.0"
    } else {
        "0.0"
    };
    let opacity = use_spring("opacity", opacity_val);
    let opacity_opposite = use_spring("opacity", opacity_val_opp);

    let transform_val = if flipped {
        "180.0"
    } else {
        "360.0"
    };
    let transform_val_opp = if flipped {
        "0.0"
    } else {
        "180.0"
    };

    let transform_string =
        format!("perspective(600px) rotateX({}deg)", transform_val);
    let transform_string_opp =
        format!("perspective(600px) rotateX({}deg)", transform_val_opp);

    let transform = use_spring("transform", &transform_string);
    let transform_opposite = use_spring("transform", &transform_string_opp);
    // let opacity_opposite = opacity.map(|v| 1.0 - v);
    div![
        class![
            C.flex,
            C.items_center,
            C.justify_center,
            C.h_screen,
            C.w_screen
        ],
        div![
            class![C.flex, C.relative, C.h_50vh, C.w_70vw,],
            div![
                class![
                    C.absolute,
                    C.flex,
                    C.items_end,
                    C.justify_start,
                    C.left_0,
                    C.top_0,
                    C.h_50vh,
                    C.w_70vw,
                    C.bg_gray_3,
                    C.shadow_lg,
                    C.rounded,
                    C.overflow_hidden,
                ],
                // style![St::Opacity => 0.0, St::Transform => "perspective(600px) rotateX(180deg)"  ],
                img![
                    class![
                        C.absolute,
                        C.left_0,
                        C.top_0,
                        C.object_cover,
                        C.h_50vh,
                        C.w_70vw,
                    ],
                    attrs![At::Src=>"static/images/front.jpg"]
                ],
                div![
                    class![
                        C.opacity_50,
                        C.w_70vw,
                        C.pl_1,
                        C.pr_1,
                        C.bg_black,
                        C.text_white,
                        C.absolute,
                        C.flex,
                        C.self_start,
                        C.text_20
                    ],
                    "Seed Rocks!"
                ],
                animated_id(
                    "front",
                    &[
                        (opacity.clone(), "0.0"),
                        (
                            transform.clone(),
                            "perspective(600px) rotateX(180deg)"
                        )
                    ]
                )
            ],
            div![
                class![
                    C.flex,
                    C.items_end,
                    C.justify_end,
                    C.absolute,
                    C.left_0,
                    C.top_0,
                    C.h_50vh,
                    C.w_70vw,
                    C.bg_gray_3,
                    C.shadow_lg,
                    C.rounded,
                    C.overflow_hidden,
                ],
                // style![St::Opacity => 1.0, St::Transform => "perspective(600px) rotateX(0deg)"  ],
                img![
                    class![
                        C.absolute,
                        C.left_0,
                        C.top_0,
                        C.object_cover,
                        C.h_50vh,
                        C.w_70vw,
                    ],
                    attrs![At::Src=>"static/images/back.jpg"]
                ],
                div![
                    class![
                        C.opacity_50,
                        C.w_70vw,
                        C.pl_1,
                        C.pr_1,
                        C.bg_black,
                        C.text_white,
                        C.absolute,
                        C.flex,
                        C.self_end,
                        C.text_20
                    ],
                    "Seed Rocks!"
                ],
                animated_id(
                    "back",
                    &[
                        (opacity_opposite.clone(), "1.0"),
                        (
                            transform_opposite.clone(),
                            "perspective(600px) rotateX(0deg)"
                        )
                    ]
                )
            ],
            on_click(move |_| {
                flipped_access.set(!flipped);
                opacity.start();
                opacity_opposite.start();
                transform.start();
                transform_opposite.start();
            }),
        ]
    ]
}

// pub fn view() -> Node<Msg> {
//     let margin_left = use_spring("margin-left", "344px");
//     let margin_top = use_spring("margin-top", "344px");
//     let color = use_spring("background-color", "rgb(0, 100, 200)");
//     div![
//         class![C.w_20, C.p_3, C.bg_gray_8, C.rounded, C.shadow_lg, C.m_2],
//         animated_id(
//             "animated_div",
//             &[margin_left.clone(), margin_top.clone(), color.clone()]
//         ),
//         style![St::MarginLeft => "0px", St::MarginTop=> "0px", St::BackgroundColor => "rgb(200, 200, 200)" ],
//         button!(
//             class![C.focus__outline_none],
//             "Click me",
//             on_click(move |_| {
//                 margin_left
//                     .set_property(format!("{}px", js_sys::Math::random() * 500.0))
//                     .start();
//                 margin_top
//                     .set_property(format!("{}px ", js_sys::Math::random() * 500.0))
//                     .start();
//                 color
//                     .set_property(format!(
//                         "rgb({}, {}, {})",
//                         js_sys::Math::random() * 256.0,
//                         js_sys::Math::random() * 256.0,
//                         js_sys::Math::random() * 256.0
//                     ))
//                     .start();
//             })
//         )
//     ]
// }
