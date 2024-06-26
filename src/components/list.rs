use crate::components::icons::Icon;
use crate::components::loading::Loading;
use crate::Route;
use dioxus::prelude::*;
use lib::api;
use lib::getdate;
use ncm_api::*;
use tracing::info;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;

// 首页推荐歌单
#[component]
pub fn List(name: String, cover_url: String, id: u64) -> Element {
    rsx! (
        div { class: "item",
            img { class: "song_cover", src: "{cover_url}" }
            div { class: "list_name",
                Link {
                    to: Route::ListDetail {
                        songlist_id: id,
                    },
                    "{name}"
                }
            }
        }
    )
}

// 带作者
#[component]
pub fn ListWithAuthor(name: String, cover_url: String, id: u64, author: String) -> Element {
    rsx! (
        Link {
            to: Route::ListDetail {
                songlist_id: id,
            },
            div { class: "item",
                img { class: "song_cover", src: "{cover_url}" }
                div { class: "list_info",
                    div { class: "list_name", "{name}" }
                    div { class: "list_author", "{author}" }
                }
            }
        }
    )
}

#[derive(PartialEq, Clone)]
pub struct Track {
    name: String,
    cover_url: String,
    pub author: String,
    id: u64,
}

#[derive(Default, Props, PartialEq, Clone)]
pub struct Tracks {
    pub vec: Vec<Track>,
}

impl From<SongList> for Track {
    fn from(song: SongList) -> Self {
        Track {
            name: song.name,
            cover_url: song.cover_img_url,
            author: song.author,
            id: song.id,
        }
    }
}

impl FromIterator<SongList> for Tracks {
    fn from_iter<I: IntoIterator<Item = SongList>>(iter: I) -> Self {
        let tracks = iter.into_iter().map(Track::from).collect();
        Tracks { vec: tracks }
    }
}

impl Extend<Track> for Tracks {
    fn extend<T: IntoIterator<Item = Track>>(&mut self, iter: T) {
        self.vec.extend(iter);
    }
}

// 我的歌单
#[component]
pub fn UserList(lists: Tracks) -> Element {
    rsx! {
        div { class: "userlist acrylic",
            h1 { "我的歌单" }
            for song_list in &lists.vec {
                ListWithAuthor {
                    name: song_list.name.clone(),
                    cover_url: song_list.cover_url.clone(),
                    id: song_list.id,
                    author: song_list.author.clone()
                }
            }
        }
    }
}

// 收藏歌单
#[component]
pub fn UserFavoriteList(lists: Tracks) -> Element {
    rsx! {
        div { class: "userlist acrylic",
            h1 { "收藏歌单" }
            for song_list in &lists.vec {
                ListWithAuthor {
                    name: song_list.name.clone(),
                    cover_url: song_list.cover_url.clone(),
                    id: song_list.id,
                    author: song_list.author.clone()
                }
            }
        }
    }
}

// 歌单详情
#[component]
pub fn ListDetail(songlist_id: u64) -> Element {
    let future = use_resource(move || async move {
        let api = &api::CLIENT;
        api.song_list_detail(songlist_id).await
    });

    rsx! {
        match &*future.read() {
            Some(Ok(response)) => rsx! {
                div {
                    id: "playlist_info_container",
                    img {
                        class: "song_cover",
                        src: "{response.cover_img_url}",
                    },
                    div {
                        id: "playlist_info",
                        h1 { "{response.name}" }
                        p { id: "data&count", "最后更新于 {getdate(response.track_update_time as i64)} · {response.songs.len()} 首歌" }
                        p { "{response.description}" }
                    }
                }
                TrackList { tracks: response.songs.clone() }
            },
            Some(Err(e)) => rsx! {
                p { "Error: {e}" }
            },
            None => rsx! {
                Loading {}
            }
        }
    }
}

// 歌单详情内的歌曲列表
#[component]
pub fn TrackList(tracks: Vec<SongInfo>) -> Element {
    use crate::components::playbar::PlayAction;
    fn lazyload_init() {
        eval(
            r#"
            function init() {
                var images = document.querySelectorAll(".lazy_load[data-src]");
                function callback(entries) {
                    for (let i of entries) {
                        if (i.isIntersecting) {
                            let img = i.target;
                            let trueSrc = img.getAttribute("data-src");
                            img.setAttribute("src", trueSrc);
                            img.removeAttribute("data-src");
                            observer.unobserve(img);
                        }
                    }
                }
                let observer = new IntersectionObserver(callback);
                for (let i of images) {
                    observer.observe(i);
                }
            }
            init();
            "#,
        );
    }

    fn play(current_id: u64, tracks: Vec<SongInfo>) {
        let mut playdata = use_context::<Signal<RwLock<crate::Play>>>();
        loop {
            if let Ok(t) = playdata.try_write() {
                if let Ok(mut v) = t.write() {
                    v.play_current_id = Some(current_id);
                    v.play_list = Some(tracks.clone());
                    use_coroutine_handle::<PlayAction>().send(PlayAction::Start);
                    break;
                }
            }
            sleep(Duration::from_secs(2))
        }
    }

    let tracks_signal = use_signal(|| tracks.clone());
    // let likesongs = &api::LIKE_SONGS_LIST;
    let mut likesongs = use_signal(|| &api::LIKE_SONGS_LIST);
    let playdata = use_context::<Signal<RwLock<crate::Play>>>();
    let current_id = playdata.read().read().unwrap().to_owned().play_current_id;
    rsx! {
        div {
            id: "track_list",
            onmounted: |_| {
                lazyload_init();
            },
            for track in tracks {
                div {
                    class: "track",
                    onclick: move |_| {
                        play(track.id, tracks_signal.read().clone());
                    },
                    if let Some(id) = current_id {
                        if id == track.id {
                            div { id: "current_song" }
                        }
                    }
                    img {
                        class: "lazy_load song_cover",
                        "data-src": "{track.pic_url}"
                    }
                    div { class: "title&singer",
                        div { class: "container",
                            h2 { "{track.name}" }
                            Link {
                                class: "singer",
                                onclick: move |event: MouseEvent| {
                                    event.stop_propagation();
                                },
                                to: Route::SingerDetail {
                                    singer_name: track.singer.clone(),
                                },
                                "{track.singer}"
                            }
                        }
                    }
                    div { class: "album",
                        Link {
                            onclick: move |event: MouseEvent| {
                                event.stop_propagation();
                            },
                            to: Route::AlbumDetail {
                                album_id: track.album_id,
                            },
                            "{track.album}"
                        }
                    }
                    if likesongs.read().check(track.id) {
                        div {
                            class: "like",
                            onclick: move |e| async move {
                                e.stop_propagation();
                                let api = &api::CLIENT;
                                let r = api.like(false, track.id).await;
                                info!("取消收藏:{}", r);
                                if r {
                                    likesongs.write().remove(track.id).await;
                                }
                            },
                            Icon { name: "favorite_fill" }
                        }
                    } else {
                        div {
                            class: "like",
                            onclick: move |e| async move {
                                e.stop_propagation();
                                let api = &api::CLIENT;
                                let r = api.like(true, track.id).await;
                                info!("收藏:{}", r);
                                if r {
                                    likesongs.write().add(track.id).await;
                                }
                            },
                            Icon { name: "favorite" }
                        }
                    }
                    div { class: "duration",
                        {format!("{}:{:02}", track.duration / 1000 / 60, track.duration / 1000 % 60)}
                    }
                }
            }
        }
    }
}
