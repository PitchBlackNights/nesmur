pub struct Clock {
    pub frequency: f32,
    pub cycle: u64,
    pub ns_per_tick: u32,
}

pub struct ChildClock {
    pub divisor: u16,
    pub tick: Box<dyn Fn() + 'static>,
}

impl ChildClock {
    pub fn new<F>(divisor: u16, tick: F) -> Self 
    where
        F: Fn() + 'static,
    {
        ChildClock {
            divisor,
            tick: Box::new(tick),
        }
    }
}