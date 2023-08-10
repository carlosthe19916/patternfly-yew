use crate::components::accordion::styles::*;
use std::fmt::Debug;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AccordionToggleProperties {
    /** Content rendered inside the Accordion toggle  */
    #[prop_or_default]
    pub children: Children,

    /** Additional classes added to the Accordion Toggle  */
    #[prop_or_default]
    pub class: Classes,

    /** Flag to show if the expanded content of the Accordion item is visible  */
    #[prop_or_default]
    pub expanded: bool,

    /** Identify the Accordion toggle number  */
    #[prop_or_default]
    pub id: AttrValue,
}

#[function_component(AccordionToggle)]
pub fn accordion_toggle(props: &AccordionToggleProperties) -> Html {
    html! (
        <div

        >

        </div>
    )
}
