#[derive(Debug, Clone, PartialEq, Default)]
pub enum Cursor {
    #[default]
    Default,
    Pointer,
    Text,
    Grab,
    Grabbing,
    Move,
    NotAllowed,
    CrossHair,
    Wait,
    Help,
}
