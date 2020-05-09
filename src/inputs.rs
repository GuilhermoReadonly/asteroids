/// The `InputState` is exactly what it sounds like, it just keeps track of
/// the user's input state so that we turn keyboard events into something
/// state-based and device-independent.
#[derive(Debug, Clone)]
pub struct InputState {
    pub xaxis: XDirection,
    pub yaxis: YDirection,
    pub fire: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum XDirection {
    Left,
    Right,
    XCenter,
}

#[derive(Debug, Clone, PartialEq)]
pub enum YDirection {
    Forward,
    Backward,
    YCenter,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: XDirection::XCenter,
            yaxis: YDirection::YCenter,
            fire: false,
        }
    }
}
