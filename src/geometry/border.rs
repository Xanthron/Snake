#[derive(Copy, Clone)]
pub struct Border {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Border {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Border {
        return Border {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        };
    }
}
