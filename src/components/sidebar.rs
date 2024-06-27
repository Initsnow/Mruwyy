use crate::components::icons::Icon;
use crate::Route;
use crate::Status;
use dioxus::prelude::*;
use std::sync::RwLock;

#[component]
pub fn Sidebar() -> Element {
    let status = use_context::<Signal<RwLock<Status>>>();
    let playdata = use_context::<Signal<RwLock<crate::Play>>>();
    rsx! {
        div {
            id: "sidebar",
            "style": if playdata.read().read().unwrap().play_current_id.is_some() { "height: calc(100% - 4.1rem);" },
            class: "acrylic",
            GoBackButton { 
                Icon { name: "chevron_backward" }
            }
            form { id: "search",
                input {
                    placeholder: "Search...",
                    name: "search",
                    r#type: "text",
                    class: "acrylic"
                }
                button { r#type: "submit", alt: "搜索",
                    Icon { name: "search" }
                }
            }
            nav {
                p { "发现" }
                ul {
                    li {
                        Link { to: Route::Home {},
                            Icon { name: "home" }
                            "主页"
                        }
                    }
                    li {
                        Link { to: Route::DailyRecommend {},
                            Icon { name: "calendar_today" }
                            "每日推荐"
                        }
                    }
                }
                p { "音乐" }
                ul {
                    // li {
                    //     a { href: "",
                    //         Icon { name: "folder" }
                    //         "离线音乐"
                    //     }
                    // }
                    // li {
                    //     Link { to: Route::RecentPlay{},
                    //         Icon { name: "history" }
                    //         "播放历史"
                    //     }
                    // }
                    li {
                        Link { to: Route::Stars {},
                            Icon { name: "star" }
                            "收藏专辑"
                        }
                    }
                }
                p { "歌单" }
                ul {
                    // li {
                    //     a { href: "",
                    //         Icon { name: "add" }
                    //         "创建歌单"
                    //     }
                    // }
                    li {
                        Link {
                            to: Route::ListDetail {
                                songlist_id: status
                                    .read()
                                    .read()
                                    .unwrap()
                                    .login
                                    .as_ref()
                                    .unwrap()
                                    .favorite_list_id,
                            },
                            Icon { name: "favorite" }
                            "我喜欢的音乐"
                        }
                    }
                }
            }
            ul {
                li {
                    if let Some(ref account_info) = status.read().read().unwrap().login {
                        Link { to: Route::AccountDetail {},
                            img {
                                alt: "logout",
                                src: "{account_info.avatar_url}",
                                style: "width: 20px; border-radius:4px"
                            }
                            "{account_info.name}"
                        }
                    } else {
                        Link { to: Route::Login {},
                            Icon { name: "account_circle" }
                            "账户"
                        }
                    }
                }
                li {
                    a { href: "",
                        Icon { name: "settings" }
                        "设置"
                    }
                }
            }
        }
        div {
            id: "content",
            style: if playdata.read().read().unwrap().play_current_id.is_some() { "padding-bottom: 4.4rem;" },
            Outlet::<Route> {}
        }
    }
}
