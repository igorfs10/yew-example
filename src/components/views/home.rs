use crate::{reqs::Pokemon, HTML_VISIBLE};
use wasm_bindgen::JsCast;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yew::web_sys::{window, Document, HtmlInputElement, Window};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Home {
    link: ComponentLink<Self>,
    props: HomeProps,
    selected_pokemon: Pokemon,
    id: String,
    fetch_task: Option<FetchTask>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct HomeProps {
    #[prop_or(HTML_VISIBLE)]
    pub visibility: &'static str,
}

pub enum Msg {
    Click,
    ReceiveResponse(Box<Result<Pokemon, anyhow::Error>>),
}

impl Component for Home {
    type Message = Msg;
    type Properties = HomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            selected_pokemon: Pokemon::default(),
            id: "1".to_string(),
            fetch_task: None,
        }
    }

    fn update(&mut self, color: Self::Message) -> ShouldRender {
        match color {
            Msg::Click => {
                let window: Window = window().unwrap();
                let document: Document = window.document().unwrap();
                let input_id_pokemon: HtmlInputElement = document
                    .get_element_by_id("input_id_pokemon")
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap();
                self.id = input_id_pokemon.value();
                let url_get = format!(
                    "https://igorfs10.github.io/PokemonSite/api/{}/",
                    input_id_pokemon.value()
                );
                let request = Request::get(&url_get)
                    .body(Nothing)
                    .expect("Could not build request.");

                let callback = self.link.callback(
                    |response: Response<Json<Result<Pokemon, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(Box::new(data))
                    },
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
            }
            Msg::ReceiveResponse(poke) => {
                match *poke {
                    Ok(pokemon) => self.selected_pokemon = pokemon,
                    Err(_) => self.selected_pokemon = Pokemon::default(),
                }
                self.fetch_task = None;
            }
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
                        <button class="button is-info">{ "Teste" }</button>
                    </div>
                    <div class="block">
                        <input type="number" min="1" max="300" oninput=self.link.callback(|_| Msg::Click) id="input_id_pokemon" value=self.id.clone()/>
                    </div>
                    <div class="block">
                        <p>{self.selected_pokemon.name.clone()}</p>
                    </div>
                </div>
            </div>
        }
    }
}
