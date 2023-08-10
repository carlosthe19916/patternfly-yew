use crate::attr;
use crate::carlos;
use crate::components::accordion::accordion_context::*;
use crate::components::accordion::styles::*;
use std::fmt::Debug;
use yew::prelude::*;

use super::accordion;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AccordionContentProperties {
    /** Content rendered inside the Accordion  */
    #[prop_or_default]
    pub children: Children,

    /** Additional classes added to the Accordion content  */
    #[prop_or_default]
    pub class: Classes,

    /** Identify the AccordionContent item  */
    #[prop_or_default]
    pub id: AttrValue,

    /** Flag to show if the expanded content of the Accordion item is visible  */
    #[prop_or_default]
    pub hidden: bool,

    /** Flag to indicate Accordion content is fixed  */
    #[prop_or_default]
    pub fixed: bool,

    /** Adds accessible text to the Accordion content */
    #[prop_or_default]
    pub aria_label: AttrValue,

    /** Id of the controlling accordion toggle to label the content. */
    #[prop_or_default]
    pub aria_labelledby: AttrValue,

    /** Component to use as content container */
    #[prop_or_default]
    pub component: Option<String>,

    /** Flag to indicate Accordion content is fixed  */
    #[prop_or_default]
    pub custom_content: bool,
}

#[function_component(AccordionContent)]
pub fn accordion_content(props: &AccordionContentProperties) -> Html {
    let accordion_context = use_context::<AccordionContextProps>();

    let container = if let Some(component) = &props.component {
        component
    } else if let Some(accordion) = &accordion_context {
        &accordion.toggle_container
    } else {
        panic!()
    };

    html! (
        <@{container.clone()}>
            <button
                id={&props.id}
                class={classes!(
                    AccordionStyles::ACCORDION_EXPANDABLE_CONTENT,
                    conditional!(props.fixed, AccordionStyles::MODIFIERS_FIXED),
                    conditional!(!props.hidden, AccordionStyles::MODIFIERS_EXPANDED),
                    props.class.clone()
                )}
                hidden={props.hidden}
                aria-label={&props.aria_label}
                aria-labelledby={&props.aria_labelledby}
            >
            </button>
        </@>
    )
}
