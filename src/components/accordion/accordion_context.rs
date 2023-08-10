use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct AccordionContextProps {
    pub content_container: String,
    pub toggle_container: String,
}
