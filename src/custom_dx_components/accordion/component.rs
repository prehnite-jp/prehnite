// ORIGINAL: https://github.com/DioxusLabs/components/tree/816f1b85ac7caefd17f8407610eb874d32d7b411/preview/src/components/accordion/component.rs
// | TYPE        | VALUE                                    |
// |-------------|------------------------------------------|
// | REPOSITORY  | https://github.com/DioxusLabs/components |
// | LICENSE     | MIT OR Apache-2.0                        |
// | AUTHORS     | ["Dioxus Labs", "DogeDark"]              |
// | MODIFIED BY | saku shirakura<saku@sakushira.com>       |

use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    self, AccordionProps, AccordionTriggerProps,
};
use dioxus_primitives::icon;

#[component]
pub fn CustomAccordion(props: AccordionProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        accordion::Accordion {
            class: "dx-accordion",
            width: "15rem",
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CustomAccordionTrigger(props: AccordionTriggerProps) -> Element {
    rsx! {
        accordion::AccordionTrigger {
            class: "dx-accordion-trigger",
            id: props.id,
            attributes: props.attributes,
            icon::Icon {
                class: "dx-accordion-expand-icon",
                width: "1em",
                height: "1em",
                stroke: "var(--secondary-color-4)",
                padding_right: "10px",
                polyline { points: "12 5 18 11 12 17" }
            }
            {props.children}
        }
    }
}
