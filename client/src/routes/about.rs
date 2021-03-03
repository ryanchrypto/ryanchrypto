use yew::prelude::*;

use crate::utils::markdown;

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        About { }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        // markdown
        let main = include_str!("../../../docs/about/main.md");
        let tech = include_str!("../../../docs/about/tech.md");
        let media = include_str!("../../../docs/about/media.md");

        html! {
            <div class="container">
                <div id="main" class="content">
                    <div class="content-item">
                        <h2>
                            { "about" }
                        </h2>
                        <div>
                            { markdown::render_markdown(main) }
                        </div>
                    </div>
                </div>
                <div id="tech" class="content">
                    <div class="content-item">
                        <h2>
                            { "tech" }
                        </h2>
                        <div>
                            { markdown::render_markdown(tech) }
                        </div>
                    </div>
                </div>
                <div id="media" class="content">
                    <div class="content-item">
                        <h2>
                            { "media" }
                        </h2>
                        <div>
                            { markdown::render_markdown(media) }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

