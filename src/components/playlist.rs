use std::{sync::RwLock, thread::sleep, time::Duration};

use dioxus::prelude::*;
use lib::api;
use ncm_api::SongInfo;

use crate::{components::icons::Icon, Play, PlayMode, Route};

#[component]
pub fn PlayList() -> Element {
    let playdata = use_context::<Signal<RwLock<crate::Play>>>();
    let currentid;
    let tracks;
    let playmode;
    {
        let play = playdata.read().read().unwrap().to_owned();
        if let Play {
            play_current_id: Some(current),
            play_list: Some(tracklist),
            mode,
            ..
        } = play
        {
            currentid = current;
            tracks = tracklist;
            playmode = mode;
        } else {
            return rsx!("Can't Get PlayData");
        }
    }

    if let PlayMode::Random = playmode {
        return rsx!("Random Mode is enabled.");
    } else {
        rsx!(TrackList { tracks })
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
                dbg!(1);
                if let Ok(mut v) = t.write() {
                    dbg!(2);
                    v.play_current_id = Some(current_id);
                    v.play_list = Some(tracks.clone());
                    use_coroutine_handle::<PlayAction>().send(PlayAction::Start);
                    break;
                }
            }
            dbg!(0);
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
            id: "playlist",
            onmounted: |_| {
                lazyload_init();
            },
            for track in tracks {
                if let Some(id) = current_id {
                    if id == track.id {
                        h1 { "正在播放" }
                        div {
                            class: "track",
                            onclick: move |_| {
                                play(track.id, tracks_signal.read().clone());
                            },
                            div { id: "current_song" }
                            img {
                                class: "lazy_load song_cover",
                                "src": "{track.pic_url}"
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
                            div { class: "like",
                                if likesongs.read().check(track.id) {
                                    div {
                                        onclick: move |e| async move {
                                            e.stop_propagation();
                                            let api = &api::CLIENT;
                                            let r = api.like(false, track.id).await;
                                            dbg!("取消收藏:", r);
                                            if r {
                                                likesongs.write().remove(track.id).await;
                                            }
                                        },
                                        Icon { name: "favorite_fill" }
                                    }
                                } else {
                                    div {
                                        onclick: move |e| async move {
                                            e.stop_propagation();
                                            let api = &api::CLIENT;
                                            let r = api.like(true, track.id).await;
                                            dbg!("收藏:", r);
                                            if r {
                                                likesongs.write().add(track.id).await;
                                            }
                                        },
                                        Icon { name: "favorite" }
                                    }
                                }
                            }
                            div { class: "duration",
                                {format!("{}:{:02}", track.duration / 1000 / 60, track.duration / 1000 % 60)}
                            }
                        }
                        h1 { "即将播放" }
                    } else {
                        div {
                            class: "track",
                            onclick: move |_| {
                                play(track.id, tracks_signal.read().clone());
                            },
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
                            div { class: "like",
                                if likesongs.read().check(track.id) {
                                    div {
                                        onclick: move |e| async move {
                                            e.stop_propagation();
                                            let api = &api::CLIENT;
                                            let r = api.like(false, track.id).await;
                                            dbg!("取消收藏:", r);
                                            if r {
                                                likesongs.write().remove(track.id).await;
                                            }
                                        },
                                        Icon { name: "favorite_fill" }
                                    }
                                } else {
                                    div {
                                        onclick: move |e| async move {
                                            e.stop_propagation();
                                            let api = &api::CLIENT;
                                            let r = api.like(true, track.id).await;
                                            dbg!("收藏:", r);
                                            if r {
                                                likesongs.write().add(track.id).await;
                                            }
                                        },
                                        Icon { name: "favorite" }
                                    }
                                }
                            }
                            div { class: "duration",
                                {format!("{}:{:02}", track.duration / 1000 / 60, track.duration / 1000 % 60)}
                            }
                        }
                    }
                } else {
                    h1 { "未播放任何歌曲" }
                }
            }
        }
    }
}
