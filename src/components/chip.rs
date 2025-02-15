//! Chip
use crate::prelude::{Button, ButtonVariant, Icon};
use std::fmt::Debug;
use yew::prelude::*;

/// Properties for [`Chip`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChipProperties {
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub badge: Option<String>,
    #[prop_or_default]
    pub overflow: bool,
    #[prop_or_default]
    pub draggable: bool,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    #[prop_or_default]
    pub icon: Option<Icon>,
}

/// Chip component
///
/// > A **chip** is used to communicate a value or a set of attribute-value pairs within workflows that involve filtering a set of objects.
///
/// See: <https://www.patternfly.org/components/chip>
///
/// ## Properties
///
/// Defined by [`ChipProperties`].
#[function_component(Chip)]
pub fn chip(props: &ChipProperties) -> Html {
    let mut classes = Classes::from("pf-v5-c-chip");

    if props.draggable {
        classes.push("pf-m-draggable");
    }

    // this is only used in the chip group component
    if props.overflow {
        classes.push("pf-m-overflow");
    }

    let body = html! {
        <>
            <span class="pf-v5-c-chip__content">
                { render_icon(props) }
                <span class="pf-v5-c-chip__text">{props.text.clone()}</span>
                { render_badge(props) }
                { render_close(props) }
            </span>
        </>
    };

    if props.overflow {
        html! {<button class={classes}>{body}</button>}
    } else {
        html! {<div class={classes}>{body}</div>}
    }
}

fn render_icon(props: &ChipProperties) -> Html {
    html! (
        if let Some(icon) = &props.icon {
            <span class="pf-v5-c-chip__icon"> { icon.as_html() } </span>
        }
    )
}

fn render_badge(props: &ChipProperties) -> Html {
    html! (
        if let Some(badge) = &props.badge {
            <span class="pf-v5-c-badge pf-m-read"> {badge} </span>
        }
    )
}

fn render_close(props: &ChipProperties) -> Html {
    html! (
        if let Some(onclose) = &props.onclose {
            <span class="pf-v5-c-chip__actions">
                <Button variant={ButtonVariant::Plain} icon={Icon::Times} onclick={onclose.reform(|e: MouseEvent| {
                    // This should work, but right now doesn't due to yewstack/yew#3041
                    e.stop_propagation();
                })} />
            </span>
        }
    )
}
