use dioxus::prelude::*;

#[component]
pub fn Icon(name: String) -> Element {
    if name == "account_circle" {
        rsx! {
        svg {
            width: "20px",
            "fill": "#e8eaed",
            height: "20px",
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 -960 960 960",
            path { "d": "M237-285q54-38 115.5-56.5T480-360q66 0 127.5 18.5T723-285q35-41 52-91t17-104q0-129.67-91.23-220.84-91.23-91.16-221-91.16Q350-792 259-700.84 168-609.67 168-480q0 54 17 104t52 91Zm243-123q-60 0-102-42t-42-102q0-60 42-102t102-42q60 0 102 42t42 102q0 60-42 102t-102 42Zm.28 312Q401-96 331-126t-122.5-82.5Q156-261 126-330.96t-30-149.5Q96-560 126-629.5q30-69.5 82.5-122T330.96-834q69.96-30 149.5-30t149.04 30q69.5 30 122 82.5T834-629.28q30 69.73 30 149Q864-401 834-331t-82.5 122.5Q699-156 629.28-126q-69.73 30-149 30Zm-.28-72q52 0 100-16.5t90-48.5q-43-27-91-41t-99-14q-51 0-99.5 13.5T290-233q42 32 90 48.5T480-168Zm0-312q30 0 51-21t21-51q0-30-21-51t-51-21q-30 0-51 21t-21 51q0 30 21 51t51 21Zm0-72Zm0 319Z" }
        }}
    } else if name == "volume_down" {
        rsx! {
        svg {
            height: "24px",
            "fill": "#e8eaed",
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 -960 960 960",
            width: "24px",
            path { "d": "M360-360H240q-17 0-28.5-11.5T200-400v-160q0-17 11.5-28.5T240-600h120l132-132q19-19 43.5-8.5T560-703v446q0 27-24.5 37.5T492-228L360-360Zm380-120q0 42-19 79.5T671-339q-10 6-20.5.5T640-356v-250q0-12 10.5-17.5t20.5.5q31 25 50 63t19 80Z" }
        }}
    } else if name == "vital_sign" {
        rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 -960 960 960",
            height: "20px",
            width: "20px",
            "fill": "#e8eaed",
            path { "d": "M362-192q-19 0-34-11t-22-28l-86-213H48v-72h221l93 232 180-446q7-16.56 22-27.28Q579-768 598-768t34 11q15 11 22 28l86 213h172v72H692l-94-232-180 446q-7 16.56-22 27.28Q381-192 362-192Z" }
        }}
    } else if name == "star" {
        rsx! {
        svg {
            width: "20px",
            height: "20px",
            "viewBox": "0 -960 960 960",
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#e8eaed",
            path { "d": "m352-293 128-76 129 76-34-144 111-95-147-13-59-137-59 137-147 13 112 95-34 144ZM243-144l63-266L96-589l276-24 108-251 108 252 276 23-210 179 63 266-237-141-237 141Zm237-333Z" }
        }}
    } else if name == "skip_previous" {
        rsx! {
        svg {
            height: "24px",
            "fill": "#e8eaed",
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 -960 960 960",
            width: "24px",
            path { "d": "M220-280v-400q0-17 11.5-28.5T260-720q17 0 28.5 11.5T300-680v400q0 17-11.5 28.5T260-240q-17 0-28.5-11.5T220-280Zm458-1L430-447q-9-6-13.5-14.5T412-480q0-10 4.5-18.5T430-513l248-166q5-4 11-5t11-1q16 0 28 11t12 29v330q0 18-12 29t-28 11q-5 0-11-1t-11-5Z" }
        }}
    } else if name == "skip_next" {
        rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#e8eaed",
            "viewBox": "0 -960 960 960",
            height: "24px",
            width: "24px",
            path { "d": "M660-280v-400q0-17 11.5-28.5T700-720q17 0 28.5 11.5T740-680v400q0 17-11.5 28.5T700-240q-17 0-28.5-11.5T660-280Zm-440-35v-330q0-18 12-29t28-11q5 0 11 1t11 5l248 166q9 6 13.5 14.5T548-480q0 10-4.5 18.5T530-447L282-281q-5 4-11 5t-11 1q-16 0-28-11t-12-29Z" }
        }}
    } else if name == "shuffle_on" {
        rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#e8eaed",
            height: "24px",
            "viewBox": "0 -960 960 960",
            width: "24px",
            path { "d": "M120-40q-33 0-56.5-23.5T40-120v-720q0-33 23.5-56.5T120-920h720q33 0 56.5 23.5T920-840v720q0 33-23.5 56.5T840-40H120Zm480-120h160q17 0 28.5-11.5T800-200v-160q0-17-11.5-28.5T760-400q-17 0-28.5 11.5T720-360v62l-97-97q-12-12-28.5-12T566-395q-12 12-12.5 28t11.5 28l99 99h-64q-17 0-28.5 11.5T560-200q0 17 11.5 28.5T600-160Zm-428-12q11 11 28 11t28-11l492-492v64q0 17 11.5 28.5T760-560q17 0 28.5-11.5T800-600v-160q0-17-11.5-28.5T760-800H600q-17 0-28.5 11.5T560-760q0 17 11.5 28.5T600-720h64L172-228q-11 11-11 28t11 28Zm-1-560 168 167q11 11 28 11t28-11q12-12 11.5-28.5T395-621L227-788q-12-11-28.5-11T171-788q-11 11-11 28t11 28Z" }
        }}
    } else if name == "shuffle" {
        rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            height: "24px",
            "fill": "#e8eaed",
            width: "24px",
            "viewBox": "0 -960 960 960",
            path { "d": "M600-160q-17 0-28.5-11.5T560-200q0-17 11.5-28.5T600-240h64l-99-99q-12-12-11.5-28.5T566-396q12-12 28.5-12t28.5 12l97 98v-62q0-17 11.5-28.5T760-400q17 0 28.5 11.5T800-360v160q0 17-11.5 28.5T760-160H600Zm-428-12q-11-11-11-28t11-28l492-492h-64q-17 0-28.5-11.5T560-760q0-17 11.5-28.5T600-800h160q17 0 28.5 11.5T800-760v160q0 17-11.5 28.5T760-560q-17 0-28.5-11.5T720-600v-64L228-172q-11 11-28 11t-28-11Zm-1-560q-11-11-11-28t11-28q11-11 27.5-11t28.5 11l168 167q11 11 11.5 27.5T395-565q-11 11-28 11t-28-11L171-732Z" }
        }}
    } else if name == "settings" {
        rsx! {
        svg {
            height: "20px",
            "fill": "#e8eaed",
            "xmlns": "http://www.w3.org/2000/svg",
            width: "20px",
            "viewBox": "0 -960 960 960",
            path { "d": "m403-96-22-114q-23-9-44.5-21T296-259l-110 37-77-133 87-76q-2-12-3-24t-1-25q0-13 1-25t3-24l-87-76 77-133 110 37q19-16 40.5-28t44.5-21l22-114h154l22 114q23 9 44.5 21t40.5 28l110-37 77 133-87 76q2 12 3 24t1 25q0 13-1 25t-3 24l87 76-77 133-110-37q-19 16-40.5 28T579-210L557-96H403Zm59-72h36l19-99q38-7 71-26t57-48l96 32 18-30-76-67q6-17 9.5-35.5T696-480q0-20-3.5-38.5T683-554l76-67-18-30-96 32q-24-29-57-48t-71-26l-19-99h-36l-19 99q-38 7-71 26t-57 48l-96-32-18 30 76 67q-6 17-9.5 35.5T264-480q0 20 3.5 38.5T277-406l-76 67 18 30 96-32q24 29 57 48t71 26l19 99Zm18-168q60 0 102-42t42-102q0-60-42-102t-102-42q-60 0-102 42t-42 102q0 60 42 102t102 42Zm0-144Z" }
        }}
    } else if name == "search" {
        rsx! {
        svg {
            "fill": "#e8eaed",
            width: "20px",
            "viewBox": "0 -960 960 960",
            height: "20px",
            "xmlns": "http://www.w3.org/2000/svg",
            path { "d": "M784-120 532-372q-30 24-69 38t-83 14q-109 0-184.5-75.5T120-580q0-109 75.5-184.5T380-840q109 0 184.5 75.5T640-580q0 44-14 83t-38 69l252 252-56 56ZM380-400q75 0 127.5-52.5T560-580q0-75-52.5-127.5T380-760q-75 0-127.5 52.5T200-580q0 75 52.5 127.5T380-400Z" }
        }}
    } else if name == "repeat_one_on" {
        rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            height: "24px",
            "viewBox": "0 -960 960 960",
            width: "24px",
            "fill": "#e8eaed",
            path { "d": "M120-40q-33 0-56.5-23.5T40-120v-720q0-33 23.5-56.5T120-920h720q33 0 56.5 23.5T920-840v720q0 33-23.5 56.5T840-40H120Zm154-160h406q33 0 56.5-23.5T760-280v-120q0-17-11.5-28.5T720-440q-17 0-28.5 11.5T680-400v120H274l34-34q12-12 11.5-28T308-370q-12-12-28.5-12.5T251-371L148-268q-6 6-8.5 13t-2.5 15q0 8 2.5 15t8.5 13l103 103q12 12 28.5 11.5T308-110q11-12 11.5-28T308-166l-34-34Zm412-480-34 34q-12 12-11.5 28t11.5 28q12 12 28.5 12.5T709-589l103-103q6-6 8.5-13t2.5-15q0-8-2.5-15t-8.5-13L709-851q-12-12-28.5-11.5T652-850q-11 12-11.5 28t11.5 28l34 34H280q-33 0-56.5 23.5T200-680v120q0 17 11.5 28.5T240-520q17 0 28.5-11.5T280-560v-120h406ZM460-540v150q0 13 8.5 21.5T490-360q13 0 21.5-8.5T520-390v-170q0-17-11.5-28.5T480-600h-50q-13 0-21.5 8.5T400-570q0 13 8.5 21.5T430-540h30Z" }
        }}
    } else if name == "repeat_on" {
        rsx! {
        svg {
            "fill": "#e8eaed",
            "viewBox": "0 -960 960 960",
            "xmlns": "http://www.w3.org/2000/svg",
            width: "24px",
            height: "24px",
            path { "d": "M120-40q-33 0-56.5-23.5T40-120v-720q0-33 23.5-56.5T120-920h720q33 0 56.5 23.5T920-840v720q0 33-23.5 56.5T840-40H120Zm154-160h406q33 0 56.5-23.5T760-280v-120q0-17-11.5-28.5T720-440q-17 0-28.5 11.5T680-400v120H274l34-34q12-12 11.5-28T308-370q-12-12-28.5-12.5T251-371L148-268q-6 6-8.5 13t-2.5 15q0 8 2.5 15t8.5 13l103 103q12 12 28.5 11.5T308-110q11-12 11.5-28T308-166l-34-34Zm412-480-34 34q-12 12-11.5 28t11.5 28q12 12 28.5 12.5T709-589l103-103q6-6 8.5-13t2.5-15q0-8-2.5-15t-8.5-13L709-851q-12-12-28.5-11.5T652-850q-11 12-11.5 28t11.5 28l34 34H280q-33 0-56.5 23.5T200-680v120q0 17 11.5 28.5T240-520q17 0 28.5-11.5T280-560v-120h406Z" }
        }}
    } else if name == "repeat" {
        rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#e8eaed",
            height: "24px",
            "viewBox": "0 -960 960 960",
            width: "24px",
            path { "d": "m274-200 34 34q12 12 11.5 28T308-110q-12 12-28.5 12.5T251-109L148-212q-6-6-8.5-13t-2.5-15q0-8 2.5-15t8.5-13l103-103q12-12 28.5-11.5T308-370q11 12 11.5 28T308-314l-34 34h406v-120q0-17 11.5-28.5T720-440q17 0 28.5 11.5T760-400v120q0 33-23.5 56.5T680-200H274Zm412-480H280v120q0 17-11.5 28.5T240-520q-17 0-28.5-11.5T200-560v-120q0-33 23.5-56.5T280-760h406l-34-34q-12-12-11.5-28t11.5-28q12-12 28.5-12.5T709-851l103 103q6 6 8.5 13t2.5 15q0 8-2.5 15t-8.5 13L709-589q-12 12-28.5 11.5T652-590q-11-12-11.5-28t11.5-28l34-34Z" }
        }}
    } else if name == "radio" {
        rsx! {
        svg {
            height: "20px",
            "fill": "#e8eaed",
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 -960 960 960",
            width: "20px",
            path { "d": "M168-96q-29.7 0-50.85-21.17Q96-138.34 96-168.07v-432.41q0-22.52 13.5-41.02Q123-660 144-668l539-196 25 68-342 124h425.96Q822-672 843-650.84t21 50.88v432.24Q864-138 842.85-117T792-96H168Zm0-72h624v-264H168v264Zm143.77-48Q352-216 380-243.77q28-27.78 28-68Q408-352 380.23-380q-27.78-28-68-28Q272-408 244-380.23q-28 27.78-28 68Q216-272 243.77-244q27.78 28 68 28ZM168-504h480v-72h72v72h72v-96H168v96Zm0 336v-264 264Z" }
        }}
    } else if name == "queue_music" {
        rsx! {
        svg {
            height: "24px",
            width: "24px",
            "viewBox": "0 -960 960 960",
            "fill": "#e8eaed",
            "xmlns": "http://www.w3.org/2000/svg",
            path { "d": "M640-160q-50 0-85-35t-35-85q0-50 35-85t85-35q11 0 21 1.5t19 6.5v-288q0-17 11.5-28.5T720-720h120q17 0 28.5 11.5T880-680q0 17-11.5 28.5T840-640h-80v360q0 50-35 85t-85 35ZM160-320q-17 0-28.5-11.5T120-360q0-17 11.5-28.5T160-400h240q17 0 28.5 11.5T440-360q0 17-11.5 28.5T400-320H160Zm0-160q-17 0-28.5-11.5T120-520q0-17 11.5-28.5T160-560h400q17 0 28.5 11.5T600-520q0 17-11.5 28.5T560-480H160Zm0-160q-17 0-28.5-11.5T120-680q0-17 11.5-28.5T160-720h400q17 0 28.5 11.5T600-680q0 17-11.5 28.5T560-640H160Z" }
        }}
    } else if name == "play_arrow" {
        rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#e8eaed",
            height: "24px",
            "viewBox": "0 -960 960 960",
            width: "24px",
            path { "d": "M320-273v-414q0-17 12-28.5t28-11.5q5 0 10.5 1.5T381-721l326 207q9 6 13.5 15t4.5 19q0 10-4.5 19T707-446L381-239q-5 3-10.5 4.5T360-233q-16 0-28-11.5T320-273Z" }
        }}
    } else if name == "pause" {
        rsx! {
        svg {
            "fill": "#e8eaed",
            width: "24px",
            "viewBox": "0 -960 960 960",
            "xmlns": "http://www.w3.org/2000/svg",
            height: "24px",
            path { "d": "M640-200q-33 0-56.5-23.5T560-280v-400q0-33 23.5-56.5T640-760q33 0 56.5 23.5T720-680v400q0 33-23.5 56.5T640-200Zm-320 0q-33 0-56.5-23.5T240-280v-400q0-33 23.5-56.5T320-760q33 0 56.5 23.5T400-680v400q0 33-23.5 56.5T320-200Z" }
        }}
    } else if name == "no_sound" {
        rsx! {
        svg {
            "fill": "#e8eaed",
            height: "24px",
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 -960 960 960",
            width: "24px",
            path { "d": "m720-424-76 76q-11 11-28 11t-28-11q-11-11-11-28t11-28l76-76-76-76q-11-11-11-28t11-28q11-11 28-11t28 11l76 76 76-76q11-11 28-11t28 11q11 11 11 28t-11 28l-76 76 76 76q11 11 11 28t-11 28q-11 11-28 11t-28-11l-76-76Zm-440 64H160q-17 0-28.5-11.5T120-400v-160q0-17 11.5-28.5T160-600h120l132-132q19-19 43.5-8.5T480-703v446q0 27-24.5 37.5T412-228L280-360Z" }
        }}
    } else if name == "list" {
        rsx! {
        svg {
            height: "20px",
            "viewBox": "0 -960 960 960",
            width: "20px",
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#e8eaed",
            path { "d": "M288-600v-72h528v72H288Zm0 156v-72h528v72H288Zm0 156v-72h528v72H288ZM180-600q-14 0-25-11t-11-25.5q0-14.5 11-25t25.5-10.5q14.5 0 25 10.35T216-636q0 14-10.35 25T180-600Zm0 156q-14 0-25-11t-11-25.5q0-14.5 11-25t25.5-10.5q14.5 0 25 10.35T216-480q0 14-10.35 25T180-444Zm0 156q-14 0-25-11t-11-25.5q0-14.5 11-25t25.5-10.5q14.5 0 25 10.35T216-324q0 14-10.35 25T180-288Z" }
        }}
    } else if name == "home" {
        rsx! {
        svg {
            height: "20px",
            width: "20px",
            "viewBox": "0 -960 960 960",
            "fill": "#e8eaed",
            "xmlns": "http://www.w3.org/2000/svg",
            path { "d": "M264-216h96v-240h240v240h96v-348L480-726 264-564v348Zm-72 72v-456l288-216 288 216v456H528v-240h-96v240H192Zm288-327Z" }
        }}
    } else if name == "history" {
        rsx! {
        svg {
            "viewBox": "0 -960 960 960",
            width: "20px",
            height: "20px",
            "fill": "#e8eaed",
            "xmlns": "http://www.w3.org/2000/svg",
            path { "d": "M480-144q-140 0-238-98t-98-238h72q0 109 77.5 186.5T480-216q109 0 186.5-77.5T744-480q0-109-77.5-186.5T480-744q-62 0-114.55 25.6Q312.91-692.8 277-648h107v72H144v-240h72v130q46-60 114.5-95T480-816q70 0 131.13 26.6 61.14 26.6 106.4 71.87 45.27 45.26 71.87 106.4Q816-550 816-480t-26.6 131.13q-26.6 61.14-71.87 106.4-45.26 45.27-106.4 71.87Q550-144 480-144Zm100-200L444-480v-192h72v162l115 115-51 51Z" }
        }}
    } else if name == "folder" {
        rsx! {
        svg {
            "viewBox": "0 -960 960 960",
            width: "20px",
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#e8eaed",
            height: "20px",
            path { "d": "M168-192q-29 0-50.5-21.5T96-264v-432q0-29.7 21.5-50.85Q139-768 168-768h216l96 96h312q29.7 0 50.85 21.15Q864-629.7 864-600v336q0 29-21.15 50.5T792-192H168Zm0-72h624v-336H450l-96-96H168v432Zm0 0v-432 432Z" }
        }}
    } else if name == "favorite_fill" {
        rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            height: "20px",
            "viewBox": "0 -960 960 960",
            width: "20px",
            "fill": "#e8eaed",
            path { "d": "m480-144-50-45q-100-89-165-152.5t-102.5-113Q125-504 110.5-545T96-629q0-89 61-150t150-61q49 0 95 21t78 59q32-38 78-59t95-21q89 0 150 61t61 150q0 43-14 83t-51.5 89q-37.5 49-103 113.5T528-187l-48 43Z" }
        }}
    } else if name == "favorite" {
        rsx! {
        svg {
            class: "false",
            height: "20px",
            width: "20px",
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#e8eaed",
            "viewBox": "0 -960 960 960",
            path { "d": "m480-144-50-45q-100-89-165-152.5t-102.5-113Q125-504 110.5-545T96-629q0-89 61-150t150-61q49 0 95 21t78 59q32-38 78-59t95-21q89 0 150 61t61 150q0 43-14 83t-51.5 89q-37.5 49-103 113.5T528-187l-48 43Zm0-97q93-83 153-141.5t95.5-102Q764-528 778-562t14-67q0-59-40-99t-99-40q-35 0-65.5 14.5T535-713l-35 41h-40l-35-41q-22-26-53.5-40.5T307-768q-59 0-99 40t-40 99q0 33 13 65.5t47.5 75.5q34.5 43 95 102T480-241Zm0-264Z" }
        }}
    } else if name == "cloud" {
        rsx! {
        svg {
            "fill": "#e8eaed",
            width: "20px",
            "xmlns": "http://www.w3.org/2000/svg",
            height: "20px",
            "viewBox": "0 -960 960 960",
            path { "d": "M240-192q-80 0-136-56T48-384q0-76 52-131.5T227-576q23-85 92.5-138.5T480-768q103 0 179 69.5T744-528q70 0 119 49t49 119q0 70-49 119t-119 49H240Zm0-72h504q40 0 68-28t28-68q0-40-28-68t-68-28h-66l-6-65q-7-74-62-124.5T480-696q-64 0-115 38.5T297-556l-14 49-51 3q-48 3-80 37.5T120-384q0 50 35 85t85 35Zm240-216Z" }
        }}
    } else if name == "chevron_backward" {
        rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 -960 960 960",
            "fill": "#fff",
            width: "26px",
            height: "26px",
            path { "d": "M560-240 320-480l240-240 56 56-184 184 184 184-56 56Z" }
        }}
    } else if name == "calendar_today" {
        rsx! {
        svg {
            width: "20px",
            "fill": "#e8eaed",
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 -960 960 960",
            height: "20px",
            path { "d": "M216-96q-29.7 0-50.85-21.5Q144-139 144-168v-528q0-29 21.15-50.5T216-768h72v-96h72v96h240v-96h72v96h72q29.7 0 50.85 21.5Q816-725 816-696v528q0 29-21.15 50.5T744-96H216Zm0-72h528v-360H216v360Zm0-432h528v-96H216v96Zm0 0v-96 96Z" }
        }}
    } else if name == "bookmark_heart" {
        rsx! {
        svg {
            "viewBox": "0 -960 960 960",
            "xmlns": "http://www.w3.org/2000/svg",
            width: "20px",
            "fill": "#e8eaed",
            height: "20px",
            path { "d": "M480-407q43-39 68.98-64.27 25.98-25.27 40-43.5t18.52-31.73q4.5-13.5 4.5-29.05 0-29.45-21.27-50.95Q569.45-648 540-648q-17.5 0-33.75 6.91Q490-634.19 480-622q-10.32-12.19-26.66-19.09Q437-648 419.55-648q-28.55 0-50.05 21.38-21.5 21.39-21.5 50.99 0 15.63 4 28.63 4 13 18 31t39.88 43.46Q435.76-447.07 480-407ZM240-144v-600q0-29.7 21.15-50.85Q282.3-816 312-816h336q29.7 0 50.85 21.15Q720-773.7 720-744v600l-240-96-240 96Zm72-107 168-67 168 67v-493H312v493Zm0-493h336-336Z" }
        }}
    } else if name == "add" {
        rsx! {
        svg {
            "viewBox": "0 -960 960 960",
            "xmlns": "http://www.w3.org/2000/svg",
            height: "20px",
            width: "20px",
            "fill": "#e8eaed",
            path { "d": "M444-444H240v-72h204v-204h72v204h204v72H516v204h-72v-204Z" }
        }}
    } else {
        rsx! {
        svg {
            "fill": "#e8eaed",
            height: "24px",
            width: "24px",
            "viewBox": "0 -960 960 960",
            "xmlns": "http://www.w3.org/2000/svg",
            path { "d": "M760-481q0-83-44-151.5T598-735q-15-7-22-21.5t-2-29.5q6-16 21.5-23t31.5 0q97 43 155 131.5T840-481q0 108-58 196.5T627-153q-16 7-31.5 0T574-176q-5-15 2-29.5t22-21.5q74-34 118-102.5T760-481ZM280-360H160q-17 0-28.5-11.5T120-400v-160q0-17 11.5-28.5T160-600h120l132-132q19-19 43.5-8.5T480-703v446q0 27-24.5 37.5T412-228L280-360Zm380-120q0 42-19 79.5T591-339q-10 6-20.5.5T560-356v-250q0-12 10.5-17.5t20.5.5q31 25 50 63t19 80Z" }
        }}
    }
}
