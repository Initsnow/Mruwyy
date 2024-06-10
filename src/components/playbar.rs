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
    Start,
    Next,
    Pause,
    Stop,
}

pub enum Timer {
    Start,
    Pause,
    Stop,
    Set(usize),
}

#[component]
pub fn PlayBar() -> Element {
    let playdata = use_context::<Signal<RwLock<crate::Play>>>();
    let player = Arc::new(RwLock::new(Player::new()));
    let time = use_signal_sync(|| (0, 0));
    
    let timer_coroutine_handle = use_coroutine(|mut rx: UnboundedReceiver<Timer>| {
        let mut time = time.clone();
        let player = player.clone();
        let flag = Arc::new(Mutex::new(false));
        let flag1 = flag.clone();

        async move {
            spawn(async move {
                let mut time = time.to_owned();
                loop {
                    if *flag1.lock().unwrap() {
                        time.write().1 = player.read().unwrap().current_total_time;
                        time.write().0 += 1;
                        sleep(Duration::from_secs(1)).await;
                    } else {
                        sleep(Duration::from_secs_f32(0.5)).await;
                    }
                }
            });

            while let Some(msg) = rx.next().await {
                match msg {
                    Timer::Start => {
                        *flag.lock().unwrap() = true;
                    }
                    Timer::Stop => {
                        *flag.lock().unwrap() = false;
                        time.write().0 = 0;
                    }
                    Timer::Pause => {
                        *flag.lock().unwrap() = false;
                    }
                    Timer::Set(t)=>{
                        time.write().0 = t;
                    }
                }
            }
        }
    });

    let play_coroutine_handle = use_coroutine(|mut rx: UnboundedReceiver<PlayAction>| {
        let playdata = playdata.to_owned();
        let player_clone = player.clone();
        let time = time.clone();

        async move {
            while let Some(msg) = rx.next().await {
                match msg {
                    PlayAction::Start => {
                        handle_play_action_start(playdata.clone(), player_clone.clone(), timer_coroutine_handle.clone()).await;
                    }
                    PlayAction::Pause => {
                        player_clone.write().unwrap().pause();
                        update_play_flag(playdata.clone(), false).await;
                        timer_coroutine_handle.clone().send(Timer::Pause);
                    }
                    PlayAction::Stop => {
                        player_clone.write().unwrap().stop();
                        update_play_flag(playdata.clone(), false).await;
                        timer_coroutine_handle.clone().send(Timer::Stop);
                    }
                    PlayAction::Next => {
                        handle_play_action_next(playdata.clone(), player_clone.clone(), timer_coroutine_handle.clone()).await;
                    }
                }
            }

            loop {
                if playdata.read().read().unwrap().play_flag {
                    if player_clone.read().unwrap().empty() {
                        use_coroutine_handle::<PlayAction>().send(PlayAction::Next);
                    } else {
                        dbg!(time.read());
                    }
                    sleep(Duration::from_secs(1)).await;
                } else {
                    sleep(Duration::from_secs(3)).await;
                }
            }
        }
    });

    let current_song = get_current_song(playdata.read().read().unwrap().to_owned());

    if let Some(s) = current_song {
        let SongInfo {
            name,
            pic_url,
            singer,
            ..
        } = s;

        let time = time.clone();
        rsx! {
            Outlet::<crate::Route> {}
            div { id: "playbar", class: "acrylic",
                input {
                    r#type: "range",
                    id: "progress",
                    max: "{time.read().1}",
                    value: "{time.read().0}",
                    oninput: move |e| {
                        // player_clone3.write().unwrap().set_time(e.value().parse().unwrap());
                        // dbg!(e.value());
                        use_coroutine_handle::<Timer>().send(Timer::Set(e.value().parse().unwrap()));
                    }
                }
                div { class: "time", "" }
                div { class: "container",
                    img { class: "song_cover", src: "{pic_url}" }
                    div { class: "title&singer",
                        h4 { "{name}" }
                        Link { class: "singer", to: Route::SingerDetail { singer_name: singer.clone() }, "{singer}" }
                    }
                    div { class: "control",
                        button { onclick: move |_| play_coroutine_handle.send(PlayAction::Pause), "pause" }
                        button { onclick: move |_| play_coroutine_handle.send(PlayAction::Stop), "stop" }
                        button { onclick: move |_| play_coroutine_handle.send(PlayAction::Next), "next" }
                    }
                }
            }
        }
    } else {
        rsx! { Outlet::<crate::Route> {} }
    }
}

