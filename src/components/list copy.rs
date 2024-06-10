use crate::components::loading::Loading;
use crate::Route;
use dioxus::prelude::*;
use lib::api;
use lib::getdate;
use ncm_api::*;
use std::sync::RwLock;

//首页推荐歌单
#[component]
pub fn List(name: String, cover_url: String, id: u64) -> Element {
    rsx! {
        div {
            class: "item",
            img {
                class: "song_cover",
                src: "{cover_url}",
            },
            div{
                class: "list_name",
                Link {
                    to: Route::ListDetail{songlist_id: id},
                    "{name}"
                }
            }
        }
    }
}

//带作者
#[component]
pub fn ListWithAuthor(name: String, cover_url: String, id: u64, author: String) -> Element {
    rsx! {
        Link {
                            to: Route::ListDetail{songlist_id: id},
        div {
            class: "item",
            img {
                class: "song_cover",
                src: "{cover_url}",
            },div{class:"list_info",
            div{
                class: "list_name",
                    "{name}"
            }div{class:"list_author","{author}"}}
        }
        }
    }
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
//我的歌单
#[component]
pub fn UserList(lists: Tracks) -> Element {
    rsx! {
        div {
            class:"userlist acrylic",
            h1{"我的歌单"}
            for song_list in lists.vec{
                ListWithAuthor{name:song_list.name.to_owned(),cover_url:song_list.cover_url.to_owned(),id:song_list.id,author:song_list.author}
            }
        }
    }
}

//收藏歌单
#[component]
pub fn UserFavoriteList(lists: Tracks) -> Element {
    rsx! {
        div {
            class:"userlist acrylic",
            h1{"收藏歌单"}
            for song_list in lists.vec{
                ListWithAuthor{name:song_list.name.to_owned(),cover_url:song_list.cover_url.to_owned(),id:song_list.id,author:song_list.author}
            }
        }
    }
}

//歌单详情
#[component]
pub fn ListDetail(songlist_id: u64) -> Element {
    let future = use_resource(move || async move {
        let api = &api::CLIENT;
        let x = api.song_list_detail(songlist_id).await; x
    });
    rsx! {
        match &*future.read() {
            Some(Ok(response)) => {
                // dbg!(&response);
                rsx!{
                    div{id:"playlist_info_container",
                        img{class:"song_cover",
                        src: "{response.cover_img_url}",
                        }
                        div{id:"playlist_info",
                            h1{"{response.name}"}
                            p{id:"data&count"," 最后更新于 {getdate(response.track_update_time as i64)} · {response.songs.len()} 首歌 "}
                            p{"{response.description}"}
                        }
                    }
                    TrackList{tracks: response.songs.to_owned()}
                }
            }
            Some(Err(e)) => rsx!{
                p {"Error: {e}"}
            },
            None => rsx!{
                Loading {}
            }
        }


    }
}

//歌单详情内的歌曲列表
#[component]
pub fn TrackList(tracks: Vec<SongInfo>) -> Element {
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
    init();"#,
        );
    }
    use crate::components::playbar::PlayAction;
    fn play(current_id: u64, tracks: Vec<SongInfo>) {
        let playdata = use_context::<Signal<RwLock<crate::Play>>>();
        // spawn(async move{
        //     loop{
        //     if let Ok( v) = playdata.to_owned().try_write(){
        //         let mut v =v.read().unwrap();
        //         v.play_current_id = Some(current_id);
        //         v.play_list = Some(tracks.clone());
        //         use_coroutine_handle::<PlayAction>().send(PlayAction::Start());
        //         break;
        //     }
        //     sleep(Duration::from_millis(1)).await;
        // }
        // });
        if let Ok(mut v) = playdata.to_owned().write().write(){
            v.play_current_id = Some(current_id);
            v.play_list = Some(tracks.clone());
            
            // use_coroutine_handle::<PlayAction>().send(PlayAction::Start(current_id,tracks));
            use_coroutine_handle::<PlayAction>().send(PlayAction::Start());
        }
        dbg!(1);
    }
    //let ids: Signal<Vec<u64>> = use_signal(|| tracks.iter().map(|e| e.id).collect());
let tracks_signal=use_signal(|| tracks.clone());
    rsx! {
        div{
            id:"track_list",
            onmounted:|_|{lazyload_init();},
            for track in tracks {
                div{
                    class:"track",
                    onclick: move |_| {play(track.id,tracks_signal.read().to_owned());},
                    img{class:"lazy_load song_cover",
                        "data-src": "{track.pic_url}",
                    }
                    div{class:"title&singer",
                        div{class:"container",
                            h2{"{track.name}"}
                            Link{class:"singer",to: Route::SingerDetail{singer_name: track.singer.clone()},"{track.singer}"}
                        }
                    }
                    div{class:"album",
                        Link {
                            to: Route::AlbumDetail{album_id: track.album_id},"{track.album}"
                        }
                    }
                    div{class:"duration",{format!("{}:{:0>2}", track.duration / 1000 / 60, track.duration / 1000 % 60)}}
                }
            }
        }
    }
}
