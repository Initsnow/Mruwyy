use crate::components::{list::TrackList, loading::Loading};
use dioxus::prelude::*;
use lib::*;
#[component]
pub fn DailyRecommend() -> Element {
    let future = use_resource(move || async move {
        let api = &api::CLIENT;
        api.recommend_songs().await
    });
    rsx! {
        match &* future.read() { Some(Ok(response)) => { rsx! { h1 { style :
        "margin-bottom: 1rem", "每日推荐" } TrackList { tracks : response.to_owned()
        } } } Some(Err(e)) => rsx! { p { "Error: {e}" } }, None => rsx! { Loading {} } }
    }
}
