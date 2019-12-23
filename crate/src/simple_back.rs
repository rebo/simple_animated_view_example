#![allow(non_snake_case)]

use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;
// use std::any::Any;

use super::Msg;
use comp_state::get_state_with_topo_id as gs;
use comp_state::set_state as ss_here;
use comp_state::set_state_with_topo_id as ss;
use comp_state::{topo, use_istate, use_state, StateAccess};

use enclose::enclose as e;
use seed::dom_types::UpdateEl;
use seed_comp_helpers::on_input;
pub fn view() -> Node<Msg> {
    let bear1 = create_bear();
    let bear2 = create_bear();

    // bear1.attack(bear2);
    div![]
}

#[derive(Clone)]
enum EntityClass {
    Animal,
}

#[derive(Clone)]
enum WeaponClass {
    Natural,
}

#[derive(Clone)]
struct Attack(u32);

#[derive(Clone)]
struct Health(u32);

#[derive(Clone)]
struct Armour(u32);

#[derive(Clone)]
struct EntityName(String);

fn create_bear() -> topo::Id {
    topo::call!({
        ss_here(EntityName("Bear".to_string()));
        ss_here(Attack(100));
        ss_here(Armour(50));
        ss_here(Health(50));
        ss_here(EntityClass::Animal);
        topo::Id::current()
    })
}

fn attack(attacker: topo::Id, target: topo::Id) {
    if let (Some(attack), Some(target_armour), Some(target_health)) = (
        gs::<Attack>(attacker),
        gs::<Armour>(target),
        gs::<Health>(target),
    ) {
        ss(
            Health(target_health.0 - (attack.0 - target_armour.0)),
            target,
        );
    }
}

fn runloop() {
    loop {
        topo::call!({
            process_input();
            draw_canvas();
        })
    }
}

// let (_, health_accessor) = use_istate(|| Attack(100));
// // let (_, armour_accessor) = use_istate(|| Armour(50));
// // let (_, class_accessor) = use_istate(|| EntityClass::Animal);
// // claws();
// let mut v = vec![];
// v.push(Box::new(health_accessor));
// // v.push(Box::new(armour_accessor));
// // v.push(Box::new(class_accessor));
// v
// }

// pub fn claws() {
//     topo::call!({
//         let (_, attack_accessor) = use_istate(|| 25);
//         let (_, attack_class_accessor) = use_istate(|| WeaponClass::Natural);
//     });
// }
