use crate::Route;
use dioxus::prelude::*;
use futures_util::StreamExt;
use lib::api;
use lib::play::Player;
use ncm_api::SongInfo;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

use crate::{Play, PlayMode};

pub enum PlayAction {
    // Start(u64,Vec<SongInfo>),
    Start(),
    Pause(),
}

#[component]
pub fn PlayBar() -> Element {
    let playdata = use_context::<Signal<RwLock<crate::Play>>>();
    use_coroutine(|mut rx: UnboundedReceiver<PlayAction>| async move {
        let playdata = playdata.to_owned();
        let player = Arc::new(Mutex::new(Player::new()));
        while let Some(msg) = rx.next().await {
            match msg {
                PlayAction::Start() => {
                    if let Play {
                        play_current_id: Some(currentid),
                        play_list: Some(tracks),
                        mode: playmode,
                        preload_limit,
                    } = playdata.read().read().unwrap().to_owned()
                    {
                        let playlist: Vec<u64> = tracks.iter().map(|e| e.id).collect();
                        if checkCache(currentid) {
                            player.lock().unwrap().restart(currentid);
                            // preload(currentid, playlist, preload_limit, playmode, player.clone())
                            //     .await;
                            preload(currentid, playlist, preload_limit, playmode);
                        } else {
                            dbg!(3);
                            match download(currentid).await {
                                Ok(_) => {
                                    player.lock().unwrap().restart(currentid);
                                    // preload(
                                    //     currentid,
                                    //     playlist,
                                    //     preload_limit,
                                    //     playmode,
                                    //     player.clone(),
                                    // )
                                    // .await;
                                    preload(currentid, playlist, preload_limit, playmode);
                                }
                                Err(e) => {
                                    dbg!(e);
                                }
                            }
                        }
                    }
                }
                PlayAction::Pause() => {
                    player.lock().unwrap().pause();
                }
            }
        }

        fn checkCache(id: u64) -> bool {
            dbg!(Path::new(format!("cache/{}", id).as_str()));
            if Path::new(format!("cache/{}", id).as_str()).exists() == true {
                dbg!("存在");
                true
            } else {
                dbg!("缓存不存在，开始下载");
                false
            }
        }
        async fn download(id: u64) -> Result<(), Box<dyn Error>> {
            dbg!("开始下载");
            let api = &api::CLIENT;
            dbg!("获取到api");
            dbg!(&[id]);
            let url = api.songs_url(&[id], "12800").await.unwrap()[0]
                .url
                .to_owned();
            dbg!(&url);
            let response = reqwest::get(url).await?;
            let mut file = File::create(format!("cache/{}", id))?;
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk)?;
            }
            Ok(())
        }
        async fn preload(
            currentid: u64,
            playlist: Vec<u64>,
            preload_limit: usize,
            playmode: PlayMode,
            // player:Arc<Mutex<Player>>,
        ) {
            match playmode {
                PlayMode::Normal => {
                    let index = playlist.iter().position(|&e| e == currentid).unwrap() + 1;
                    let end_index = index + preload_limit;

                    let slice = if end_index >= playlist.len() {
                        let end_index = end_index % playlist.len();
                        let mut new_slice = Vec::new();
                        new_slice.extend_from_slice(&playlist[index..]);
                        new_slice.extend_from_slice(&playlist[..end_index]);
                        new_slice
                    } else {
                        playlist[index..end_index].to_vec()
                    };
                    for e in slice {
                        if checkCache(e) {
                            // append(e, sink);
                            dbg!("预加载 {} 成功，击中缓存", e);
                        } else {
                            match download(e).await {
                                Ok(_) => {
                                    // let file = File::open(format!("cache/{}", currentid)).unwrap();
                                    // let source = Decoder::new(BufReader::new(file)).unwrap();
                                    dbg!(currentid, e);
                                    dbg!("预加载 {} 成功，成功下载", e);
                                    // append(e, sink);
                                }
                                Err(err) => {
                                    dbg!(err);
                                }
                            }
                        }
                    }
                    // dbg!(slice);
                }
                PlayMode::Shuffle => {}
                _ => {
                    //nothing to do..
                }
            }
        }
    });
    let getCurrentSong = move || {
        if let Play {
            play_current_id: Some(id),
            play_list: Some(lists),
            ..
        } = playdata.read().read().unwrap().to_owned()
        {
            for e in lists {
                if e.id == id {
                    return Some(e);
                }
            }
            None
        } else {
            None
        }
    };

    if let Some(s) = getCurrentSong() {
        let SongInfo {
            name,
            pic_url,
            singer,
            ..
        } = s;
        rsx! {

            Outlet::<crate::Route> {}
            div{id:"playbar",
            class:"acrylic",
                img{class:"song_cover",
                    src:"{pic_url}"
                }
                div{class:"title&singer",

                            h4{"{name}"}
                            Link{class:"singer",to: Route::SingerDetail{singer_name: singer.clone()},"{singer}"}

                    }
                    div{class:"control",button{onclick: |_|{
                        use_coroutine_handle::<PlayAction>().send(PlayAction::Pause());
                    },"next"}}
            }
        }
    } else {
        rsx! {Outlet::<crate::Route> {}}
    }
}
