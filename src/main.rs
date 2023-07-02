use yew::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use reqwasm::http::Request;
use log::info;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherInfo {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "generationtime_ms")]
    pub generationtime_ms: f64,
    #[serde(rename = "utc_offset_seconds")]
    pub utc_offset_seconds: i64,
    pub timezone: String,
    #[serde(rename = "timezone_abbreviation")]
    pub timezone_abbreviation: String,
    pub elevation: f64,
    #[serde(rename = "hourly_units")]
    pub hourly_units: HourlyUnits,
    pub hourly: Hourly,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyUnits {
    pub time: String,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hourly {
    pub time: Vec<String>,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: Vec<f64>,
}

#[function_component]
fn App() -> Html {
    let msg = use_state(|| "");
    let onclick = {
        let msg = msg.clone();
        move |_| {
            msg.set("loading weather info...");
            fetch_weather_info();
        }
    };

    return html! {
        <div>
            <button {onclick}>{"check weather"}</button>
            <p>{ *msg }</p>
        </div>
    };
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

fn fetch_weather_info() {
    wasm_bindgen_futures::spawn_local(async move {
        let res = Request::get("https://api.open-meteo.com/v1/forecast?latitude=53.3331&longitude=-6.2489&hourly=temperature_2m&forecast_days=1")
        .send().await;
        match res {
            Ok(r) => {
                let j: Result<WeatherInfo, _> = r.json().await;
                match j {
                    Ok(p) => {
                        // log to web console
                        info!("{:?}", p);
                    }
                    Err(e) => {
                        info!("{:?}", e);
                    }
                }
            }
            Err(e) => {
                info!("{:?}", e);
            }
        }
    });
}