use crate::components::{list::TrackList, loading::Loading};
use dioxus::prelude::*;
use lib::api;
use ncm_api::*;
use std::error::Error;
#[component]
pub fn SingerDetail(singer_name: String) -> Element {
    let future: Resource<Result<(SingerInfo, Vec<SongInfo>), Box<dyn Error>>> = use_resource(
        use_reactive!(
            | (singer_name,) | async move { let api = & api::CLIENT; let singerinfo = api
            .search_singer(singer_name.to_string(), 0, 1). await ? [0].clone(); dbg!(&
            singerinfo.id); let songs = api.singer_songs(singerinfo.id). await ?; let ids
            : & Vec < u64 > = & songs.iter().map(| e | e.id).collect(); let songs = api
            .songs_detail(ids). await ?; Ok((singerinfo, songs)) }
        ),
    );
    rsx! {
        match &* future.read() { Some(Ok((singer, songs))) => { rsx! { div { id :
        "playlist_info_container", img { class : "song_cover", src : "{singer.pic_url}",
        } div { id : "playlist_info", h1 { "{singer.name}" } } } TrackList { tracks :
        songs.to_owned() } } } Some(Err(e)) => rsx! { p { "Error: {e}" } }, None => rsx!
        { Loading {} } }
    }
}
