use crate::components::icons::Icon;
use crate::{Play, PlayMode, Route};
use dioxus::prelude::*;
use futures_util::StreamExt;
use lib::api;
use lib::play::Player;
use lib::TIME;
use ncm_api::SongInfo;
use rand::Rng;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use tokio::time::sleep;

pub enum PlayAction {
    Start,
    Next,
    Previous,
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}

pub enum Timer {
    Start,
    Pause,
    Stop,
    Set(u64),
    SetTime(u64),
}

#[component]
pub fn PlayBar() -> Element {
    let playdata = use_context::<Signal<RwLock<crate::Play>>>();
    let player = Arc::new(RwLock::new(Player::new()));
    let time = use_signal_sync(|| (0, 0));
    let timer_coroutine_handle: Coroutine<Timer> =
        use_coroutine(|mut rx: UnboundedReceiver<Timer>| {
            let mut time: Signal<(u64, u64), SyncStorage> = time.clone();
            let player = player.clone();
            let flag = Arc::new(Mutex::new(false));
            let flag1 = flag.clone();
            let player_clone = player.clone();
            async move {
                spawn(async move {
                    let mut time = time.to_owned();
                    loop {
                        if *flag1.lock().unwrap() {
                            time.write().1 = TIME.read().unwrap().get_total_millis();
                            time.write().0 = TIME.read().unwrap().get_current_millis();
                            sleep(Duration::from_secs_f32(0.3)).await;
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
                        Timer::SetTime(t) => {
                            time.write().0 = t;
                            TIME.write().unwrap().set(t);
                        }
                        Timer::Set(t) => {
                            time.write().0 = t;
                            TIME.write().unwrap().set(t);
                            player_clone.read().unwrap().seek(t);
                        }
                    }
                }
            }
        });

    let play_coroutine_handle = use_coroutine(|mut rx: UnboundedReceiver<PlayAction>| {
        let playdata = playdata.to_owned();
        let player_clone = player.clone();
        let player_clone_ = player.clone();

        async move {
            spawn(async move {
                loop {
                    if player_clone_.read().unwrap().empty()
                        && playdata.read().try_read().unwrap().deref().play_flag
                    {
                        handle_play_action_next(
                            playdata.clone(),
                            player_clone_.clone(),
                            timer_coroutine_handle.clone(),
                        )
                        .await;
                    }
                    sleep(Duration::from_secs(1)).await;
                }
            });

            while let Some(msg) = rx.next().await {
                match msg {
                    PlayAction::Start => {
                        handle_play_action_start(
                            playdata.clone(),
                            player_clone.clone(),
                            timer_coroutine_handle.clone(),
                        )
                        .await;
                    }
                    PlayAction::Pause => {
                        player_clone.read().unwrap().pause();
                        update_play_flag(playdata.clone(), false).await;
                        timer_coroutine_handle.clone().send(Timer::Pause);
                    }
                    PlayAction::Resume => {
                        player_clone.read().unwrap().play();
                        update_play_flag(playdata.clone(), true).await;
                        timer_coroutine_handle.clone().send(Timer::Start);
                    }
                    PlayAction::Stop => {
                        player_clone.read().unwrap().stop();
                        update_play_flag(playdata.clone(), false).await;
                        timer_coroutine_handle.clone().send(Timer::Stop);
                    }
                    PlayAction::Next => {
                        dbg!("Next?");
                        handle_play_action_next(
                            playdata.clone(),
                            player_clone.clone(),
                            timer_coroutine_handle.clone(),
                        )
                        .await;
                    }
                    PlayAction::Previous => {
                        handle_play_action_previous(
                            playdata.clone(),
                            player_clone.clone(),
                            timer_coroutine_handle.clone(),
                        )
                        .await;
                    }
                    PlayAction::SetVolume(v) => {
                        player_clone.read().unwrap().set_volume(v);
                    }
                }
            }
        }
    });
    let current_song = get_current_song(playdata.read().read().unwrap().to_owned());
    async fn changeMode(mut playdata: Signal<RwLock<Play>>, to: PlayMode) {
        loop {
            if let Ok(p) = playdata.try_write() {
                if let Ok(mut r) = p.write() {
                    r.mode = to;
                    break;
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    }
    async fn changeMute(mut playdata: Signal<RwLock<Play>>, to: bool) {
        loop {
            if let Ok(p) = playdata.try_write() {
                if let Ok(mut r) = p.write() {
                    r.mute = to;
                    break;
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    }
    async fn changeVolume(mut playdata: Signal<RwLock<Play>>, to: f32) {
        loop {
            if let Ok(p) = playdata.try_write() {
                if let Ok(mut r) = p.write() {
                    r.volume = to;
                    break;
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    }
    if let Some(s) = current_song {
        let SongInfo {
            name,
            pic_url,
            singer,
            ..
        } = s;

        let time = time.clone();
        let likesongs = &api::LIKE_SONGS_LIST;

        let volume = playdata.read().read().unwrap().volume;
        rsx! {
            Outlet::<crate::Route> {}
            div { id: "playbar", class: "acrylic",
                input {
                    r#type: "range",
                    id: "progress",
                    max: "{time.read().1}",
                    value: "{time.read().0}",
                    step: 1000,
                    oninput: move |e| {
                        use_coroutine_handle::<Timer>().send(Timer::SetTime(e.value().parse().unwrap()));
                    },
                    onchange: move |e| {
                        use_coroutine_handle::<Timer>().send(Timer::Set(e.value().parse().unwrap()));
                    }
                }
                if time.read().1 != 0 {
                    div {
                        class: "time",
                        left: "{time.read().0 as u64 as f64 / time.read().1 as u64 as f64 * 100.0}%",
                        "{time.read().0 / 1000 / 60}:{time.read().0 / 1000 % 60:02}"
                    }
                }
                div { class: "controls",
                    div { class: "container",
                        img { class: "song_cover", src: "{pic_url}" }
                        div { class: "title&singer",
                            h4 { "{name}" }
                            Link {
                                class: "singer",
                                to: Route::SingerDetail {
                                    singer_name: singer.clone(),
                                },
                                "{singer}"
                            }
                        }
                        div { class: "control",
                            if likesongs.check(playdata.read().read().unwrap().to_owned().play_current_id.unwrap()) {
                                div {
                                    onclick: move |_| async move {
                                        let api = &api::CLIENT;
                                        let currentsongid = playdata
                                            .read()
                                            .read()
                                            .unwrap()
                                            .to_owned()
                                            .play_current_id
                                            .unwrap();
                                        let r = api.like(false, currentsongid).await;
                                        if r {
                                            likesongs.remove(currentsongid).await;
                                        }
                                    },
                                    Icon { name: "favorite_fill" }
                                }
                            } else {
                                div {
                                    onclick: move |_| async move {
                                        let api = &api::CLIENT;
                                        let currentsongid = playdata
                                            .read()
                                            .read()
                                            .unwrap()
                                            .to_owned()
                                            .play_current_id
                                            .unwrap();
                                        let r = api.like(true, currentsongid).await;
                                        if r {
                                            likesongs.add(currentsongid).await;
                                        }
                                    },
                                    Icon { name: "favorite" }
                                }
                            }
                        }
                    }
                    div { class: "container",
                        button { onclick: move |_| play_coroutine_handle.send(PlayAction::Previous),
                            Icon { name: "skip_previous" }
                        }
                        if playdata.read().read().unwrap().play_flag {
                            button { onclick: move |_| play_coroutine_handle.send(PlayAction::Pause),
                                Icon { name: "pause" }
                            }
                        } else {
                            button { onclick: move |_| play_coroutine_handle.send(PlayAction::Resume),
                                Icon { name: "play_arrow" }
                            }
                        }
                        button { onclick: move |_| play_coroutine_handle.send(PlayAction::Next),
                            Icon { name: "skip_next" }
                        }
                    }
                    div { class: "container",
                        button {
                            //change to Link
                            Link { to: Route::PlayList {},
                                Icon { name: "queue_music" }
                            }
                        }
                        if playdata.read().read().unwrap().mode == PlayMode::Normal {
                            button {
                                onclick: move |_| async move {
                                    changeMode(playdata.to_owned(), PlayMode::Loop).await;
                                },
                                Icon { name: "repeat" }
                            }
                        } else if playdata.read().read().unwrap().mode == PlayMode::Loop {
                            button {
                                onclick: move |_| async move {
                                    changeMode(playdata.to_owned(), PlayMode::Single).await;
                                },
                                Icon { name: "repeat_on" }
                            }
                        } else if playdata.read().read().unwrap().mode == PlayMode::Single {
                            button {
                                onclick: move |_| async move {
                                    changeMode(playdata.to_owned(), PlayMode::Normal).await;
                                },
                                Icon { name: "repeat_one_on" }
                            }
                        }
                        if playdata.read().read().unwrap().mode == PlayMode::Random {
                            button {
                                onclick: move |_| async move {
                                    changeMode(playdata.to_owned(), PlayMode::Normal).await;
                                },
                                Icon { name: "shuffle_on" }
                            }
                        } else {
                            button {
                                onclick: move |_| async move {
                                    changeMode(playdata.to_owned(), PlayMode::Random).await;
                                },
                                Icon { name: "shuffle" }
                            }
                        }

                        div { class: "volume_controls",
                            if playdata.read().read().unwrap().mute {
                                button {
                                    onclick: move |_| async move {
                                        play_coroutine_handle.send(PlayAction::SetVolume(volume));
                                        changeMute(playdata.to_owned(), false).await;
                                    },
                                    Icon { name: "no_sound" }
                                }
                            } else {
                                if volume >= 0.5 {
                                    button {
                                        onclick: move |_| async move {
                                            play_coroutine_handle.send(PlayAction::SetVolume(0.0));
                                            changeMute(playdata.to_owned(), true).await;
                                        },
                                        Icon { name: "volume_up" }
                                    }
                                } else {
                                    button {
                                        onclick: move |_| async move {
                                            play_coroutine_handle.send(PlayAction::SetVolume(0.0));
                                            changeMute(playdata.to_owned(), true).await;
                                        },
                                        Icon { name: "volume_down" }
                                    }
                                }
                            }
                            div { class: "volume_container",
                                input {
                                    r#type: "range",
                                    id: "volume",
                                    max: "1",
                                    step: "0.01",
                                    value: "{volume}",
                                    oninput: move |e| async move {
                                        play_coroutine_handle.send(PlayAction::SetVolume(e.value().parse().unwrap()));
                                        changeVolume(playdata.to_owned(), e.value().parse().unwrap()).await;
                                    }
                                }
                                div {
                                    class: "volume",
                                    left: "{volume * 100.0}%",
                                    "{volume * 100.0:.0}"
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            Outlet::<crate::Route> {}
        }
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
                player.write().unwrap().stop();
                update_play_flag(playdata.clone(), false).await;
                timer_coroutine_handle.clone().send(Timer::Stop);
                return;
            }
            let id = playlist[index];
            update_current_id(playdata.clone(), id).await;
            handle_play_action_start(
                playdata.clone(),
                player.clone(),
                timer_coroutine_handle.clone(),
            )
            .await;
        }
        PlayMode::Loop => {
            let mut index = playlist.iter().position(|&e| e == currentid).unwrap() + 1;
            if index >= playlist.len() {
                index = 0;
            }
            let id = playlist[index];
            update_current_id(playdata.clone(), id).await;
            handle_play_action_start(
                playdata.clone(),
                player.clone(),
                timer_coroutine_handle.clone(),
            )
            .await;
        }
        PlayMode::Random => {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..playlist.len());
            let id = playlist[index];
            update_current_id(playdata.clone(), id).await;
            handle_play_action_start(
                playdata.clone(),
                player.clone(),
                timer_coroutine_handle.clone(),
            )
            .await;
        }
        PlayMode::Single => {
            let index = playlist.iter().position(|&e| e == currentid).unwrap();
            let id = playlist[index];
            update_current_id(playdata.clone(), id).await;
            handle_play_action_start(
                playdata.clone(),
                player.clone(),
                timer_coroutine_handle.clone(),
            )
            .await;
        }
    }
}

async fn handle_play_action_previous(
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
            let currentindex = playlist.iter().position(|&e| e == currentid).unwrap();
            dbg!(currentindex);
            if currentindex == 0 {
                player.write().unwrap().stop();
                update_play_flag(playdata.clone(), false).await;
                timer_coroutine_handle.clone().send(Timer::Stop);
                return;
            }
            let index = currentindex - 1;
            let id = playlist[index];
            dbg!(id, index);
            update_current_id(playdata.clone(), id).await;
            handle_play_action_start(
                playdata.clone(),
                player.clone(),
                timer_coroutine_handle.clone(),
            )
            .await;
        }
        PlayMode::Random => {
            // Implement Random mode handling if needed
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
    let url = api.songs_url(&[id], "12800").await.unwrap()[0]
        .url
        .to_owned();
    let response = reqwest::get(url).await?;
    fs::create_dir_all("cache/")?;
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
            PlayMode::Random => Vec::new(),
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
