#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vision {
    pub grants_vision: bool,
}

impl Vision {
    pub fn new(active: bool) -> Vision {
        return Vision {
            grants_vision: active,
        };
    }
}
