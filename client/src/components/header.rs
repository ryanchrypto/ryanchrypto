use yew::prelude::*;

pub struct Header {
    pub props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub home: bool,
}

impl Component for Header {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { props }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="header">
                <ul>
                    <li>
                        <a href="/" alt="home">
                            { "home" }
                        </a>
                    </li>
                    <li>
                        <a href="/about" alt="about">
                            { "about" }
                        </a>
                    </li>
                    <li>
                        <a href="/blog" alt="blog">
                            { "blog" }
                        </a>
                    </li>
                </ul>
            </nav>
        }
    }
}