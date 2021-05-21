use crate::HTML_VISIBLE;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Home {
    link: ComponentLink<Self>,
    props: HomeProps,
    color: &'static str,
}

#[derive(Properties, Clone, PartialEq)]
pub struct HomeProps {
    #[prop_or(HTML_VISIBLE)]
    pub visibility: &'static str,
}

pub enum Colors {
    Green,
    Blue,
}

impl Component for Home {
    type Message = Colors;
    type Properties = HomeProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            color: "color: rgb(0, 0, 0)",
        }
    }

    fn update(&mut self, color: Self::Message) -> ShouldRender {
        match color {
            Colors::Green => self.color = "color: rgb(0, 255, 0);",
            Colors::Blue => self.color = "color: rgb(0, 0, 255);",
        }
        true
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
                    <div class="buttons">
                        <button class="button is-success" onclick=self.link.callback(|_| Colors::Green)>{ "Verde" }</button>
                        <button class="button is-info" onclick=self.link.callback(|_| Colors::Blue)>{ "Azul" }</button>
                    </div>
                    <div class="block">
                        <p style={self.color}>{"Olá mundo"}</p>
                    </div>
                </div>
            </div>
        }
    }
}
