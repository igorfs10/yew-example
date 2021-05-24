use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub type_primary: String,
    pub type_secondary: String,
    pub ability_primary: String,
    pub ability_secondary: Option<serde_json::Value>,
    pub ability_hidden: String,
    pub hp: i64,
    pub attack: i64,
    pub defense: i64,
    pub special_attack: i64,
    pub special_defense: i64,
    pub speed: i64,
    pub image: String,
}

pub async fn get(url: String) -> Option<Pokemon> {
    let client = reqwest::Client::new();

    let res = client.get(&url).send().await.unwrap();

    if res.status() == 200 {
        serde_json::from_str(&res.text().await.unwrap()).expect("Não foi possível carregar pokémon")
    } else {
        None
    }
}

pub async fn wrap<F: std::future::Future>(f: F, done_cb: yew::Callback<F::Output>) {
    done_cb.emit(f.await);
}
