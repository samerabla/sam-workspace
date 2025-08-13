use dioxus::prelude::*;
use gloo_timers::future::sleep;
use std::time::Duration;

fn main() {
    dioxus::launch(App);
}

fn App_() -> Element {
    let mut x: Signal<u64> = use_signal(|| 0);
    let chess = Chess::new();
    x.set(chess.sum_all());
    rsx! {
        h1 { "{x}" }
    }
}

async fn get_weather(location: &WeatherLocation) -> Result<String, String> {
    sleep(Duration::from_secs(3)).await;
    Ok("Sunny".to_string())
}

fn App() -> Element {
    let country = use_signal(|| WeatherLocation {
        city: "Berlin".to_string(),
        country: "Germany".to_string(),
        coordinates: (52.5244, 13.4105),
    });

    // Because the resource's future subscribes to `country` by reading it (`country.read()`),
    // every time `country` changes the resource's future will run again and thus provide a new value.
    let current_weather = use_resource(move || async move { get_weather(&country()).await });

    rsx! {
        // the value of the resource can be polled to
        // conditionally render elements based off if it's future
        // finished (Some(Ok(_)), errored Some(Err(_)),
        // or is still running (None)
        match &*current_weather.read_unchecked() {
            Some(Ok(weather)) => rsx! {
                WeatherElement { weather }
            },
            Some(Err(e)) => rsx! {
                p { "Loading weather failed, {e}" }
            },
            None => rsx! {
                p { "Loading..." }
            },
        }
    }
}

#[derive(Clone)]
struct WeatherLocation {
    city: String,
    country: String,
    coordinates: (f64, f64),
}

#[component]
fn WeatherElement(weather: String) -> Element {
    rsx! {
        p { "The weather is {weather}" }
    }
}

pub struct Chess {
    index: u32,
}

impl Chess {
    pub fn new() -> Self {
        Chess { index: 0 }
    }

    // Calculate sum of 2^0 + 2^1 + ... + 2^63 (all chessboard squares)
    // pub fn sum_all(&self) -> u64 {
    //     (0..64).map(|x| 2u64.pow(x)).sum()
    // }
    pub fn sum_all(self) -> u64 {
        self.sum()
    }
}

impl Iterator for Chess {
    type Item = u64; // Using u64 to prevent overflow

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 64 {
            let result = 2u64.pow(self.index);
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}
