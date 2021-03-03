use yew::prelude::*;

use crate::components::sidebar::{
    Link,
    Section,
    Sidebar,
};

use crate::utils::markdown;

pub struct Blog {
    pub props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub slug: String,
}

impl Component for Blog {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blog { props }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let mut menu: Vec<Section> = Vec::new();
        
        let mut section: Vec<Link> = Vec::new();

        // blog

        section.push(Link  {
            name: "A Degenerate Regenerate".to_string(),
            root: "blog".to_string(),
            slug: "a-degenerate-regenerate".to_string(),
        });

        menu.push(Section {
            title: "blog".to_string(),
            links: section,
        });

        // index
        let mut path = include_str!("../../../docs/blog/index.md");

        // blog
        if self.props.slug == "a-degenerate-regenerate" {
            path = include_str!("../../../docs/blog/a-degenerate-regenerate.md")
        }

        html! {
            <div class="container">
                <Sidebar menu=menu />
                <div class="document">
                    <div class="document-item">
                        { markdown::render_markdown(path) }
                    </div>
                </div>
            </div>
        }
    }
}
