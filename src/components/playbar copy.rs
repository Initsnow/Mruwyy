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
use std::time::Duration;
use tokio::time::sleep;

use crate::{Play, PlayMode};

pub enum PlayAction {
    // Start(u64,Vec<SongInfo>),
    Start(),
    Next(),
    Pause(),
    Stop(),
}

pub enum Timer {
    Start(),
    Pause(),
    Stop(),
}

#[component]
pub fn PlayBar() -> Element {
    let playdata = use_context::<Signal<RwLock<crate::Play>>>();
    let player = Arc::new(RwLock::new(Player::new()));
    let player_clone = player.clone();
    let player_clone2 = player.clone();
    let time = use_signal_sync(|| (0, 0));
    let player_clone3 = player.clone();
    use_coroutine(|mut rx: UnboundedReceiver<Timer>| async move {
        let mut flag = Arc::new(Mutex::new(false));
let flag1=flag.clone();
        let player = player_clone3.clone();
            spawn(async move {
                let mut time = time.to_owned();
                dbg!(&flag1,player.read().unwrap().current_total_time);
                loop{
                    dbg!(&flag1);
                if flag1.lock().unwrap().to_owned() {
                        time.write().1 = player.read().unwrap().current_total_time;

                    time.write().0+=1;
                    dbg!(&time);
                    sleep(Duration::from_secs(1)).await;
                }else{sleep(Duration::from_secs_f32(0.5)).await;}}
            });
        while let Some(msg) = rx.next().await {
            match msg {
                Timer::Start() => {*flag.lock().unwrap()=true;},
                Timer::Stop() => {*flag.lock().unwrap()=false;time.to_owned().write().0=0;},
                Timer::Pause() => {*flag.lock().unwrap()=false;},
            }
        }
    });
    use_coroutine(|mut rx: UnboundedReceiver<PlayAction>| async move {
        let playdata = playdata.to_owned();
        spawn(async move {
            let playdata = playdata.to_owned();
            let mut time = time.to_owned();

            loop {
                if playdata.read().read().unwrap().play_flag {
                    if player_clone.read().unwrap().empty() {
                        dbg!("空了！");
                        use_coroutine_handle::<PlayAction>().send(PlayAction::Next());
                    } else {
                        dbg!("播放中！");
                        // *time.write()= player_clone.read().unwrap().time.lock().unwrap().to_owned();
                        dbg!(time.read());
                    }
                    sleep(Duration::from_secs(1)).await;
                } else {
                    sleep(Duration::from_secs(3)).await;
                }
            }
        });
        let updateFlag = move |b: bool| async move {
            let playdata = playdata.to_owned();
            loop {
                if let Ok(p) = playdata.to_owned().try_write() {
                    if let Ok(mut r) = p.write() {
                        r.play_flag = b;
                        dbg!("更改后的值为", r.play_flag);
                        break;
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        };
        let updateCurrentID = move |b: u64| async move {
            let playdata = playdata.to_owned();
            loop {
                if let Ok(p) = playdata.to_owned().try_write() {
                    if let Ok(mut r) = p.write() {
                        r.play_current_id = Some(b);
                        dbg!("更改后的值为", r.play_current_id);
                        break;
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        };
        while let Some(msg) = rx.next().await {
            match msg {
                PlayAction::Start() => {
                    if let Play {
                        play_current_id: Some(currentid),
                        ..
                    } = playdata.read().read().unwrap().to_owned()
                    {
                        if checkCache(currentid) {
                            player_clone2.write().unwrap().restart(currentid);
                            spawn(async move {
                                use_coroutine_handle::<Timer>().send(Timer::Start());
                                updateFlag(true).await;
                                dbg!("尝试更改flag为true");
                            });
                            // preload(currentid, playlist, preload_limit, playmode, player.clone())
                            //     .await;
                        } else {
                            match download(currentid).await {
                                Ok(_) => {
                                    player_clone2.write().unwrap().restart(currentid);
                                    spawn(async move {
                                        use_coroutine_handle::<Timer>().send(Timer::Start());
                                        updateFlag(true).await;
                                        preload().await;
                                    });
                                    // preload(
                                    //     currentid,
                                    //     playlist,
                                    //     preload_limit,
                                    //     playmode,
                                    //     player.clone(),
                                    // )
                                    // .await;
                                }
                                Err(e) => {
                                    dbg!(e);
                                }
                            }
                        }
                    }
                }
                PlayAction::Pause() => {
                    player_clone2.read().unwrap().pause();
                    spawn(async move {
                        updateFlag(false).await;
                        dbg!("尝试更改flag为false");
                    });
                }
                PlayAction::Stop() => {
                    player_clone2.read().unwrap().stop();
                    spawn(async move {
                        updateFlag(false).await;
                        dbg!("尝试更改flag为false");
                    });
                }
                PlayAction::Next() => {
                    if let Play {
                        play_current_id: Some(currentid),
                        play_list: Some(tracks),
                        mode: playmode,
                        ..
                    } = playdata.read().read().unwrap().to_owned()
                    {
                        let playlist: Vec<u64> = tracks.iter().map(|e| e.id).collect();
                        match playmode {
                            PlayMode::Normal => {
                                let index =
                                    playlist.iter().position(|&e| e == currentid).unwrap() + 1;
                                if index >= playlist.len() {
                                    continue;
                                }
                                let id = playlist[index];
                                spawn(async move {
                                    updateCurrentID(id).await;
                                    dbg!("尝试更改currentid为{}", id);
                                    use_coroutine_handle::<PlayAction>().send(PlayAction::Start());
                                });
                            }
                            PlayMode::Shuffle => {}
                            _ => {
                                //nothing to do..
                            }
                        }
                    }
                }
                _ => {}
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
        async fn preload() -> Vec<u64> {
            let playdata = use_context::<Signal<RwLock<crate::Play>>>();
            if let Play {
                play_current_id: Some(currentid),
                play_list: Some(tracks),
                mode: playmode,
                preload_limit,
                ..
            } = playdata.to_owned().read().read().unwrap().to_owned()
            {
                let playlist: Vec<u64> = tracks.iter().map(|e| e.id).collect();
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
                        for e in slice.clone() {
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
                        slice
                    }
                    PlayMode::Shuffle => Vec::new(),
                    _ => {
                        //nothing to do..
                        Vec::new()
                    }
                }
            } else {
                Vec::new()
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
    let player_clone3 = player.clone();
    if let Some(s) = getCurrentSong() {
        let SongInfo {
            name,
            pic_url,
            singer,
            ..
        } = s;

        // let new_stop_timer_signal = Arc::new(AtomicBool::new(false));
        // let stop_timer_signal = new_stop_timer_signal.clone();
        // let start_time = Instant::now();

        // std::thread::spawn(move || {
        //     while !stop_timer_signal.load(Ordering::SeqCst) {
        //         let elapsed = start_time.elapsed();
        //         println!("Elapsed time: {:.2?}", elapsed);
        //         time.lock().unwrap().0=elapsed.as_secs();
        //         dbg!(time.lock().unwrap());
        //         std::thread::sleep(Duration::from_secs(1));
        //     }
        // });

        // Update the stop signal for future calls
        let time = time.to_owned();
        dbg!(time.clone());
        rsx! {

            Outlet::<crate::Route> {}
            div{id:"playbar",
            class:"acrylic",
            input{
                r#type:"range",
                id:"progress",
                max:"{time.read().1}",
                value: "{time.read().0}",
                oninput: move |e|{
                    // player_clone3.write().unwrap().set_time(e.value().parse().unwrap());
                    dbg!(e.value());
                }
            }
            div {class:"time",""}
            div {class:"container",
                img{class:"song_cover",
                    src:"{pic_url}"
                }
                div{class:"title&singer",

                            h4{"{name}"}
                            Link{class:"singer",to: Route::SingerDetail{singer_name: singer.clone()},"{singer}"}

                    }
                    div{class:"control",button{onclick: |_|{
                        use_coroutine_handle::<PlayAction>().send(PlayAction::Pause());
                    },"pause"}
                    button{onclick: |_|{
                        use_coroutine_handle::<PlayAction>().send(PlayAction::Stop());
                    },"stop"}
                    button{onclick: |_|{
                        use_coroutine_handle::<PlayAction>().send(PlayAction::Next());
                    },"next"}
                }
            }
        }
        }
    } else {
        rsx! {Outlet::<crate::Route> {}}
    }
}
