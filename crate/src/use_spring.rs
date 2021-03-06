#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::redundant_closure_for_method_calls)]

use comp_state::{do_once, use_istate, StateAccess};
use enclose::enclose as e;
use modulator::ModulatorEnv;
use once_cell::unsync::Lazy;
use regex::Regex;
use seed::{prelude::*, *};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;

#[derive(PartialEq)]
enum AMCCommand {
    DoNothing,
    Stop,
}
//
impl Default for AMCCommand {
    fn default() -> Self {
        Self::DoNothing
    }
}

#[derive(Clone, PartialEq)]
enum RafStatus {
    Running,
    Stopped,
}

impl Default for RafStatus {
    fn default() -> Self {
        Self::Stopped
    }
}

#[derive(Default)]
struct AnimationMasterControl {
    modulator_env: ModulatorEnv<f32>,
    properties: Vec<StateAccess<AnimProperty>>,
    raf_closure: RcMutClosure,
    command: AMCCommand,
    raf_status: RafStatus,
}

impl AnimationMasterControl {
    pub fn _stop(&mut self) {
        self.command = AMCCommand::Stop
    }
}

// const stringShapeRegex = /[+\-]?(?:0|[1-9]\d*)(?:\.\d*)?(?:[eE][+\-]?\d+)?/g
thread_local! {
    static REGEXP: Lazy<RefCell<Regex>> = Lazy::new(||{
        RefCell::new(Regex::new(r"([+\-]?(?:0|[1-9]\d*)(?:\.\d*)?(?:[eE][+\-]?\d+)?)").unwrap())
    })
}

fn amc_start_raf() {
    MODULATOR.with(|m| {
        request_animation_frame(
            m.borrow().raf_closure.borrow().as_ref().unwrap(),
        )
    });
}

fn amc_is_stopped(name: &str) -> bool {
    MODULATOR.with(|m| {
        let pos = m.borrow().modulator_env.value(name);
        let vel = m
            .borrow_mut()
            .modulator_env
            .get_mut(name)
            .unwrap()
            .as_any()
            .downcast_mut::<modulator::sources::ScalarSpring>()
            .unwrap()
            .vel;
        pos.abs() < 0.0001 && vel.abs() < 0.0001
    })
}

fn amc_contains(name: &str) -> bool {
    MODULATOR.with(|m| m.borrow().modulator_env.get(name).is_some())
}

fn amc_val_for_prop(name: &str) -> Option<f32> {
    if amc_contains(name) {
        Some(MODULATOR.with(|m| m.borrow().modulator_env.value(name.as_ref())))
    } else {
        None
    }
}

fn amc_advance(timestep: f64) {
    MODULATOR.with(|m| {
        m.borrow_mut().modulator_env.advance((1000.0_f64 * timestep) as u64)
    });
}

fn amc_properties() -> Vec<StateAccess<AnimProperty>> {
    MODULATOR.with(|m| m.borrow().properties.clone())
}

