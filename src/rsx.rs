#![allow(non_snake_case)]
use crate::{Direction, SortBy, Sortable, UseSorter};
use dioxus::prelude::*;

/// See [`Th`].
#[derive(Props, Clone, PartialEq)]
pub struct ThProps<F: 'static + PartialEq> {
    sorter: UseSorter<F>,
    field: F,
    children: Element,
}

/// Convenience helper. Builds a `<th>` element with a click handler that calls [`UseSorter::toggle_field`]. Renders the current state using [`ThStatus`].
pub fn Th<F: Copy + Sortable>(props: ThProps<F>) -> Element {
    let mut sorter = props.sorter;
    let field = props.field;
    rsx! {
        th {
            onclick: move |_| sorter.toggle_field(field),
            { props.children }
            ThStatus {
                sorter: sorter,
                field: field,
            }
        }
    }
}

/// See [`ThStatus`].
#[derive(PartialEq, Props, Clone)]
pub struct ThStatusProps<F: 'static + PartialEq> {
    sorter: UseSorter<F>,
    field: F,
}

/// Convenience helper. Renders the [`Sortable`] value for a given [`UseSorter`] and field.
///  - If the field is unsortable then render an empty string.
///  - If the field is sortable in one direction then render an arrow pointing in that direction.
///  - If the field is sortable in both directions then render an arrow pointing in the active direction, or a double-headed arrow if the field is inactive.
///
/// Active fields will be shown in bold (i.e., the current field being sorted by). Inactive fields will be greyed out.
pub fn ThStatus<F: 'static + Sortable + Clone>(props: ThStatusProps<F>) -> Element {
    let sorter = props.sorter;
    let field = props.field;
    let (active_field, active_dir) = sorter.get_state();
    let active = active_field == field;

    match field.sort_by() {
        None => rsx!(""),
        Some(sort_by) => {
            use Direction::*;
            use SortBy::*;
            match sort_by {
                Fixed(Ascending) => rsx!(ThSpan { active: active, "↓" }),
                Fixed(Descending) => rsx!(ThSpan { active: active, "↑" }),

                Reversible(_) => rsx!(
                ThSpan {
                    active: active,
                    match (active, active_dir) {
                        (true, Direction::Ascending) => "↓",
                        (true, Direction::Descending) => "↑",
                        (false, _) => "↕",
                    }
                }),
            }
        }
    }
}

/// See [`ThSpan`].
#[derive(Props, Clone, PartialEq)]
struct ThSpan {
    active: bool,
    children: Element,
}

/// Convenience helper. Renders an active or inactive gielement.
fn ThSpan(props: ThSpan) -> Element {
    let colour = if props.active { "#555" } else { "#ccc" };
    let nbsp = "&nbsp;";
    rsx! {
        span {
            style: "color: {colour};",
            span { dangerous_inner_html: "{nbsp}", }
            { props.children }
        }
    }
}
