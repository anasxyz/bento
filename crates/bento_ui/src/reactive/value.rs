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