thread_local! {
    static MODULATOR: Lazy<RefCell<AnimationMasterControl>> = Lazy::new(|| {
        let amc_control = AnimationMasterControl::default();
        let g = amc_control.raf_closure.clone();
        let timestep = 1000.0/60.0;
        let mut possible_last_frame_timestamp : Option<f64> = None;

        let mut delta = 0.0;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp| {

            //  log!("1st Log");
            // Need to return without calling RAF again if requested to stop or no properties left.
            if MODULATOR.with(|m| m.borrow().command == AMCCommand::Stop || m.borrow().properties.is_empty()) {
                    possible_last_frame_timestamp = None;
                    MODULATOR.with(|m| m.borrow_mut().raf_status = RafStatus::Stopped);
                    return;
            }

            // If possible_last_frame_timestep is none, then this is the first run after a pause.
            // shedule a raf restart and return.
            if possible_last_frame_timestamp.is_none() {
                // log!("2nd Log");
                possible_last_frame_timestamp = Some(timestamp);
                MODULATOR.with(|m|
                    request_animation_frame(m.borrow().raf_closure.borrow().as_ref().unwrap())
                );
                return;
            }
            // log!("3rd Log");
            // From this point we know that timestep is_some.
            let last_frame_timestamp = possible_last_frame_timestamp.unwrap();
            // update section
            delta += timestamp - last_frame_timestamp; // note += here
            possible_last_frame_timestamp = Some(timestamp);
            // Simulate the total elapsed time in fixed-size chunks
            while delta >= timestep {
                // move everything according to this timestep. Therefore our timesteps are constantaa
                amc_advance(timestep);
                delta -= timestep;
            }


            // vec to store props that need to be removed
            let mut props_to_remove  = vec![];

            // itrate over props and divs inside props
            for (prop,prop_access) in amc_properties().iter().map(|p| (p.hard_get(), p)) {
                for (mut elem_control, elem_control_access) in prop.elem_control_accesses.iter().map(|d| (d.hard_get(),d)){
                    find_and_update_html_element(&mut elem_control, elem_control_access);
                    // if that element does exist:

                    update_div(&mut elem_control, &prop, prop_access.clone());
                    elem_control_access.set(elem_control);
                }

                if  amc_is_stopped(&prop.name) {
                    props_to_remove.push(prop_access.clone());
                    // log!("Animation Stopped");
                }
            }

            for (anim, anim_access) in props_to_remove.iter().map(|pa| (pa.hard_get(), pa)) {
                MODULATOR.with(|m|
                {
                    m.borrow_mut().properties.retain(|p| p.id != anim_access.id);
                    m.borrow_mut().modulator_env.kill(&anim.name);
                });
            }
           amc_start_raf();
        // request_animation_frame(amc_control.raf_closure.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f64)>));

        // request_animation_frame(g.borrow().as_ref().unwrap());

        RefCell::new(amc_control)
    })
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

type RcMutClosure = Rc<RefCell<Option<Closure<(dyn FnMut(f64) + 'static)>>>>;

use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct ElemControl {
    pub div_id: String,
    pub html_element: Option<web_sys::HtmlElement>,
    pub should_stop: bool,
    pub from_props: HashMap<String, FromProp>,
}

#[derive(Clone)]
pub struct FromProp {
    prop: String,
    vals: Vec<f32>,
    template: Vec<String>,
}

impl ElemControl {
    pub fn new<T: Into<String>>(div_id: T) -> Self {
        let mut dc = Self::default();
        dc.div_id = div_id.into();
        dc
    }
}

#[derive(Clone)]
pub struct AnimProperty {
    pub name: String,
    pub property: String,
    pub to: String,
    pub to_vals: Vec<f32>,
    pub to_template: Vec<String>,
    pub ideal_time: f32,
    pub map_func: RcRefCellFnBox,
    pub latest_prop_val: Option<String>,
    pub default_from: Option<String>,
    pub status: AnimPropertyStatus,
    pub elem_control_accesses: Vec<StateAccess<ElemControl>>,
    // pub spring: modulator::sources::ScalarSpring,
}

#[derive(Clone, PartialEq)]
pub enum AnimPropertyStatus {
    Stopped,
    Running,
}

pub fn use_spring<T: Into<String>>(
    property: T,
    to: T,
) -> StateAccess<AnimProperty> {
    let to = to.into();
    let (mut anim_prop, anim_prop_access) = comp_state::use_istate(e!((to)
        || AnimProperty {
            name: String::default(),
            property: property.into(),
            ideal_time: 0.2,
            to,
            latest_prop_val: None,
            map_func: Rc::new(RefCell::new(None)),
            to_vals: vec![],
            to_template: vec![],
            status: AnimPropertyStatus::Stopped,
            default_from: None,
            elem_control_accesses: vec![],
        }));

    if anim_prop.to != to {
        anim_prop.to = to
    }

    anim_prop.name = format!("{:#?}", anim_prop_access.id);
    anim_prop_access.set(anim_prop);
    anim_prop_access
}

pub trait AnimPropertyAccessTrait {
    fn set_property<T: Into<String>>(&self, to: T) -> Self;
    fn default_from<T: Into<String>>(&self, from: T) -> Self;
    fn start(&self);
    fn preignite(&self) -> Self;
    fn ideal_time(&self, time: f32) -> Self;
    fn latest_prop_val(&self) -> Option<String>;
    fn map<F: Fn(f32) -> f32 + 'static>(
        &self,
        map_func: F,
    ) -> StateAccess<AnimProperty>;
}

type RcRefCellFnBox = Rc<RefCell<Option<Box<(dyn Fn(f32) -> f32 + 'static)>>>>;

impl AnimPropertyAccessTrait for StateAccess<AnimProperty> {
    fn map<F: Fn(f32) -> f32 + 'static>(
        &self,
        map_func: F,
    ) -> StateAccess<AnimProperty> {
        let existing_prop = self.hard_get();

        let (mut anim_prop, anim_prop_access) =
            comp_state::use_istate(|| AnimProperty {
                name: String::default(),
                property: existing_prop.name.clone(),
                ideal_time: 0.1,
                to: existing_prop.to.clone(),
                latest_prop_val: None,
                to_vals: vec![],
                to_template: vec![],
                map_func: Rc::new(RefCell::new(Some(
                    Box::new(map_func) as Box<dyn Fn(f32) -> f32>
                ))),
                status: AnimPropertyStatus::Stopped,
                default_from: None,
                elem_control_accesses: vec![],
            });

        anim_prop.name = format!("{:#?}", anim_prop_access.id);
        anim_prop_access.set(anim_prop);
        anim_prop_access
    }

    fn default_from<T: Into<String>>(&self, from: T) -> Self {
        self.update(|anim| anim.default_from = Some(from.into()));
        self.clone()
    }

    fn latest_prop_val(&self) -> Option<String> {
        self.hard_get().latest_prop_val
    }

    fn ideal_time(&self, time: f32) -> Self {
        self.update(|anim| anim.ideal_time = time);
        self.clone()
    }

    fn set_property<T: Into<String>>(&self, to: T) -> Self {
        self.update(|anim| anim.to = to.into());
        self.clone()
    }

    fn preignite(&self) -> Self {
        self.clone()
    }

    fn start(&self) {
        let anim_prop = self.hard_get();
        // we need to clear all existing "froms" and start the animation again from a new from value.
        for elem_control_access in &anim_prop.elem_control_accesses {
            elem_control_access.update(|elem_control| {
                elem_control.from_props.remove(&anim_prop.property);
            });
        }

        // create a new spring
        let mut spring = modulator::sources::ScalarSpring::new(
            anim_prop.ideal_time,
            5.5,
            1.0,
        );
        spring.spring_to(0.0);

        // tear down the modulator associated with this property and re-initialise it
        MODULATOR.with(|m| {
            let mut amc = m.borrow_mut();
            amc.modulator_env.kill(&anim_prop.name);
            amc.modulator_env.take(&anim_prop.name, Box::new(spring));
            if !amc.properties.iter().any(|p| p.id == self.id) {
                amc.properties.push(self.clone());
            }
            // if the raf loop is currently stopped we need to restart it.
            if amc.raf_status == RafStatus::Stopped {
                request_animation_frame(
                    amc.raf_closure.borrow().as_ref().unwrap(),
                );
                amc.raf_status = RafStatus::Running;
            }
        });

        self.update(|anim_prop| {
            anim_prop.to_vals = REGEXP.with(|reg| {
                reg.borrow()
                    .captures_iter(&anim_prop.to)
                    .map(|c| c[0].to_string().parse::<f32>().unwrap())
                    .collect::<Vec<f32>>()
            });
            anim_prop.to_template = REGEXP.with(|reg| {
                reg.borrow()
                    .split(&anim_prop.to)
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            });
            anim_prop.status = AnimPropertyStatus::Running
        });
    }
}

pub fn animated_id<T: Into<String>>(
    div_id: T,
    properties: &[(StateAccess<AnimProperty>, &str)],
) -> (seed::Attrs, seed::Style) {
    let (div, elem_control_access) = use_istate(|| ElemControl::new(div_id));

    do_once(|| {
        for prop_access in properties.iter().map(|p| &p.0) {
            prop_access.update(e!((elem_control_access) | prop | {
                prop.elem_control_accesses.push(elem_control_access)
            }));
        }
    });
    let mut style = seed::Style::empty();
    for (prop_access, default) in properties.iter() {
        let prop = prop_access.hard_get();

        if let Some(new_prop) = prop.latest_prop_val {
            style.add(prop.property, new_prop)
        } else {
            style.add(prop.property, default)
        }
    }

    // log!(style);
    (id!(div.div_id), style)
}

fn find_and_update_html_element(
    elem_control: &mut ElemControl,
    elem_control_access: &StateAccess<ElemControl>,
) {
    if elem_control.html_element.is_none() {
        if let Some(Ok(html_element)) = document()
            .get_element_by_id(&elem_control.div_id)
            .map(wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>)
        {
            elem_control.html_element = Some(html_element);
            elem_control_access.set(elem_control.clone());
        }
    }
}

fn update_div(
    elem_control: &mut ElemControl,
    prop: &AnimProperty,
    prop_access: StateAccess<AnimProperty>,
) {
    if let Some(html_element) = &elem_control.html_element {
        // ensure element control has currently  a set from property.
        if elem_control.from_props.get(&prop.property).is_none() {
            if let Ok(existing_from) =
                html_element.style().get_property_value(&prop.property)
            {
                if !existing_from.is_empty() {
                    let template = REGEXP.with(|reg| {
                        reg.borrow()
                            .split(&existing_from)
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                    });
                    let vals = REGEXP.with(|reg| {
                        reg.borrow()
                            .captures_iter(&existing_from)
                            .map(|c| c[0].parse::<f32>().unwrap())
                            .collect::<Vec<f32>>()
                    });
                    let from_prop = FromProp {
                        prop: existing_from,
                        vals,
                        template,
                    };

                    elem_control
                        .from_props
                        .insert(prop.property.clone(), from_prop);
                }
            }
        }

        // if there is a from property, then animate it.
        if let Some(interpolated_prop) = new_prop_for_elem(elem_control, prop) {
            // log!(interpolated_prop);
            //  prop.latest_prop_val = Some(interpolated_prop.clone());
            prop_access.update(|p| {
                p.latest_prop_val = Some(interpolated_prop.clone())
            });
            let _ = html_element
                .style()
                .set_property(&prop.property, &interpolated_prop);
        }
    }
}

fn new_prop_for_elem(
    elem_control: &ElemControl,
    prop: &AnimProperty,
) -> Option<String> {
    let from_prop = &elem_control.from_props.get(&prop.property)?;
    let val = amc_val_for_prop(&prop.name)?;

    let interpolated_vals = from_prop
        .vals
        .iter()
        .zip(prop.to_vals.iter())
        .map(|(from, to)| to.mul_add(1.0 - val, from * val))
        .collect::<Vec<f32>>();

    let mut idx = 0;

    Some(
        REGEXP
            .with(|reg| {
                reg.borrow().replace_all(&prop.to, |_: &regex::Captures| {
                    idx += 1;
                    if let Some(val) = interpolated_vals.get(idx - 1) {
                        val.to_string()
                    } else {
                        " ".to_string()
                    }
                })
            })
            .to_string(),
    )
}

use seed::virtual_dom::UpdateEl;
pub trait UpdateElLocal<T> {
    fn update(self, el: &mut T);
}

impl<Ms> UpdateElLocal<El<Ms>> for (seed::Attrs, seed::Style) {
    fn update(self, el: &mut El<Ms>) {
        self.0.update(el);
        self.1.update(el);
    }
}
