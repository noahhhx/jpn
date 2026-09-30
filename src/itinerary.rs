//! The trip itself. Edit this file to change what the site shows.
//!
//! `START` is the date of Day 0; every other date follows from it.
//! `DAYS` is one entry per day, in order. `INFO` is the general info page.
//! Images live in `public/img/` and are referenced as `/img/<name>`.

use crate::trip::{
    Block::{Checklist, Links, List, Note, Schedule, Text},
    Day, Section, image, item, link, place, step,
};

/// Date of Day 0 (year, month, day).
pub const START: (i32, u32, u32) = (2026, 10, 14);

pub const DAYS: &[Day] = &[
    // Day 0 — Wed 14 Oct
    Day {
        city: "Gatwick",
        title: "Night at Gatwick",
        sections: &[
            Section {
                heading: "Hotel",
                blocks: &[
                    place(
                        "Travelodge Gatwick Airport Central",
                        "Povey Cross Road, Gatwick, RH6 0BE",
                        "Check-in from 3pm · check-out by 12 noon · family room (triple)",
                    ),
                    Text(
                        "Leave the M23 at Junction 9, signposted for Gatwick Airport. Go straight through the first roundabout (signposted Redhill A23) and then at the next roundabout take the 4th exit towards Redhill A23. At the next roundabout take the first exit past the Esso service station and turn on to Povey Cross Road. The Travelodge is on the left hand side.",
                    ),
                    Text(
                        "North Terminal pick-up point is outside the terminal from the Pick-Up zone. Gatwick Hoppa buses also run a regular shuttle to and from the airport — £5.25 per adult.",
                    ),
                ],
            },
            Section {
                heading: "Parking",
                blocks: &[
                    place(
                        "Gatwick Holiday Parking",
                        "Gatwick Holiday Parking, Larkins Road, Gatwick",
                        "Booked from Wed 14 Oct 20:30 to Mon 9 Nov 09:00",
                    ),
                    List(&[
                        item("From London or Brighton, exit the M23 at Junction 9", ""),
                        item("At the first roundabout, follow signs for the North Terminal", ""),
                        item(
                            "At the second roundabout, continue straight towards the Long Stay car parks",
                            "",
                        ),
                        item(
                            "Go past the Shell petrol station and follow the road signs to Gatwick Holiday Parking",
                            "",
                        ),
                    ]),
                    Note(
                        "Do not enter the official London Gatwick Long Stay car park by mistake — you may be charged extra. At the Long Stay entrance roundabout keep following signs to Gatwick Holiday Parking, a further 0.6 miles ahead.",
                    ),
                    Text(
                        "Free shuttle buses run every 15 minutes from the car park and take around 7 minutes to reach the North Terminal.",
                    ),
                    List(&[
                        item("Drop-off (departure)", "North Terminal, bus stop 10"),
                        item("Pick-up (return)", "North Terminal, bus stop 3"),
                    ]),
                ],
            },
        ],
    },
    // Day 1 — Thu 15 Oct
    Day {
        city: "In the air",
        title: "Fly to Tokyo",
        sections: &[
            Section {
                heading: "Getting to the airport",
                blocks: &[
                    Text(
                        "North Terminal pick-up point is outside the terminal from the Pick-Up zone. Gatwick Hoppa buses also run a regular shuttle to and from the airport — £5.25 per adult.",
                    ),
                    Text("May be worth getting a taxi for £15?"),
                ],
            },
            Section {
                heading: "Flights",
                blocks: &[
                    Schedule(&[
                        step("11:15", "Depart London Gatwick (LGW)", "Air China CA848 · 11h 30m"),
                        step("05:45", "Arrive Shanghai Pudong (PVG)", "Friday 16 October · 4h 15m change"),
                        step("10:00", "Depart Shanghai Pudong (PVG)", "Air China CA929 · 3h"),
                        step("14:00", "Arrive Tokyo Narita (NRT)", "Friday 16 October"),
                    ]),
                    Text(
                        "Each: 1 personal item, 1 × 5kg hand baggage (20 × 40 × 55cm), 2 × 23kg checked baggage.",
                    ),
                ],
            },
            Section {
                heading: "Travel insurance",
                blocks: &[
                    Text("GreatCover Gold Single Trip, couple cover, 15 Oct – 8 Nov 2026."),
                    Note("Medical emergency abroad: call Zurich Assist on +44 203 467 4128."),
                ],
            },
        ],
    },
    // Day 2 — Fri 16 Oct
    Day {
        city: "Tokyo",
        title: "Land and Explore",
        sections: &[
            Section {
                heading: "",
                blocks: &[Text(
                    "Land at Terminal 1, South Wing. Wait for Jack to arrive at 14:30.",
                )],
            },
            Section {
                heading: "Welcome Suica card",
                blocks: &[
                    Text(
                        "Get one from the B1F floor in the airport. Welcome Suica can be bought at Welcome Suica vending machines, JR EAST Travel Service Center, JAPAN RAIL CAFÉ and TAKANAWA GATEWAY Travel Service Center. The card is valid for 28 days, including the day of purchase.",
                    ),
                    Text(
                        "Use it like an Oyster for transport, in shops and for vending machines. Top up at Seven Bank ATMs, found at 7-Eleven and other convenience stores.",
                    ),
                    Note("Cash only — you can't top up with a card."),
                    image("/img/suica-top-up.webp", "Topping up at a Seven Bank ATM"),
                    Links(&[
                        link(
                            "Where to buy a Welcome Suica",
                            "https://www.jreast.co.jp/en/multi/welcomesuica/purchase.html",
                        ),
                        link(
                            "Suica card guide",
                            "https://travel.rakuten.com/contents/usa/en-us/guide/suica-card/",
                        ),
                    ]),
                ],
            },
            Section {
                heading: "Bus to Tokyo",
                blocks: &[
                    Text(
                        "Get a \"low cost bus\" ticket from the counter, ~£7pp into Tokyo. Takes 1hr from bus platform 7. They leave every 5–10 mins.",
                    ),
                    Text(
                        "Buy a time-designated ticket at the LCB Ticket Counter in the terminal and get to the bus stop at least 5 minutes before departure. You can pay with cash, IC card (PASMO, Suica, etc.) or credit card. Staff check tickets when boarding.",
                    ),
                    Links(&[link("Timetable", "https://tyo-nrt.com/en/timetable/to-tokyo")]),
                ],
            },
            Section {
                heading: "Accommodation",
                blocks: &[
                    Text(
                        "Head straight to the accommodation to drop bags. Meet Ben there and go out for dinner.\nGoose arrives at 7pm so she can meet us at the accommodation ready for bed!",
                    ),
                    place(
                        "Ikebukuro apartment",
                        "3-chōme-58-3 Ikebukuro, Toshima City, Tokyo 171-0014",
                        "",
                    ),
                ],
            },
        ],
    },
    // Day 3 — Sat 17 Oct
    Day {
        city: "Tokyo",
        title: "Explore Tokyo",
        sections: &[
            Section {
                heading: "Getting around",
                blocks: &[
                    Text(
                        "Use the 72hr subway ticket instead of Suica — likely cheaper if using it more than 3 times a day (about £1 per trip). £10.",
                    ),
                    Links(&[link(
                        "72hr Tokyo subway ticket",
                        "https://www.klook.com/en-HK/activity/1552-subway-ticket-tokyo/",
                    )]),
                ],
            },
            Section {
                heading: "Suggested activities",
                blocks: &[
                    List(&[
                        item("Central Tokyo", ""),
                        item("Ginza", ""),
                        item("Ichiran Shimbashi", "Solo ramen booths"),
                        item("Centre The Bakery", "Toast cafe"),
                        item("Happiness is KATSUDON Nihonbashi", "Recommended, non-touristy katsudon"),
                    ]),
                    image("/img/tokyo-map.webp", "Tokyo areas"),
                ],
            },
        ],
    },
    // Day 4 — Sun 18 Oct
    Day {
        city: "Tokyo",
        title: "Explore Tokyo + Muscle Bar/Karaoke Evening",
        sections: &[
            Section {
                heading: "Suggested activities",
                blocks: &[List(&[
                    item("Shinjuku", ""),
                    item("Tokyo Metropolitan Building", "Viewpoint"),
                    item("Samurai Restaurant Time", "Samurai show with drinks and Halloween treats"),
                    item(
                        "Omoide Yokocho (Memory Lane)",
                        "Narrow alleyways with lots of yakitori (chicken) spots",
                    ),
                    item("Shinjuku Golden-Gai", "Narrow alleyways with bars"),
                    item("Hanazono Shrine", ""),
                ])],
            },
            Section {
                heading: "Muscle bar",
                blocks: &[
                    Schedule(&[step(
                        "20:00",
                        "2× tables",
                        "¥6,000 per person — all you can drink for 80 minutes, fresh squeezed sour, service charge and tax included",
                    )]),
                    place(
                        "Muscle Girls",
                        "Hayama Building B1F, 2-41-2 Ikebukuro, Toshima-ku, Tokyo",
                        "5 min walk from the accommodation",
                    ),
                    Links(&[link(
                        "Booking",
                        "https://e.japanticket.com/shops/muscle_girls/products/9682/?lang=en",
                    )]),
                ],
            },
            Section {
                heading: "Karaoke",
                blocks: &[
                    Text(
                        "You can just walk into popular chains like Big Echo, Joysound or Karaoke Kan nearby.",
                    ),
                    Text(
                        "English-language support at karaoke halls is usually pretty limited. If you're lucky a staff member may speak some English, but usually the paperwork is all in Japanese. Google Translate is your friend here.",
                    ),
                    Links(&[link(
                        "Karaoke in Japan guide",
                        "https://tokyocheapo.com/entertainment/karaoke-japan-guide/",
                    )]),
                ],
            },
        ],
    },
    // Day 5 — Mon 19 Oct
    Day {
        city: "Tokyo",
        title: "Explore Tokyo + Early Night",
        sections: &[Section {
            heading: "Suggested activities",
            blocks: &[List(&[
                item("Shibuya", ""),
                item("Meiji Jingu", "Shrine"),
                item("Harajuku", "Good for cafes and fashion"),
                item("Sodateru Towel shop", ""),
                item("KOFFEE MAMEYA", "Coffee tasting").url("https://koffee-mameya.com/"),
                item("Garlic Bar Nyonnyogo", ""),
                item("Michi Naki Michi", "Hidden izakaya behind a fridge door")
                    .url("https://tabelog.com/en/tokyo/A1303/A130301/13269807/"),
            ])],
        }],
    },
    // Day 6 — Tue 20 Oct
    Day {
        city: "Mt Fuji",
        title: "Travel to Fuji",
        sections: &[
            Section {
                heading: "To book",
                blocks: &[
                    Note(
                        "Book 2× boats for Takachiho Gorge on 3 Nov @ 9am — £25 per boat. Try to get 9–10am slots for best light.",
                    ),
                    Links(&[
                        link("Boat booking", "https://eipro.jp/takachiho1/eventCalendars/index"),
                        link("Boat info (translated)", "https://en-62123.site-translation.com/boat/"),
                    ]),
                ],
            },
            Section {
                heading: "Train",
                blocks: &[
                    Text("30 min train from the accommodation to Shinjuku (Yamanote line, green)."),
                    Text("Use the green machine in any JR station and collect 2× tickets per person."),
                    Text(
                        "Luggage: up to 2 bags each, free, no reservation needed. Each bag 250cm or less (h + w + d) and 30kg or less.",
                    ),
                    Links(&[link(
                        "How to collect tickets",
                        "https://www.klook.com/en-GB/notice_content/20530/?from_source=email&from_medium=system_email&from_campaign=mob_ptp_voucher",
                    )]),
                ],
            },
            Section {
                heading: "Suggested activities",
                blocks: &[List(&[
                    item("Lake Kawaguchi viewpoints", "Chureito Pagoda / Oishi Park"),
                    item("Cafe Shizen", ""),
                    item("Sake brewery", "Tasting")
                        .url("https://www.kainokaiun.jp/en-concept-sakabousi.html#tasting"),
                    item("Water bus", "").url("https://en.kaba-bus.com/yamanakako/"),
                ])],
            },
            Section {
                heading: "Accommodation",
                blocks: &[place(
                    "MIYA HOUSE Funatsu B",
                    "3240-1 Funatsu, Fujikawaguchiko, Minamitsuru District, Yamanashi 401-0301",
                    "Check-in after 4pm · check-out before 10am",
                )],
            },
        ],
    },
    // Day 7 — Wed 21 Oct
    Day {
        city: "Kyoto",
        title: "Travel to Kyoto",
        sections: &[
            Section {
                heading: "",
                blocks: &[Text("Fuji sunrise at the lake. Check out before 10am.")],
            },
            Section {
                heading: "Drive",
                blocks: &[
                    place(
                        "Nippon Rent-A-Car (Alamo)",
                        "3647-1 Funatsu, Fujikawaguchiko, Minamitsuru District, Yamanashi 401-0301",
                        "Collect the car at 10am",
                    ),
                    Text(
                        "5hr drive. If we can see Fuji, worth visiting Izu Panorama Park on the way.\nAdd a stop at Shurakuen Park at the 3hr mark for a walk and a break.",
                    ),
                    place(
                        "Car drop-off",
                        "9 Nakatonoda-cho, Higashi 9-jo, Minami-ku, Kyoto",
                        "Drop off by 8pm",
                    ),
                    Text("30 min subway back to the apartment — drop the car and go out for dinner."),
                ],
            },
            Section {
                heading: "Accommodation",
                blocks: &[place(
                    "Home in Kyoto (Nijo Castle)",
                    "430-39 Tsukinukechō, Kyoto 602-8364",
                    "Check-in after 3pm",
                )],
            },
        ],
    },
    // Day 8 — Thu 22 Oct
    Day {
        city: "Kyoto",
        title: "Omakase",
        sections: &[
            Section {
                heading: "",
                blocks: &[
                    Text("Central/East — look to do things around this area (see the map on Day 11)."),
                    Links(&[link(
                        "1 day bus/subway ticket",
                        "https://www.discoverkyoto.com/visitors-guide/day-pass-ticket/",
                    )]),
                ],
            },
            Section {
                heading: "Suggested activities",
                blocks: &[
                    List(&[
                        item("Heian-jingū Shrine", ""),
                        item("Chion-in Temple", ""),
                        item("Kiyomizu-dera", "Lesser-known bamboo forest with temple"),
                        item("Chawanzaka", "Ceramics street"),
                        item("KAI Cutlery Store Kyoto", ""),
                        item("Kamitowa", "Book and stationery shop"),
                        item("Kyoto Takashimaya Shopping Center (T8)", "Art on the 6th floor"),
                        item("Nishiki Food Market", ""),
                        item("Katsuda", "Tonkatsu (deep-fried pork cutlet)"),
                        item(
                            "Jidai Matsuri",
                            "A famous historical festival held every year. The whole procession takes about 2 hours to pass a single spot.",
                        ),
                    ]),
                    image("/img/jidai-matsuri-route.webp", "Jidai Matsuri procession route"),
                ],
            },
            Section {
                heading: "Dinner",
                blocks: &[
                    place(
                        "Noguchi Tsunagu",
                        "371-4 Kiyomotocho, Higashiyama-ku, Kyoto",
                        "",
                    ),
                    Note("7pm — don't be late!"),
                ],
            },
        ],
    },
    // Day 9 — Fri 23 Oct
    Day {
        city: "Kyoto & Kobe",
        title: "Kyoto Exploration & Kobe",
        sections: &[
            Section {
                heading: "Suggested activities",
                blocks: &[List(&[
                    item("Anything not done in central Kyoto the day before", ""),
                    item("Nijo Castle", ""),
                    item("Ninomaru-goten Palace", ""),
                    item("Kinkaku-ji", "Temple"),
                ])],
            },
            Section {
                heading: "Minato Hanabi fireworks, Kobe",
                blocks: &[
                    Text("1.5hr train to Kobe. Aim to leave Kyoto at 2–3pm."),
                    Text("Fireworks festival at Meriken Park. More entertainment at Plenty Square (in front of Seishun-Chou Station)."),
                    Text(
                        "Follow the official \"Kobe Minato no Yoru\" Instagram (@kobehanabi) to get a commemorative holo card — first 200 people, from 17:00, at the Meriken Park Information Tent (west side of the Maritime Museum entrance).",
                    ),
                    Links(&[
                        link("Event info", "https://www.feel-kobe.jp/en/events/detail_1107.html"),
                        link("Minato Hanabi", "https://minatohanabi.jp/enjoy/"),
                    ]),
                ],
            },
            Section {
                heading: "Dinner",
                blocks: &[
                    Schedule(&[step("20:00", "Kobe beef steak Restaurant Mouriya Royal", "")]),
                    place(
                        "Mouriya Royal",
                        "1-9-9 Kitanagasa-dori, No. Kishi Bldg. 2F, Chuo-ku, Kobe, Hyogo 650-0012",
                        "",
                    ),
                    Links(&[link("Menu", "https://www.mouriya.co.jp/en/royal/menu")]),
                ],
            },
        ],
    },
    // Day 10 — Sat 24 Oct
    Day {
        city: "Amanohashidate & Ine",
        title: "Ine Fishing Village & Amanohashidate",
        sections: &[
            Section {
                heading: "Car",
                blocks: &[place(
                    "Nippon Rent-A-Car Nishi-Oji Sanjo",
                    "24 Saiinkamiimadacho, Ukyo Ward, Kyoto 615-0001",
                    "Pick up 8am (20 min walk from the accommodation) · return by 8pm",
                )],
            },
            Section {
                heading: "Plan",
                blocks: &[
                    Schedule(&[
                        step(
                            "11:00",
                            "Amanohashidate",
                            "Take the chairlift/monorail up to Amanohashidate View Land. Open 9:00–17:00 in October — the summit has the famous \"flying dragon\" view.",
                        ),
                        step(
                            "12:00",
                            "Drive to Ine (about 45 min)",
                            "The road follows the coast and gets increasingly rural as you approach the village.",
                        ),
                        step(
                            "13:00",
                            "Park at Funaya no Sato Ine",
                            "A large free car park on the hill above Ine. Walk into town — it's not big enough for large cars.",
                        ),
                        step(
                            "13:15",
                            "Walk the funaya waterfront",
                            "These aren't just preserved buildings: around 230 funaya remain along roughly 5km of coastline, and many are still homes and working spaces.",
                        ),
                        step(
                            "13:45",
                            "Lunch",
                            "Ine Cafe for something casual with the village atmosphere, or Ine Restaurant Funaya for seafood.",
                        ),
                        step(
                            "14:45",
                            "Boat around Ine Bay",
                            "£5pp. A boat house tour could also be fun, £3.50pp — need cash.",
                        ),
                        step("15:30", "Coffee + explore", ""),
                        step("16:00", "Start back to Kyoto", ""),
                    ]),
                    place(
                        "Funaya no Sato Ine car park",
                        "459 Kameshima, Ine, Yoza District, Kyoto 626-0424",
                        "",
                    ),
                    Links(&[
                        link("View Land lift & monorail", "https://www.viewland.jp/en/lift_mono/"),
                        link("Ine Bay sea taxi", "https://www.ine-kankou.jp/e_active/small-boat-sea-taxi"),
                        link("Boat house tour", "https://www.ine-kankou.jp/e_active/tomobutoya"),
                    ]),
                ],
            },
        ],
    },
    // Day 11 — Sun 25 Oct
    Day {
        city: "Kyoto",
        title: "Kyoto Exploration",
        sections: &[Section {
            heading: "Suggested activities",
            blocks: &[
                List(&[
                    item("To-ji Temple", ""),
                    item("Sanjūsangendō Temple", ""),
                    item(
                        "Kyoto Ramen Koji",
                        "Corridor of ramen shops on the 10th floor of the Kyoto Station building",
                    ),
                    item(
                        "Fushimi Inari Taisha",
                        "Go at 5am to avoid crowds. Buy a small gate and get it personalised as a souvenir.",
                    ),
                ]),
                image("/img/kyoto-map.webp", "Kyoto areas"),
            ],
        }],
    },
    // Day 12 — Mon 26 Oct
    Day {
        city: "Koyasan",
        title: "Koyasan Stay",
        sections: &[
            Section {
                heading: "Car",
                blocks: &[place(
                    "Nippon Rent-A-Car Nishi-Oji Sanjo",
                    "24 Saiinkamiimadacho, Ukyo Ward, Kyoto 615-0001",
                    "Pick up 8am",
                )],
            },
            Section {
                heading: "On the way",
                blocks: &[List(&[item(
                    "Ikoma Sanjo Amusement Park",
                    "Sky Cycle — views in the sky on the way. Adds 30 mins.",
                )])],
            },
            Section {
                heading: "Temple stay",
                blocks: &[
                    place(
                        "Koyasan Sanadabo Rengejoin",
                        "700 Kōyasan, Koya, Ito District, Wakayama 648-0211",
                        "2.5hr drive · check-in between 15:00 and 16:00",
                    ),
                    Schedule(&[
                        step("17:00", "Asokukan meditation", ""),
                        step("18:00", "Dinner", ""),
                        step("Night", "Fire ceremony", ""),
                        step("21:00", "Quiet hours", "Until 06:00"),
                        step("06:00", "Morning Buddhist ceremonies", ""),
                    ]),
                    List(&[
                        item("Sutra copying", "¥1,000"),
                        item("Public bath", "Men and women are separated"),
                    ]),
                ],
            },
        ],
    },
    // Day 13 — Tue 27 Oct
    Day {
        city: "Kyoto",
        title: "Kyoto Exploration",
        sections: &[
            Section {
                heading: "",
                blocks: &[Text("Check out by 10am. Return the car by 8pm.")],
            },
            Section {
                heading: "Suggested activities",
                blocks: &[List(&[
                    item("Use the car to get to places that are harder to reach by train", ""),
                    item("Katsura River", ""),
                    item("Kyoto City Rakusai Bamboo Park", ""),
                    item("Matsunoo-taisha (Matsuo-taisha) Shrine", ""),
                    item("Arashiyama Monkey Park Iwatayama", ""),
                    item("Bread, Espresso and Arashiyama Garden", "Bakery"),
                    item("Otagi Nenbutsu-ji Temple", "Rakan statues"),
                ])],
            },
        ],
    },
    // Day 14 — Wed 28 Oct
    Day {
        city: "Kyoto",
        title: "Knife Making",
        sections: &[
            Section {
                heading: "Knife making",
                blocks: &[
                    Schedule(&[step("14:30", "Studio Shinobi YASE", "Until 17:30")]),
                    place(
                        "Studio Shinobi YASE",
                        "168-1 Yasenosecho, Sakyo Ward, Kyoto 601-1254",
                        "",
                    ),
                ],
            },
            Section {
                heading: "Suggested activities",
                blocks: &[List(&[
                    item("Kyoto Botanical Gardens", "Get picture stamp cards here"),
                    item("Philosopher's Path (Tetsugaku no Michi)", "North end"),
                    item("Ginkaku-ji", "Temple"),
                ])],
            },
        ],
    },
    // Day 15 — Thu 29 Oct
    Day {
        city: "Osaka",
        title: "Osaka",
        sections: &[
            Section {
                heading: "Send bags to Yufuin",
                blocks: &[
                    Note("Check at Kyoto Station if they can send our bags to Yufuin!"),
                    Text("Size = h + w + d. 160 size = £30."),
                    List(&[
                        item("7-Eleven", "Partners with Yamato Transport"),
                        item(
                            "Yamato Transport Kyoto Station Center",
                            "308 Aburanokoji-cho, very close to Kyoto Station. Open 8am–9pm.",
                        ),
                        item(
                            "Sagawa luggage counter, Kyoto Station (ASTY Kyoto)",
                            "Open 9am–8pm",
                        ),
                    ]),
                    Text("Send them to:"),
                    place(
                        "Yufuin Bath Satoyamasafu (ゆふいんバース 里山茶風)",
                        "828番地1 Yufuinchō Kawaminami, Yufu, Oita 879-5103",
                        "+81 977-76-5755",
                    ),
                    Note("Put your name + \"arriving 31 October\" prominently on the luggage label."),
                ],
            },
            Section {
                heading: "Osaka",
                blocks: &[
                    Text("25 min train to Osaka from Kyoto Station, every 15 mins."),
                    List(&[
                        item("Dotonbori", "Entertainment/nightlife district"),
                        item("Shinsekai", "Neon district"),
                        item(
                            "Object Osaka store",
                            "DIY patch and charm shop — pick a base (pouch, keychain, passport cover) then go wild with iron-on patches",
                        ),
                    ]),
                ],
            },
        ],
    },
    // Day 16 — Fri 30 Oct
    Day {
        city: "Uji",
        title: "Tea Ceremony",
        sections: &[
            Section {
                heading: "Tea ceremony",
                blocks: &[
                    Schedule(&[step("13:30", "Tea ceremony in Uji", "Until 16:30")]),
                    Text(
                        "After exiting Keihan Uji Station, go down the stairs to the roundabout. The tourist information centre is on your left. Your interpreter, wearing a kimono, will be waiting there to welcome you.",
                    ),
                    place("Meeting point", "Keihan Uji Station, Uji, Kyoto", ""),
                ],
            },
            Section {
                heading: "Suggested activities",
                blocks: &[
                    List(&[item("Byōdō-in Temple", "Explore south Kyoto")]),
                    Text("Quiet evening before the long journey to Yufuin."),
                ],
            },
        ],
    },
    // Day 17 — Sat 31 Oct
    Day {
        city: "Hiroshima → Yufuin",
        title: "Hiroshima → Yufuin",
        sections: &[
            Section {
                heading: "Morning",
                blocks: &[
                    Schedule(&[step(
                        "06:00",
                        "Leave the accommodation",
                        "30 mins to Kyoto Station. Grab breakfast from the station.",
                    )]),
                    image("/img/kyoto-hiroshima-trains.webp", "Kyoto → Hiroshima shinkansen"),
                ],
            },
            Section {
                heading: "Hiroshima",
                blocks: &[
                    Text(
                        "30 min walk or 15 min bus to the Hiroshima Peace Memorial Museum. Buy tickets on the day — £1. Takes 45 mins to go round.",
                    ),
                    image("/img/peace-museum-bus.webp", "Bus to the Peace Memorial Park"),
                    Text(
                        "15 min walk to the park near the castle for lunch. Okonomiyaki (savoury pancake) originated here.",
                    ),
                    Text("Hiroshima → Hakata"),
                ],
            },
            Section {
                heading: "Accommodation",
                blocks: &[
                    Text("Booked, pending confirmation. The ryokan is a 15 min walk from the station."),
                    place(
                        "Yufuin Bath Satoyamasafu",
                        "828番地1 Yufuinchō Kawaminami, Yufu, Oita 879-5103",
                        "Check-in by 6pm",
                    ),
                ],
            },
        ],
    },
    // Day 18 — Sun 1 Nov
    Day {
        city: "Beppu → Aso",
        title: "Beppu → Aso",
        sections: &[
            Section {
                heading: "Car",
                blocks: &[
                    place(
                        "Nissan Rent a Car — Yufuin",
                        "3729-1 Kawakami, Yufuin-cho, Yufu, Oita 879-5102",
                        "Pick up 10am",
                    ),
                    Text(
                        "Look at Lake Kinrin in Yufuin, then pick up the car from Nissan Rent a Car by the station and drive to Beppu (30 mins).",
                    ),
                ],
            },
            Section {
                heading: "The Seven Hells of Beppu",
                blocks: &[
                    List(&[
                        item(
                            "Common Admission Pass",
                            "£12 — buy it at any of the hells",
                        ),
                        item("5 are clustered and walkable", "The other 2 need a drive"),
                        item("Free parking at all the hells", "Over 600 spaces"),
                        item("Only open 8am–5pm", ""),
                        item("Need to try the pudding", ""),
                    ]),
                    image("/img/beppu-hells-map.webp", "Where the hells are"),
                    List(&[
                        item("Umi Jigoku", "A milky blue pool with strong venting steam"),
                        item(
                            "Oniishibozu Jigoku",
                            "A series of small bubbling grey mud pools surrounded by volcanic rocks",
                        ),
                        item("Shiraike Jigoku", "A well landscaped milky white pond with a smaller aquarium"),
                        item(
                            "Kamado Jigoku",
                            "Multiple blue, green and red ponds with a large demon statue and stalls selling food cooked by the steam. Book here to get an onsen steam-cooked egg with entry.",
                        ),
                        item(
                            "Oniyama Jigoku",
                            "A pond with very strong steam. Also home to a large number of caged crocodiles, which seems quite odd.",
                        ),
                        item(
                            "Chinoike Jigoku",
                            "Named for a dramatic red \"blood pond\", though it's more of a dull orange. Large gift shop and a full restaurant serving hell-steamed food.",
                        ),
                        item(
                            "Tatsumaki Jigoku",
                            "The only one with a spouting geyser — erupts every 30–40 minutes",
                        ),
                    ]),
                    image("/img/beppu-hells-guide.webp", "Beppu hells pamphlet"),
                    Links(&[
                        link(
                            "First-timer's guide to the Beppu hells",
                            "https://whereandwander.com/first-timers-guide-to-beppu-hells-hot-springs-recommendations/",
                        ),
                        link(
                            "Official pamphlet (PDF)",
                            "https://beppu-jigoku.com/pamphlet/images/English.pdf",
                        ),
                    ]),
                ],
            },
            Section {
                heading: "Beppu Ropeway",
                blocks: &[
                    Text("Views over Beppu — one of the most beautiful sights in Japan in autumn."),
                    place(
                        "Beppu Ropeway",
                        "10-7 Aza Kanbara, Oaza Minami Tateishi, Beppu, Oita 874-0000",
                        "£8 round trip · 250 parking spaces",
                    ),
                    Links(&[link(
                        "Ropeway info",
                        "https://japantravel.navitime.com/en/area/jp/spot/02301-13800174/",
                    )]),
                ],
            },
            Section {
                heading: "Accommodation",
                blocks: &[
                    Text("1.5hr drive to the accommodation — leave Beppu by 4pm."),
                    Text("Stop for a food shop in Oguni, a 15 min drive from the onsen."),
                    place(
                        "Kurasako Onsen Sakura",
                        "2849-1 Manganji, Minamioguni, Aso District, Kumamoto 869-2402",
                        "Check-in 3pm – 6pm",
                    ),
                ],
            },
        ],
    },
    // Day 19 — Mon 2 Nov
    Day {
        city: "Aso",
        title: "Nabegataki Falls & Mt Aso",
        sections: &[
            Section {
                heading: "Plan",
                blocks: &[
                    Schedule(&[
                        step(
                            "Morning",
                            "Breakfast at Au Pan & Coffee bakery",
                            "In Manganji, 10 min drive",
                        ),
                        step(
                            "10:30",
                            "Nabegataki Falls",
                            "20 min drive. £1.50pp entry — need to book online. Spend 30 mins here, then try the local delicacies at Sorairo no Tane, a popular bakery on Route 387 near Nabegataki Park.",
                        ),
                        step(
                            "13:00",
                            "Mt Aso",
                            "1hr drive. Cars can drive all the way to a parking lot next to the crater, but it's £5 per car (round trip) for the last few km, which is a toll road.",
                        ),
                        step("16:00", "Should be finished", ""),
                        step("17:00", "Sunset", ""),
                    ]),
                    place(
                        "Au Pan & Coffee",
                        "6733-1 Manganji, Minamioguni, Aso District, Kumamoto 869-2402",
                        "Au Kurokawa, building C",
                    ),
                    place("Nabegataki Falls", "Kurobuchi, Oguni, Aso District, Kumamoto", ""),
                    Links(&[
                        link("Au Pan & Coffee", "https://www.instagram.com/aupan_coffee/"),
                        link("Nabegataki Falls guide", "https://www.journeyera.com/nabegataki-falls/"),
                        link(
                            "Nabegataki Falls tickets",
                            "https://webket.jp/pc/ticket/itemdetail?fc=51545&ac=9000&igc=0003",
                        ),
                    ]),
                ],
            },
            Section {
                heading: "Mt Aso walks",
                blocks: &[
                    List(&[
                        item("Mt Eboshidake", "2hr round trip"),
                        item("Mt Kishimadake", "1hr round trip"),
                    ]),
                    Text("Alternate plan if able to go within 1km: 90 min hike."),
                    Links(&[
                        link("Aso visitor centre", "https://aso-visitorcenter.com/en/"),
                        link("Mt Aso hike", "https://www.japan-guide.com/e/e4552.html"),
                        link("Trail map", "https://yamap.com/mountains/14169"),
                    ]),
                ],
            },
        ],
    },
    // Day 20 — Tue 3 Nov
    Day {
        city: "Takachiho",
        title: "Takachiho Gorge Boat Tour & Walk",
        sections: &[
            Section {
                heading: "Gorge boats",
                blocks: &[
                    Text(
                        "Assuming we have boats at around 9–10am. 1.5hr drive to Takachiho.\nA couple of shrines around, but also a nice place to be and walk.",
                    ),
                    place(
                        "Takachiho Gorge car park 1 (高千穂峡第1御塩井駐車場)",
                        "204 Mukoyama, Takachiho, Nishiusuki District, Miyazaki 882-1103",
                        "",
                    ),
                    Links(&[
                        link(
                            "Boat calendar, Nov 2026",
                            "https://takachiho-kanko.info/boat/detail.php?year=2026&month=11#calendar_block",
                        ),
                        link("Boat info", "https://takachiho-kanko.info/boat/detail.php"),
                        link("Booking terms", "https://eipro.jp/takachiho1/terms/view/toppage"),
                    ]),
                ],
            },
            Section {
                heading: "Kyushu Olle: Takachiho Course",
                blocks: &[
                    Text("13.2km, 4.5hrs. Aim to start at 11–12. Bring a packed lunch."),
                    Links(&[
                        link(
                            "AllTrails",
                            "https://www.alltrails.com/en-gb/trail/japan/miyazaki--2/kyushu-olle-takachiho-course",
                        ),
                        link(
                            "Hiking around Takachiho Gorge",
                            "https://www.travellingminions.com/hiking-around-takachiho-gorge/",
                        ),
                    ]),
                ],
            },
        ],
    },
    // Day 21 — Wed 4 Nov
    Day {
        city: "Miyajima",
        title: "Aso → Miyajima",
        sections: &[
            Section {
                heading: "Drive",
                blocks: &[
                    Text("Check out by 10am. 2.5hr drive to Kokura."),
                    place(
                        "Nissan Rent a Car — Kokura Station",
                        "2-11-25 Asano, Kokurakita-ku, Kitakyushu, Fukuoka 802-0001",
                        "Return the car by 4pm",
                    ),
                ],
            },
            Section {
                heading: "Train & ferry",
                blocks: &[
                    Text(
                        "Train from Kokura to Hiroshima (~1hr) — about 5 per hour, so no need to book.",
                    ),
                    image("/img/kokura-hiroshima-train.webp", "Kokura → Hiroshima"),
                    Text("Then a local train to Miyajimaguchi."),
                    image("/img/hiroshima-miyajimaguchi-train.webp", "Hiroshima → Miyajimaguchi"),
                    Text(
                        "And finally the ferry — book on the day at the stand. There are two operators, JR or local, both around £1.\nTotal travel time 2hrs.",
                    ),
                    image("/img/miyajima-ferry.webp", "Ferry to Miyajima and walk to the hotel"),
                ],
            },
            Section {
                heading: "Accommodation",
                blocks: &[place(
                    "b hotel Kaniwasou — 201 2BR Apt",
                    "1165 Miyajimacho Ebisumachi, Hatsukaichi, Hiroshima 739-0588",
                    "Check-in from 4pm",
                )],
            },
        ],
    },
    // Day 22 — Thu 5 Nov
    Day {
        city: "Tokyo",
        title: "Miyajima → Tokyo (Jack's birthday)",
        sections: &[
            Section {
                heading: "Travel",
                blocks: &[Text(
                    "Check out by 10am.\nFerry and train back to Hiroshima. Train back to Tokyo — 4hrs. Roughly 4 trains per hour, so no need to book.\n20 mins from Tokyo Station to the accommodation.",
                )],
            },
            Section {
                heading: "Accommodation",
                blocks: &[place(
                    "Family Stayさんのアパートメント, Sumida City",
                    "2-chōme-2-7 Tatekawa, Sumida City, Tokyo 130-0023",
                    "中村ビル · check-in after 4pm with lockbox",
                )],
            },
        ],
    },
    // Day 23 — Fri 6 Nov
    Day {
        city: "Tokyo",
        title: "Sumo",
        sections: &[
            Section {
                heading: "Suggested activities",
                blocks: &[List(&[
                    item("Skytree", ""),
                    item("Seria Marui Kinshichō", "Kitchenware shop"),
                    item("Kameido Gyoza Main Store", "Lunch spot"),
                    item("Solamachi", "Shopping centre"),
                ])],
            },
            Section {
                heading: "Dinner with sumo experience",
                blocks: &[
                    place(
                        "Yokozuna Tonkatsu Dosukoi Tanaka",
                        "3-1-11 Tatekawa, Sumida-ku, Tokyo",
                        "4 min walk from the apartment",
                    ),
                    Schedule(&[
                        step("18:45", "Open for entry", ""),
                        step("19:00", "Opening", "About sumo"),
                        step("19:10", "Show starts", ""),
                        step("19:50", "Challenge the wrestlers", ""),
                        step("20:30", "Photo & question time", ""),
                        step("21:00", "End of programme", ""),
                    ]),
                ],
            },
        ],
    },
    // Day 24 — Sat 7 Nov
    Day {
        city: "Tokyo",
        title: "Tori No Ichi",
        sections: &[
            Section {
                heading: "Suggested activities",
                blocks: &[List(&[
                    item("Sensō-ji", "Temple"),
                    item("dacō 御茶ノ水", "Bakery"),
                    item("Yanaka Ginza", "Old town shopping street"),
                    item("Jinbocho", "Book festival"),
                ])],
            },
            Section {
                heading: "Tori No Ichi",
                blocks: &[
                    Text(
                        "An annual traditional Japanese festival held in November to pray for good fortune, health and business prosperity. A lantern-lit night that feels like stepping into old Edo — all grilled squid, hot sake and ornate lucky rakes.",
                    ),
                    Schedule(&[step(
                        "15:00",
                        "Tori No Ichi",
                        "3–5pm. Best atmosphere after dark under the lanterns.",
                    )]),
                    place("Chokokuji Temple", "Chokokuji Temple, Asakusa, Tokyo", ""),
                    place("Otori Shrine", "Otori Shrine, Asakusa, Tokyo", ""),
                    Text(
                        "The signature item is the kumade — a decorative bamboo rake covered in lucky charms (koban gold coins, Okame masks, the Seven Gods of Fortune) meant to \"rake in\" wealth and luck.",
                    ),
                    Text(
                        "Eat everything. Yakisoba, takoyaki, grilled corn, chocolate bananas, hot sake in paper cups.",
                    ),
                ],
            },
            Section {
                heading: "",
                blocks: &[Text("Ben to leave around 8–9pm.")],
            },
        ],
    },
    // Day 25 — Sun 8 Nov
    Day {
        city: "Tokyo",
        title: "Flights",
        sections: &[Section {
            heading: "",
            blocks: &[
                Text(
                    "Check out at 10am. Need to find somewhere to store bags.\nLeave the accommodation around 2pm to get to the airport for 3:30–4pm. Can get the train or bus back.",
                ),
                image("/img/return-flights.webp", "Return flights"),
                image("/img/return-flights-times.webp", "Return flight times"),
            ],
        }],
    },
    // Day 26 — Mon 9 Nov
    Day {
        city: "Home",
        title: "Drive Home",
        sections: &[Section {
            heading: "",
            blocks: &[Text(
                "Parking shuttle pick-up from North Terminal bus stop 3. Parking is booked until 09:00.",
            )],
        }],
    },
];

