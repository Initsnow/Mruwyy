#![allow(non_snake_case)]
use dioxus::prelude::*;
use lib::api;
use std::sync::{RwLock};
use ncm_api::SongInfo;
use tracing::Level;
mod components;
use components::{
    account::AccountDetail,
    account::AccountInfo,
    album::AlbumDetail,
    list::{List, ListDetail},
    loading::Loading,
    login::Login,
    playbar::PlayBar,
    sidebar::Sidebar,
    singer::SingerDetail,
};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(PlayBar)]
    #[layout(Sidebar)]
    #[route("/")]
    Home {},
    #[route("/playlist/:songlist_id")]
    ListDetail { songlist_id: u64 },
    #[route("/album/:album_id")]
    AlbumDetail { album_id: u64 },
    #[route("/singer/:singer_name")]
    SingerDetail { singer_name: String },
    #[route("/login")]
    Login {},
    #[route("/account")]
    AccountDetail {},
    #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[derive(Clone)]
struct Status {
    login: Option<AccountInfo>,
}

#[derive(Clone)]
enum PlayMode {
    Normal,
    Shuffle,
    Single,
}

#[derive(Clone)]
struct Play {
    play_current_id: Option<u64>,
    play_flag: bool,
    play_list: Option<Vec<SongInfo>>,
    mode: PlayMode,
    preload_limit: usize,
}

//const STYLE: &str = manganis::mg!(file("./assets/icons/style.css"));
fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    //println!("style css location:{}", STYLE);
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(RwLock::new(Status { login: None })));
    use_context_provider(|| {
        Signal::new(RwLock::new(Play {
            play_current_id: None,
            play_list: None,
            play_flag:false,
            mode: PlayMode::Normal,
            preload_limit: 1,
        }))
    });
    spawn(async {
        let mut status = use_context::<Signal<RwLock<Status>>>();
        let api = &api::CLIENT;
        if let Ok(login_info) = api.login_status().await {
            dbg!("已通过cookie登录");
            status.write().write().unwrap().login = Some(AccountInfo {
                name: login_info.nickname,
                uid: login_info.uid,
                avatar_url: login_info.avatar_url,
            })
        }
    });

    rsx! {
        link { rel: "stylesheet", href: "style.css" }
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        p {"NOBODY (´。＿。｀)"}
        p {"404 - Page not found: {route:?}"}
        Link { to: Route::Home {}, "Go home, and never come back." }
    }
}

#[component]
fn Home() -> Element {
    let future = use_resource(|| async {
        let api = &api::CLIENT;
        api.top_song_list("全部", "hot", 0, 10).await
    });
    rsx! {
        match &*future.read() {
            Some(Ok(response)) => rsx!{
                div{class:"row",
                    h1 {"推荐歌单"}
                    div {class:"song_list",
                        for song_list in response{
                            List{name:song_list.name.to_owned(),cover_url:song_list.cover_img_url.to_owned(),id:song_list.id}
                        }
                    }
                }


            },
            Some(Err(e)) => rsx!{
                p {"Error: {e}"}
            },
            None => rsx!{
                Loading {}
            }
        }


    }
}
