use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavLinkProps {
    #[props(into)]
    pub to: IntoRoutable,
    pub icon: Element,
    #[props(into)]
    pub label: String,
    #[props(default = false)]
    pub desktop_only: bool,
    // Error
    // #[props(extends = Link)]
    // #[props(extends = GlobalAttributes)]
    // pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavbarLink(props: NavLinkProps) -> Element {
    let NavLinkProps {
        // attributes,
        icon,
        label,
        to,
        desktop_only,
    } = props;
    let mut class =
        "items-center hover:text-amber-100 hover:bg-green-700 sm:px-4 sm:py-2".to_string();
    if desktop_only {
        class.push_str(" hidden sm:flex");
    } else {
        class.push_str(" flex");
    }
    rsx! {
        Link {
            class,
            active_class: "text-amber-100 bg-green-700",
            to,
            {icon},
            span {class: "hidden sm:inline", "{label}"}
        }
    }
}
