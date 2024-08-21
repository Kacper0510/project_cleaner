#[derive(Debug, PartialEq, Clone)]
pub enum PopUpState {
    Open(PopUpKind),
    Closed,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PopUpKind {
    Info,
    Delete(DeletePopUpKind),
    Exit,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeletePopUpKind {
    Confirm,
    Deleting,
}