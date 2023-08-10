use crate::components::accordion::accordion_context::*;
use crate::components::accordion::styles::*;
use std::fmt::Debug;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AccordionHeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl AccordionHeadingLevel {
    pub fn level(&self) -> &str {
        return match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AccordionDisplaySize {
    DEFAULT,
    LG,
}

impl AccordionDisplaySize {
    pub fn classes(&self) -> &str {
        return match self {
            Self::DEFAULT => "",
            Self::LG => AccordionStyles::MODIFIERS_DISPLAY_LG,
        };
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AccordionProperties {
    /** Content rendered inside the Accordion  */
    #[prop_or_default]
    pub children: Children,

    /** Additional classes added to the Accordion  */
    #[prop_or_default]
    pub class: Classes,

    /** Adds accessible text to the Accordion */
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /** Modifies empty state max-width and sizes of icon, title and body */
    #[prop_or(AccordionHeadingLevel::H3)]
    pub heading_level: AccordionHeadingLevel,

    /** Cause component to consume the available height of its container */
    #[prop_or_default]
    pub full_height: bool,

    /** Flag to indicate whether use definition list or div */
    #[prop_or(true)]
    pub as_definition_list: bool,

    /** Flag to indicate the accordion had a border */
    #[prop_or_default]
    pub bordered: bool,

    /** Flag to indicate the accordion had a border */
    #[prop_or(AccordionDisplaySize::DEFAULT)]
    pub display_size: AccordionDisplaySize,
}

#[function_component(Accordion)]
pub fn accordion(props: &AccordionProperties) -> Html {
    let accordion_list = if props.as_definition_list {
        "dl"
    } else {
        "div"
    };

    let role = if !props.as_definition_list && props.aria_label.is_some() {
        Some("region")
    } else {
        None
    };

    // Context definition
    let content_container = if props.as_definition_list {
        "dd"
    } else {
        "div"
    };
    let toggle_container = if props.as_definition_list {
        "dt"
    } else {
        props.heading_level.level()
    };
    let accordion_context = AccordionContextProps {
        content_container: content_container.to_string(),
        toggle_container: toggle_container.to_string(),
    };

    html! (
        <@{accordion_list}
            class={classes!(
                AccordionStyles::ACCORDION,
                conditional!(props.bordered, AccordionStyles::MODIFIERS_BORDERED),
                props.display_size.classes().to_string(),
                props.class.clone(),
            )}
            aria-label={&props.aria_label}
            {role}
        >
            <ContextProvider<AccordionContextProps> context={accordion_context}>
                { for props.children.iter() }
            </ContextProvider<AccordionContextProps>>
        </@>
    )
}