async fn handle_play_action_start(
    playdata: Signal<RwLock<crate::Play>>,
    player: Arc<RwLock<Player>>,
    timer_coroutine_handle: Coroutine<Timer>,
) {

        let currentid;
        {
            let play = playdata.read().read().unwrap().to_owned();
            if let Play {
                play_current_id: Some(id),
                ..
            } = play
            {
                currentid = id;
            } else {
                return; // Exit early if play_current_id is None
            }
        }
        if check_cache(currentid) {
            player.write().unwrap().restart(currentid);
            timer_coroutine_handle.send(Timer::Stop);
            timer_coroutine_handle.send(Timer::Start);
            update_play_flag(playdata.clone(), true).await;
        } else {
            match download(currentid).await {
                Ok(_) => {
                    player.write().unwrap().restart(currentid);
                    timer_coroutine_handle.send(Timer::Start);
                    update_play_flag(playdata.clone(), true).await;
                    preload(playdata.clone()).await;
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
    }


    async fn handle_play_action_next(
        playdata: Signal<RwLock<crate::Play>>,
        player: Arc<RwLock<Player>>,
        timer_coroutine_handle: Coroutine<Timer>,
    ) {
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
                return; // Exit early if any of the required fields are None
            }
        }
    
        let playlist: Vec<u64> = tracks.iter().map(|e| e.id).collect();
        match playmode {
            PlayMode::Normal => {
                let index = playlist.iter().position(|&e| e == currentid).unwrap() + 1;
                if index >= playlist.len() {
                    return;
                }
                let id = playlist[index];
                dbg!(id);
                update_current_id(playdata.clone(), id).await;
                handle_play_action_start(playdata.clone(), player.clone(), timer_coroutine_handle.clone()).await;
            }
            PlayMode::Shuffle => {
                // Implement Shuffle mode handling if needed
            }
            _ => {
                // Handle other play modes if needed
            }
        }
    }
    

fn check_cache(id: u64) -> bool {
    Path::new(&format!("cache/{}", id)).exists()
}

async fn download(id: u64) -> Result<(), Box<dyn Error>> {
    let api = &api::CLIENT;
    let url = api.songs_url(&[id], "12800").await.unwrap()[0].url.to_owned();
    let response = reqwest::get(url).await?;
    let mut file = File::create(format!("cache/{}", id))?;
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
    }
    Ok(())
}

async fn preload(playdata: Signal<RwLock<crate::Play>>) -> Vec<u64> {
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
                    if check_cache(e) {
                        dbg!("预加载 {} 成功，击中缓存", e);
                    } else {
                        match download(e).await {
                            Ok(_) => {
                                dbg!("预加载 {} 成功，成功下载", e);
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
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    }
}

async fn update_play_flag(mut playdata: Signal<RwLock<crate::Play>>, flag: bool) {
    loop {
        if let Ok(p) = playdata.try_write() {
            if let Ok(mut r) = p.write() {
                r.play_flag = flag;
                break;
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
}

async fn update_current_id(mut playdata: Signal<RwLock<crate::Play>>, id: u64) {
    loop {
        if let Ok(p) = playdata.try_write() {
            if let Ok(mut r) = p.write() {
                r.play_current_id = Some(id);
                break;
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
}

fn get_current_song(play: Play) -> Option<SongInfo> {
    if let Play {
        play_current_id: Some(id),
        play_list: Some(lists),
        ..
    } = play
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
}
