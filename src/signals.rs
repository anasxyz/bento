#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Signal {
    Click,
    Hover,
    HoverEnd,
    Press,
    Release,
    FocusGained,
    FocusLost,
}
