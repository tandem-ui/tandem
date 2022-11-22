/**
 * !! This file is AUTO GENERATED by the Paperclip Yew compiler.
 */

use yew::prelude::*;
use yew::{function_component, Children, html, Properties, Callback, MouseEvent};

#[path = "common.pc.rs"]
mod common;

#[path = "theme.pc.rs"]
mod theme;

#[derive(Properties, PartialEq)]
pub struct TabsProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Tabs(props: &TabsProps) -> Html {
    html! {
        <div class={if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_Tabs-1e0c5ead-7", scope_class_name)
        } else {
            "_Tabs-1e0c5ead-7".to_string()
        }}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
    pub class: String,
}

#[function_component]
pub fn Tab(props: &TabProps) -> Html {
    html! {
        <div class={format!("{} {}", props.class.clone(), if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_Tab-1e0c5ead-38", scope_class_name)
        } else {
            "_Tab-1e0c5ead-38".to_string()
        })}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LeftSidebarProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn LeftSidebar(props: &LeftSidebarProps) -> Html {
    html! {
        <common::Sidebar>
            <common::SidebarPanel>
                { for props.children.iter() }
            </common::SidebarPanel>
            
        </common::Sidebar>
    }
}

#[derive(Properties, PartialEq)]
pub struct LeftSidebarHeaderProps {
    pub __scope_class_name: Option<String>,
    pub title: Children,
}

#[function_component]
pub fn LeftSidebarHeader(props: &LeftSidebarHeaderProps) -> Html {
    html! {
        <common::SidebarPanelContent __scope_class_name={if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_LeftSidebarHeader-1e0c5ead-99", scope_class_name)
        } else {
            "_LeftSidebarHeader-1e0c5ead-99".to_string()
        }}>
            <div class={"_LeftSidebarHeader-1e0c5ead-80"}>
                <div class={"_LeftSidebarHeader-1e0c5ead-77"}></div>
                
                <span>
                    { for props.title.iter() }
                </span>
                
            </div>
            
            <div class={"_LeftSidebarHeader-1e0c5ead-98"}>
                <div class={"_LeftSidebarHeader-1e0c5ead-97"}></div>
                
            </div>
            
        </common::SidebarPanelContent>
    }
}

#[derive(Properties, PartialEq)]
struct LayerIconProps {
    pub __scope_class_name: Option<String>,
    pub class: String,
}