pub const INFO: &[Section] = &[
    Section {
        heading: "Before we go",
        blocks: &[Links(&[
            link(
                "Data plan (Ubigi eSIM)",
                "https://cellulardata.ubigi.com/data-plans-and-coverage/ubigi-esim-data-plans/?destination=jpn&one-off=on&monthly=on&annual=on",
            ),
            link("Entry/exit form (Visit Japan Web)", "https://www.vjw.digital.go.jp/main/#/vjwpco001"),
            link(
                "Air China ticket validation",
                "https://www.airchina.com.cn/en-US/flight/query-services/air-ticket-verification",
            ),
            link(
                "Air China check-in (opens 36hrs before — 13th, 11pm)",
                "https://m.airchina.com.cn/ac/c/invoke/SeatcheckIn/qrySeatCheckIns@pg?comeFlag=showSeatCheckInsFlights&entryFlag=3&type=selectPerson",
            ),
            link(
                "Packing list",
                "https://docs.google.com/spreadsheets/d/1e-DINvKEhJGVDWHRux8yd8sh3Z3hVFILatizR2FtRhA/edit?usp=drive_link",
            ),
        ])],
    },
    Section {
        heading: "Disposable cameras",
        blocks: &[
            Text("Get cameras from Kitamura Camera (24 hour printing). One per category:"),
            Checklist(&[
                "1. Nice",
                "2. Tragedy/Funny",
                "3. Nature vs Skyscraper",
                "4. \"Only in Japan\"",
                "5. Pics with other people",
                "6. Food",
            ]),
        ],
    },
    Section {
        heading: "Rules to follow",
        blocks: &[List(&[
            item("No tipping", "This is rude"),
            item("Don't jaywalk", ""),
            item("Take rubbish home", "Bringing poop bags"),
            item("Take the receipt up to the till to pay", "Table paying isn't a thing"),
            item("Don't show shoulders or cleavage", ""),
            item("Don't go with promoters or street sellers!", ""),
        ])],
    },
    Section {
        heading: "Apps to download",
        blocks: &[List(&[
            item("Tabelog", "Food rating app. Anything with a score of 3.5+ is going to be excellent."),
            item("Klook", "Booked experiences and some transport"),
            item("Smart EX", "Book the bullet train, but you need an account. Easier to use Klook."),
            item("Yamato", "Luggage transfer"),
            item("Google Maps", ""),
            item("Google Translate", "With Japanese downloaded"),
            item("Stamp Quest", "Finding all the stamps"),
            item("GO", "Uber for Japan"),
            item("Luup", "Electric scooters"),
            item("Suica", "Transport card (Apple only)"),
            item("Uniqlo", "They have water filling stations"),
            item("LINE", "Lots of discounts"),
            item("Flush", "Shows all public toilets"),
            item("Payke", "Translation, shows what's in products, and has discounts"),
        ])],
    },
    Section {
        heading: "Flight survival kit",
        blocks: &[
            Text("Wear joggers, t-shirt, jumper."),
            Checklist(&[
                "Sliders",
                "Pants + socks",
                "Compression socks",
                "Neck pillow",
                "Eye mask",
                "Footrest",
                "Layers/blanket",
                "Passport",
                "Cash",
                "Itinerary",
                "Notebook + pen",
                "Snacks",
                "Water",
                "Cards",
                "Book",
                "Headphones",
                "Films and TV downloaded",
                "Power bank",
                "Charging cables",
                "Adaptor",
                "Phone stand",
                "Deodorant",
                "Lip balm",
                "Moisturiser",
                "Hand sanitiser",
                "Face wipes",
                "Toothbrush and toothpaste",
                "Paracetamol",
                "Salt tablets",
                "Plasters + waterproof",
                "Poop bags for rubbish",
            ]),
        ],
    },
];
