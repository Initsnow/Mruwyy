use std::sync::RwLock;

use crate::Route;
use crate::Status;
use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    let status = use_context::<Signal<RwLock<Status>>>();
    let playdata = use_context::<Signal<RwLock<crate::Play>>>();
    rsx! {
        div { id: "sidebar",
        "style":if playdata.read().read().unwrap().play_current_id.is_some() {"height: calc(100% - 4.1rem);"},
            class: "acrylic",
        GoBackButton { img {src: "./assets/icons/chevron_backward.svg", alt: "chevron_backward"} }
        form { id: "search",
            input {
                placeholder: "Search...",
                name: "search",
                r#type: "text",
                class: "acrylic"
            }
            button { r#type: "submit",
                img { alt: "Search", src: "./assets/icons/search.svg" }
            }
        }
        nav {
            p { "发现" }
            ul {
                li {
                    Link { to: Route::Home {},
                        img { alt: "home", src: "./assets/icons/home.svg" }
                        "主页"
                    }
                }
                li {
                    Link { to: "/woc/dad",
                        img {
                            src: "./assets/icons/calendar_today.svg",
                            alt: "calendar_today"
                        }
                        "每日推荐"
                    }
                }
                li {
                    a { href: "contact.html",
                        img { alt: "radio", src: "./assets/icons/radio.svg" }
                        "私人FM"
                    }
                }
                li {
                    a { href: "",
                        img { src: "./assets/icons/vital_sign.svg", alt: "vital_sign" }
                        "心动模式"
                    }
                }
            }
            p { "音乐" }
            ul {
                li {
                    a { href: "",
                        img { src: "./assets/icons/folder.svg", alt: "folder" }
                        "离线音乐"
                    }
                }
                li {
                    a { href: "",
                        img { src: "./assets/icons/history.svg", alt: "history" }
                        "播放历史"
                    }
                }
                li {
                    a { href: "",
                        img { src: "./assets/icons/star.svg", alt: "star" }
                        "我的收藏"
                    }
                }
                li {
                    a { href: "",
                        img { src: "./assets/icons/cloud.svg", alt: "cloud" }
                        "我的云盘"
                    }
                }
            }
            p { "歌单" }
            ul {
                li {
                    a { href: "",
                        img { alt: "add", src: "./assets/icons/add.svg" }
                        "创建歌单"
                    }
                }
                li {
                    a { href: "",
                        img { src: "./assets/icons/favorite.svg", alt: "favorite" }
                        "我喜欢的音乐"
                    }
                }
                li {
                    a { href: "",
                        img { alt: "list", src: "./assets/icons/list.svg" }
                        "我创建的歌单"
                    }
                }
                li {
                    a { href: "",
                        img { alt: "list", src: "./assets/icons/list.svg" }
                        "我收藏的歌单"
                    }
                }
            }
        }
        ul {
            li {
                if let Some(ref account_info) = status.read().read().unwrap().login{
                    Link { to: Route::AccountDetail{},
                                        img { alt: "logout", src: "{account_info.avatar_url}",style:"width: 20px; border-radius:4px"}
                                        "{account_info.name}"
                                    }
                }else{
                Link { to: Route::Login{},
                    img { alt: "logout", src: "./assets/icons/account_circle.svg" }
                    "账户"
                }
                }
            }
            li {
                a { href: "",
                    img { alt: "settings", src: "./assets/icons/settings.svg" }
                    "设置"
                }
            }
        }
    }
    div { id:"content",
        Outlet::<Route> {}
    }
    }
}
