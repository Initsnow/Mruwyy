use crate::components::{album::Albums, loading::Loading};
use dioxus::prelude::*;
use lib::*;
#[component]
pub fn Stars() -> Element {
    let future = use_resource(move || async move {
        let api = &api::CLIENT;
        api.album_sublist(0, 50).await
    });
    rsx! {
        match &* future.read() { Some(Ok(response)) => { rsx! { h1 { style :
        "margin-bottom: 1rem", "收藏专辑" } Albums { tracks : response.to_owned()
        .into_iter().collect() } } } Some(Err(e)) => rsx! { p { "Error: {e}" } }, None =>
        rsx! { Loading {} } }
    }
}
