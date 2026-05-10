use std::collections::HashMap;

pub struct Base {
    pub dirty: bool,
    pub delta: f32,
    pub animations: HashMap<&'static str, Animation>,
    pub transitions: HashMap<&'static str, (f32, Easing)>,
    pub default_transition: Option<(f32, Easing)>,
}

impl Base {
    pub fn new() -> Self {
        Self {
            dirty: true,
            delta: 0.0,
            animations: HashMap::new(),
            transitions: HashMap::new(),
            default_transition: None,
        }
    }

    pub fn tick(&mut self) -> bool {
        let delta = self.delta;
        let mut any_running = false;
        for anim in self.animations.values_mut() {
            if anim.elapsed == 0.0 {
                if let Some(cb) = &anim.on_start {
                    cb();
                }
            }
            anim.elapsed += delta;
            if let Some(cb) = &anim.on_tick {
                cb((anim.elapsed / anim.duration).clamp(0.0, 1.0));
            }
            match anim.loop_mode {
                LoopMode::Once => {
                    if anim.elapsed < anim.duration {
                        any_running = true;
                    } else {
                        anim.elapsed = anim.duration;
                        if let Some(cb) = &anim.on_complete {
                            cb();
                        }
                    }
                }
                LoopMode::Loop => {
                    anim.elapsed %= anim.duration;
                    any_running = true;
                }
                LoopMode::PingPong => {
                    if anim.elapsed >= anim.duration * 2.0 {
                        anim.elapsed -= anim.duration * 2.0;
                    }
                    any_running = true;
                }
            }
        }
        any_running
    }

    pub fn animated_value(&self, field: &'static str, current: AnimatableValue) -> AnimatableValue {
        match self.animations.get(field) {
            Some(anim) => {
                let t = (anim.elapsed / anim.duration).clamp(0.0, 1.0);
                let t = match anim.loop_mode {
                    LoopMode::PingPong => {
                        let cycle = anim.elapsed / anim.duration;
                        let t = cycle.fract();
                        if cycle as u32 % 2 == 0 { t } else { 1.0 - t }
                    }
                    _ => t,
                };
                let t = anim.easing.apply(t);
                match (&anim.from, &anim.to) {
                    (AnimatableValue::Float(a), AnimatableValue::Float(b)) => {
                        AnimatableValue::Float(a + (b - a) * t)
                    }
                    (AnimatableValue::Color(a), AnimatableValue::Color(b)) => {
                        AnimatableValue::Color([
                            a[0] + (b[0] - a[0]) * t,
                            a[1] + (b[1] - a[1]) * t,
                            a[2] + (b[2] - a[2]) * t,
                            a[3] + (b[3] - a[3]) * t,
                        ])
                    }
                    _ => current,
                }
            }
            None => current,
        }
    }

    pub fn stop_animation(&mut self, field: &'static str) {
        self.animations.remove(field);
        self.dirty = true;
    }
}

impl Default for Base {
    fn default() -> Self {
        Self::new()
    }
}

pub trait HasBase {
    fn base(&self) -> &Base;
    fn base_mut(&mut self) -> &mut Base;
    fn pre_update(&mut self) {}
}

#[derive(Copy, Clone)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Easing {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => t * (2.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum LoopMode {
    Once,
    Loop,
    PingPong,
}

pub enum AnimatableValue {
    Float(f32),
    Color([f32; 4]),
}

pub struct Animation {
    pub from: AnimatableValue,
    pub to: AnimatableValue,
    pub duration: f32,
    pub elapsed: f32,
    pub easing: Easing,
    pub loop_mode: LoopMode,
    pub on_start: Option<Box<dyn Fn()>>,
    pub on_tick: Option<Box<dyn Fn(f32)>>,
    pub on_complete: Option<Box<dyn Fn()>>,
}
