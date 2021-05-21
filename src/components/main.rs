use crate::components::views::documentation::Documentation;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use super::super::{HTML_HIDDEN, HTML_VISIBLE};
use super::views::home::Home;

pub struct Main {
    link: ComponentLink<Self>,
    home_visibility: &'static str,
    documentation_visibility: &'static str,
}

pub enum Show {
    Home,
    Documentation,
}

impl Component for Main {
    type Message = Show;
    type Properties = ();
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            home_visibility: HTML_VISIBLE,
            documentation_visibility: HTML_HIDDEN,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Show::Home => {
                self.home_visibility = HTML_VISIBLE;
                self.documentation_visibility = HTML_HIDDEN;
            }
            Show::Documentation => {
                self.home_visibility = HTML_HIDDEN;
                self.documentation_visibility = HTML_VISIBLE;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <nav class="navbar" role="navigation" aria-label="main navigation">
                    <div id="navbarBasicExample" class="navbar-menu is-active">
                        <div class="navbar-start">
                            <a class="navbar-item" onclick=self.link.callback(|_| Show::Home)>
                                {"Home"}
                            </a>
                            <a class="navbar-item" onclick=self.link.callback(|_| Show::Documentation)>
                                {"Documentation"}
                            </a>
                        </div>
                    </div>
                </nav>
                <Home visibility={self.home_visibility}/>
                <Documentation visibility={self.documentation_visibility}/>
            </>
        }
    }
}
