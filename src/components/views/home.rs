use crate::reqs::*;
use crate::{reqs::Pokemon, HTML_VISIBLE};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Home {
    link: ComponentLink<Self>,
    props: HomeProps,
    color: &'static str,
    name: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct HomeProps {
    #[prop_or(HTML_VISIBLE)]
    pub visibility: &'static str,
}

pub enum Colors {
    Green,
    Blue,
    Click,
    Test(Box<Option<Pokemon>>),
}

impl Component for Home {
    type Message = Colors;
    type Properties = HomeProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            color: "color: rgb(0, 0, 0)",
            name: "TEST".to_string(),
        }
    }

    fn update(&mut self, color: Self::Message) -> ShouldRender {
        match color {
            Colors::Green => self.color = "color: rgb(0, 255, 0);",
            Colors::Blue => self.color = "color: rgb(0, 0, 255);",
            Colors::Click => {
                wasm_bindgen_futures::spawn_local(wrap(
                    get("https://igorfs10.github.io/PokemonSite/api/1/".to_string()),
                    self.link.callback(|e| Colors::Test(Box::new(e))),
                ));
            }
            Colors::Test(poke) => match *poke {
                Some(pokemon) => self.name = pokemon.name,
                None => self.name = "None".to_string(),
            },
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
                        <button class="button is-info" onclick=self.link.callback(|_| Colors::Click)>{ "Teste" }</button>
                    </div>
                    <div class="block">
                        <p style={self.color}>{self.name.clone()}</p>
                    </div>
                </div>
            </div>
        }
    }
}
