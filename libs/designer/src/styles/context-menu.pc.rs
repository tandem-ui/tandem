/**
 * !! This file is AUTO GENERATED by the Paperclip Yew compiler.
 */

use yew::prelude::*;
use yew::{function_component, Children, html, Properties, Callback, MouseEvent};

#[path = "theme.pc.rs"]
mod imp;

#[derive(Properties, PartialEq)]
pub struct ContextMenuProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ContextMenu(props: &ContextMenuProps) -> Html {
    html! {
        <div>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuItemProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ContextMenuItem(props: &ContextMenuItemProps) -> Html {
    html! {
        <div>
            { for props.children.iter() }
        </div>
    }
}

