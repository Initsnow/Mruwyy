use dioxus::prelude::*;
#[component]
pub fn Settings() -> Element {

    rsx! {
        div { class: "settings_page",
            div { class: "container",
                h1 { "歌曲" }
                div { class: "item",
                    div { class: "left",
                        p { "音质" }
                    }
                    div { class: "right",
                        select { class: "quality",
                            option { "标准" }
                            option { "高清" }
                            option { "极高" }
                        }
                    }
                }
            }
            div{
                class:"container",
                h1{"缓存"}
                div { class: "item",
                    div { class: "left",
                        p { "缓存上限" }
                    }
                    div { class: "right",
                        select { class: "cache",
                            option { "无限制" }
                            option { "500MB" }
                            option { "8GB" }
                        }
                    }
                }
                div{class:"item",div{"class":"left",p{"已缓存 首(字节)"}} div{class:"right",button{"清除缓存"}}}
            }
            div { class: "container",
                h1 { "其他" }
                div { class: "item",
                    div { class: "left",
                        p { "启动后显示主页" }
                    }
                    div { class: "right",
                        input{r#type:"checkbox"}
                    }
                }
            }
        }
    }
}