#[function_component]
fn LayerIcon(props: &LayerIconProps) -> Html {
    html! {
        <div class={format!("{} {}", props.class.clone(), if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_LayerIcon-root-1e0c5ead-190", scope_class_name)
        } else {
            "_LayerIcon-root-1e0c5ead-190".to_string()
        })}>
            <div class={"_LayerIcon-1e0c5ead-189"}></div>
            
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TreeNavigationItemContentProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn TreeNavigationItemContent(props: &TreeNavigationItemContentProps) -> Html {
    html! {
        <div>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TreeNavigationItemProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
    pub class: String,
}

#[function_component]
pub fn TreeNavigationItem(props: &TreeNavigationItemProps) -> Html {
    html! {
        <div class={props.class.clone()}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TagTypeProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn TagType(props: &TagTypeProps) -> Html {
    html! {
        <span class={if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_TagType-1e0c5ead-207", scope_class_name)
        } else {
            "_TagType-1e0c5ead-207".to_string()
        }}>
            { for props.children.iter() }
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct TreeNavigationItemHeaderProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
    pub class: String,
    pub controls: Children,
    pub onClick: ,
    pub style: ,
}

#[function_component]
pub fn TreeNavigationItemHeader(props: &TreeNavigationItemHeaderProps) -> Html {
    html! {
        <div class={format!("{} {}", props.class.clone(), if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_TreeNavigationItemHeader-root-1e0c5ead-302", scope_class_name)
        } else {
            "_TreeNavigationItemHeader-root-1e0c5ead-302".to_string()
        })} onClick={props.onClick.clone()} style={props.style.clone()}>
            <div class={"_TreeNavigationItemHeader-1e0c5ead-291"}>
                <div class={"_TreeNavigationItemHeader-1e0c5ead-289"}></div>
                
                { for props.children.iter() }
            </div>
            
            <div class={"_TreeNavigationItemHeader-1e0c5ead-301"}>
                { for props.controls.iter() }
            </div>
            
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LayerNavigationItemHeaderProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
    pub class: String,
    pub controls: Children,
    pub onClick: ,
    pub style: ,
}

#[function_component]
pub fn LayerNavigationItemHeader(props: &LayerNavigationItemHeaderProps) -> Html {
    html! {
        <TreeNavigationItemHeader __scope_class_name={if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_LayerNavigationItemHeader-container-1e0c5ead-394", scope_class_name)
        } else {
            "_LayerNavigationItemHeader-container-1e0c5ead-394".to_string()
        }} class={props.class.clone()} controls={
            { for props.controls.iter() }
} onClick={props.onClick.clone()} style={props.style.clone()}>
            <LayerIcon class={props.class.clone()}></LayerIcon>
            
            { for props.children.iter() }
        </TreeNavigationItemHeader>
    }
}

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    html! {
        <div class={if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_Tooltip-1e0c5ead-478", scope_class_name)
        } else {
            "_Tooltip-1e0c5ead-478".to_string()
        }}>
            { for props.children.iter() }
            <div class={"_Tooltip-1e0c5ead-477"}>
                <div class={"_Tooltip-1e0c5ead-476"}>
                    <div class={"_Tooltip-1e0c5ead-474"}></div>
                    
                    
                </div>
                
            </div>
            
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LayersProps {
    pub __scope_class_name: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Layers(props: &LayersProps) -> Html {
    html! {
        <div class={if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_Layers-1e0c5ead-488", scope_class_name)
        } else {
            "_Layers-1e0c5ead-488".to_string()
        }}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TokensProps {
    pub __scope_class_name: Option<String>,
}

#[function_component]
fn Tokens(props: &TokensProps) -> Html {
    html! {
        <div class={if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_Tokens-1e0c5ead-530", scope_class_name)
        } else {
            "_Tokens-1e0c5ead-530".to_string()
        }}>
            <TreeNavigationItem>
                <LayerNavigationItemHeader class={"composite-token open"}>
                    
                </LayerNavigationItemHeader>
                
                <LayerNavigationItemHeader class={"atom-token open"}>
                    
                </LayerNavigationItemHeader>
                
                <LayerNavigationItemHeader class={"atom-token open"}>
                    
                </LayerNavigationItemHeader>
                
                <LayerNavigationItemHeader class={"trigger open"}>
                    
                </LayerNavigationItemHeader>
                
            </TreeNavigationItem>
            
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ShadowIconProps {
    pub __scope_class_name: Option<String>,
}

#[function_component]
pub fn ShadowIcon(props: &ShadowIconProps) -> Html {
    html! {
        <div class={if let Some(scope_class_name) = &props.__scope_class_name {
            format!("{} {}", "_ShadowIcon-1e0c5ead-542", scope_class_name)
        } else {
            "_ShadowIcon-1e0c5ead-542".to_string()
        }}></div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LeftSidebarPreviewProps {
    pub __scope_class_name: Option<String>,
}

#[function_component]
pub fn LeftSidebarPreview(props: &LeftSidebarPreviewProps) -> Html {
    html! {
        <LeftSidebar>
            <LeftSidebarHeader title={
                
}></LeftSidebarHeader>
            
            <common::SidebarSection>
                <common::SidebarPanelHeader>
                    
                    <div class={"_LeftSidebarPreview-1e0c5ead-554"}></div>
                    
                </common::SidebarPanelHeader>
                
                <Layers>
                    <TreeNavigationItem>
                        <LayerNavigationItemHeader class={"component container open"} controls={
                            <Tooltip title={"display shadow"}>
                                <ShadowIcon></ShadowIcon>
                                
                            </Tooltip>
                            
}>
                            
                        </LayerNavigationItemHeader>
                        
                        <TreeNavigationItemContent>
                            <TreeNavigationItem>
                                <LayerNavigationItemHeader class={"shadow element container open"}>
                                    
                                </LayerNavigationItemHeader>
                                
                            </TreeNavigationItem>
                            
                            <TreeNavigationItem>
                                <LayerNavigationItemHeader class={"slot container open"}>
                                    
                                </LayerNavigationItemHeader>
                                
                                <TreeNavigationItemContent>
                                    <TreeNavigationItem>
                                        <LayerNavigationItemHeader class={"text"}>
                                            
                                        </LayerNavigationItemHeader>
                                        
                                        <LayerNavigationItemHeader class={"element"}>
                                            
                                            <TagType>
                                                
                                            </TagType>
                                            
                                        </LayerNavigationItemHeader>
                                        
                                    </TreeNavigationItem>
                                    
                                </TreeNavigationItemContent>
                                
                            </TreeNavigationItem>
                            
                            <TreeNavigationItem>
                                <LayerNavigationItemHeader class={"selected slot container open"}>
                                    
                                </LayerNavigationItemHeader>
                                
                                <TreeNavigationItemContent>
                                    <TreeNavigationItem>
                                        <LayerNavigationItemHeader class={"text"}>
                                            
                                        </LayerNavigationItemHeader>
                                        
                                    </TreeNavigationItem>
                                    
                                </TreeNavigationItemContent>
                                
                            </TreeNavigationItem>
                            
                        </TreeNavigationItemContent>
                        
                    </TreeNavigationItem>
                    
                </Layers>
                
            </common::SidebarSection>
            
            <common::SidebarSection>
                <common::SidebarPanelHeader>
                    
                    <div class={"_LeftSidebarPreview-1e0c5ead-636"}></div>
                    
                </common::SidebarPanelHeader>
                
                <Tokens></Tokens>
                
            </common::SidebarSection>
            
        </LeftSidebar>
    }
}

