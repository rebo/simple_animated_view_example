use comp_state::{do_once, get_state_with_topo_id, set_state_with_topo_id, use_istate, StateAccess};
use enclose::enclose as e;
use modulator::ModulatorEnv;
use seed::{prelude::*, *};
use slotmap::{DefaultKey, DenseSlotMap};
use std::cell::RefCell;
use std::rc::Rc;

use once_cell::unsync::Lazy;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
pub struct Anim {
    id: comp_state::topo::Id,
    name: String,
    from: f32,
    to: f32,
    property: String,
    html_element: Option<web_sys::HtmlElement>,
}

impl Anim {
    pub fn new<S: Into<String>>(
        name: S,
        id: comp_state::topo::Id,
        from: f32,
        to: f32,
        property: S,
    ) -> Anim {
        Anim {
            name: name.into(),
            id,
            from,
            to,
            property: property.into(),
            html_element: None,
        }
    }
}

thread_local! {
    static MODULATOR: Lazy<RefCell<ModulatorEnv<f32>>> = Lazy::new(|| {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let window = web_sys::window().expect("should have a window in this context");
        let mut maybe_performance: Option<f64> = None;
        let performance = window
            .performance()
            .expect("performance should be available");

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let new_now = performance.now();
            let delta = if let Some(old_now) = maybe_performance {
                (new_now - old_now)
            } else {
                0.0
            };
            maybe_performance = Some(new_now);
            if delta < 30.0 {
                MODULATOR.with(|m|
                    m.borrow_mut().advance((1000.0f64*delta) as u64)
                );
            }
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());

        RefCell::new(ModulatorEnv::<f32>::new())

    })
}

// static GLOBAL_DATA: Lazy<Arc<Mutex<ModulatorEnv<f32>>>> = Lazy::new(|| {
//     let mut m = ModulatorEnv::<f32>::new();
//     Arc::new(Mutex::new(m))
// });

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

type RcMutClosure = Rc<RefCell<Option<Closure<(dyn FnMut() + 'static)>>>>;

#[derive(Clone, Default)]
pub struct AnimationGroup {
    anims: DenseSlotMap<DefaultKey, Anim>,
    status: AnimStatus,
    closure: RcMutClosure,
}

#[derive(Clone, PartialEq)]
enum AnimStatus {
    PreInitialized,
    Initialized,
    Running,
    Complete,
}



impl std::default::Default for AnimStatus {
    fn default() -> Self {
        AnimStatus::PreInitialized
    }
}

pub trait AnimationGroupAccessTrait {
    fn trigger(&self);
     fn add_spring<T: Into<String>>(
        &self,
        from: f32,
        to: f32,
        property: T,
        units: T,
        ideal_time_secs: f32,
        undamp: f32,
    ) -> StateAccess<AnimationGroup>;
    fn dom_id(&self) -> String ;
}

impl AnimationGroupAccessTrait for StateAccess<AnimationGroup> {
    fn dom_id(&self) -> String {
        format!("{:#?}", self.id)
    }
    fn trigger(&self) {
        self.update(|anim_group| anim_group.status = AnimStatus::Running);
        let anim_group = self.hard_get();
        request_animation_frame(anim_group.closure.borrow().as_ref().unwrap());

        for anim in anim_group.anims.values() {
            MODULATOR.with(|m| {
                m.borrow_mut()
                    .get_mut(&anim.name)
                    .unwrap()
                    .set_enabled(true)
            });
        }
    }
     fn add_spring<T: Into<String>>(
            &self,
            from: f32,
            to: f32,
            property: T,
            units: T,
            ideal_time_secs: f32,
            undamp: f32,
        ) -> StateAccess<AnimationGroup> {
            let id = self.id;
            let property = property.into();
            let units = units.into();
            let name = format!("{:#?}{}", id, property);
            let anim_group = self.hard_get();

            if anim_group.status == AnimStatus::PreInitialized {
                
                self.update(|anim_group| {
    
                    let g = anim_group.closure.clone();
                    let anim_group_access = self.clone();
                    *g.borrow_mut() = Some(Closure::wrap(Box::new(e!( (anim_group_access,name) move || {
                        let mut anim_group = anim_group_access.hard_get();
                        if anim_group.status == AnimStatus::Running {
                            let mut remove_keys = vec![];
                            for (key, anim) in anim_group.anims.iter_mut() {
                                let val = MODULATOR.with(|m| m.borrow().value(&anim.name));
                                let interp = val * anim.from + (1.0-val) * anim.to;
                                if let Some(html_element) = &anim.html_element {
                                        let _ = html_element
                                            .style()
                                            .set_property(&anim.property, &format!("{}{}", interp,units));
                                    } else if let Some(generic_element) =
                                        document().get_element_by_id(&format!("{:#?}", anim.id))
                                    {
                                        if let Ok(html_element) =
                                            generic_element.dyn_into::<web_sys::HtmlElement>()
                                        {
                                            let _ = html_element.style().set_property(
                                                &anim.property,
                                                &format!("{}{}", interp,units),
                                            );
                                            anim.html_element = Some(html_element);
                                        }
                                    }
                                    let is_stopped = MODULATOR.with(|m|{
                                        let pos = m.borrow().value(&name);
                                        let vel = m.borrow_mut().get_mut(&name).unwrap().as_any().downcast_mut::<modulator::sources::ScalarSpring>().unwrap().vel;
                                         pos < 0.001 && vel < 0.001
                                    }
                                    );
                                    if  is_stopped {
                                        remove_keys.push(key);
                                        anim_group.status = AnimStatus::Complete;
                                        log!("removing");
                                    }
                                }
            
                                for key in remove_keys {
                                    anim_group_access.update(|anim_group| {anim_group.anims.remove(key);});
                                }
            
                                if anim_group_access.hard_get().anims.is_empty(){
                                        let _ = anim_group.closure.borrow_mut().take();
                                        log!("stopped anim");
                                        return;
                                }
                            }
            
                            request_animation_frame(anim_group.closure.borrow().as_ref().unwrap());
                        }
            
            
                    )) as Box<dyn FnMut()>));
        
        
                    anim_group.status = AnimStatus::Initialized}
                );
            }
        
            do_once(|| {
                self.update(e!((name,property)  |anim_group| {
                    let mut spring = modulator::sources::ScalarSpring::new(ideal_time_secs, undamp, 1.0);
                    
                    spring.enabled = false;
                    spring.spring_to(0.0);
                    MODULATOR.with(|m| m.borrow_mut().take(&name, Box::new(spring)));
        
                    anim_group
                        .anims
                        .insert(Anim::new(name, id, from, to, property));
                }))
            });
            self.clone()        

}}

pub fn use_spring<T: Into<String>>(
    from: f32,
    to: f32,
    property: T,
    units: T,
    ideal_time_secs: f32,
    undamp: f32,
) -> StateAccess<AnimationGroup> {

    let anim_group_access = use_istate(AnimationGroup::default).1;
    anim_group_access.add_spring( from, to, property.into(), units.into(), ideal_time_secs, undamp );

    anim_group_access
}