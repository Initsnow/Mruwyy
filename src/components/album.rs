use crate::components::list::Tracks;
use crate::{
    components::{list::TrackList, loading::Loading},
    Route,
};
use api::SongList;
use dioxus::prelude::*;
use lib::*;
#[component]
pub fn AlbumDetail(album_id: u64) -> Element {
    let future = use_resource(move || async move {
        let api = &api::CLIENT;
        api.album(album_id).await
    });
    rsx! {
        match &* future.read() { Some(Ok(response)) => { rsx! { div { id :
        "album_info_container", img { class : "song_cover", src : "{response.pic_url}", }
        div { id : "album_info", h1 { "{response.name}" } p { id : "data&count",
        " 发行时间 {getdate(response.publish_time as i64)} · {response.songs.len()} 首歌 · {response.songs.iter().map(|e| e.duration/1000/60).sum::<u64>()} 分钟"
        } p { "{response.description}" } } } TrackList { tracks : response.songs
        .to_owned() } } } Some(Err(e)) => rsx! { p { "Error: {e}" } }, None => rsx! {
        Loading {} } }
    }
}
#[component]
pub fn Albums(tracks: Tracks) -> Element {
    rsx! {
        div { class: "userlist acrylic album",
            for song_list in &tracks.vec {
                AlbumWithAuthor {
                    name: song_list.name.clone(),
                    cover_url: song_list.cover_url.clone(),
                    id: song_list.id,
                    author: song_list.author.clone()
                }
            }
        }
    }
}
#[component]
pub fn AlbumWithAuthor(
    name: String,
    cover_url: String,
    id: u64,
    author: String,
) -> Element {
    rsx!(
        Link { class: "album", to: Route::AlbumDetail { album_id: id },
            div { class: "item",
                img { class: "song_cover album", src: "{cover_url}" }
                div { class: "list_info",
                    div { class: "list_name", "{name}" }
                    div { class: "list_author", "{author}" }
                }
            }
        }
    )
}
