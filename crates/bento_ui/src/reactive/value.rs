pub enum Reactive<T: 'static> {
    Static(T),
    Dynamic(Box<dyn Fn() -> T>),
}

impl<T: Copy + 'static> Reactive<T> {
    pub fn get(&self) -> T {
        match self {
            Reactive::Static(v) => *v,
            Reactive::Dynamic(f) => f(),
        }
    }
}

impl<T: Clone + 'static> Reactive<T> {
    pub fn get_clone(&self) -> T {
        match self {
            Reactive::Static(v) => v.clone(),
            Reactive::Dynamic(f) => f(),
        }
    }
}

impl From<String> for Reactive<String> {
    fn from(v: String) -> Self { Reactive::Static(v) }
}
impl<F: Fn() -> String + 'static> From<F> for Reactive<String> {
    fn from(f: F) -> Self { Reactive::Dynamic(Box::new(f)) }
}
impl From<&'static str> for Reactive<String> {
    fn from(v: &'static str) -> Self { Reactive::Static(v.to_string()) }
}

impl From<[f32; 4]> for Reactive<[f32; 4]> {
    fn from(v: [f32; 4]) -> Self { Reactive::Static(v) }
}
impl<F: Fn() -> [f32; 4] + 'static> From<F> for Reactive<[f32; 4]> {
    fn from(f: F) -> Self { Reactive::Dynamic(Box::new(f)) }
}
