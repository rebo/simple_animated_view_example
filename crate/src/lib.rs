#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::clippy::missing_const_for_fn)]
mod generated;

mod animated_view;
// mod use_spring;
mod use_spring;
// #[macro_use]
// extern crate shrinkwraprs;
// #[macro_use]
// extern crate derive_more;
// use comp_state::topo;
use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------
#[derive(Default)]
pub struct Model {}

// ------ ------
//     Init 
// ------ ------

// type AppType = seed::App<Msg, Model, Node<Msg>>;
fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    seed_comp_helpers::init::<Msg, Model, _>(orders);
    if let Some(mount_point_element) = document().get_element_by_id("app") {
        mount_point_element.set_inner_html("");
    }

    AfterMount::new(Model::default())
}

// pub fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Init<Model> {
//     seed_comp_helpers::init::<Msg, Model, _>(orders);

//     if let Some(mount_point_element) = document().get_element_by_id("app") {
//         mount_point_element.set_inner_html("");
//     }
//     Init::new(Model::default())
// }
// ------ ------
//    Routes
// ------ ------

pub fn routes(_url: Url) -> Option<Msg> {
    None
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    // DragEnter,
    // DragOver,
    // DragLeave,
    // Drop,
    AnimationComplete,
    DoNothing,
}

impl Default for Msg {
    fn default() -> Self {
        Self::DoNothing
    }
}

// ------ ------
//     View
// ------ ------

// Notes:
// - \u{00A0} is the non-breaking space
//   - https://codepoints.net/U+00A0
//
// - "▶\u{fe0e}" - \u{fe0e} is the variation selector, it prevents ▶ to change to emoji in some browsers
//   - https://codepoints.net/U+FE0E

pub fn view(_model: &Model) -> impl View<Msg> {
    comp_state::topo::call!(animated_view::view())
}

pub fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AnimationComplete => {
            log!("Animation complete callback seed!");
        }
        Msg::DoNothing => {}
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    seed::App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();

    log!("App started.");
}
