#![allow(non_snake_case)]
use dioxus::prelude::*;
use lib::api;
use ncm_api::SongInfo;
use std::sync::RwLock;
use tracing::{info, Level};
mod components;
use components::{
    account::AccountDetail,
    account::AccountInfo,
    album::AlbumDetail,
    dailyrecommend::DailyRecommend,
    list::{List, ListDetail},
    loading::Loading,
    login::Login,
    playbar::PlayBar,
    playlist::PlayList,
    sidebar::Sidebar,
    singer::SingerDetail,
    stars::Stars,
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
    #[route("/playlist")]
    PlayList {},
    #[route("/dailyrecommend")]
    DailyRecommend {},
    #[route("/stars")]
    Stars {},
    #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[derive(Clone)]
struct Status {
    login: Option<AccountInfo>,
}

#[derive(Clone, Debug, PartialEq)]
/**
Normal => 播完暂停

Loop => 循环列表

Random => 循环随机

Shuffle => 随机播放

Single => 单曲循环
*/
enum PlayMode {
    Normal,
    Loop,
    Random,
    Single,
}

#[derive(Clone, Debug)]
struct Play {
    play_current_id: Option<u64>,
    play_flag: bool,
    play_list: Option<Vec<SongInfo>>,
    mode: PlayMode,
    preload_limit: usize,
    volume: f32,
    mute: bool,
}

// #[derive(Clone)]
// struct UserLike {
//     list: Vec<u64>,
// }

//const STYLE: &str = manganis::mg!(file("./assets/icons/style.css"));
fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(RwLock::new(Status { login: None })));
    use_context_provider(|| {
        Signal::new(RwLock::new(Play {
            play_current_id: None,
            play_list: None,
            play_flag: false,
            mode: PlayMode::Normal,
            preload_limit: 1,
            volume: 0.3,
            mute: false,
        }))
    });
    spawn(async {
        let mut status = use_context::<Signal<RwLock<Status>>>();
        let userlike = &api::LIKE_SONGS_LIST;
        let api = &api::CLIENT;
        if let Ok(login_info) = api.login_status().await {
            info!("已通过cookie登录");
            let list: Vec<u64> = api.user_song_id_list(login_info.uid).await.unwrap();
            userlike.init(list).await;
            let t: u64 = api
                .user_song_list(login_info.uid, 0, 100)
                .await
                .unwrap()
                .into_iter()
                .filter(|e| e.name.contains("喜欢的音乐") && &e.author == &login_info.nickname)
                .map(|x| x.id)
                .collect::<Vec<u64>>()[0];
            info!("获取到用户喜欢音乐歌单:{}", t);
            // dbg!(api.songs_url_v1(&[1965786312,2150753897], "jymaster").await);
            // dbg!(api.songs_music_detail(2150753897).await);

            status.write().write().unwrap().login = Some(AccountInfo {
                name: login_info.nickname,
                uid: login_info.uid,
                avatar_url: login_info.avatar_url,
                favorite_list_id: t,
            })
        }
    });

    rsx! {
        link { rel: "stylesheet", href: "style.css" }
        Router::<Route> {}
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        p { "NOBODY (´。＿。｀)" }
        p { "404 - Page not found: {route:?}" }
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
