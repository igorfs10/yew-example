use crate::HTML_HIDDEN;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone)]
pub struct Documentation {
    props: DocumentationProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct DocumentationProps {
    #[prop_or(HTML_HIDDEN)]
    pub visibility: &'static str,
}

impl Component for Documentation {
    type Message = ();
    type Properties = DocumentationProps;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="columns" style={self.props.visibility}>
                <div class="column m-3">
                    <div class="block">
                        <p>{"Heloo"}</p>
                    </div>
                </div>
            </div>
        }
    }
}
