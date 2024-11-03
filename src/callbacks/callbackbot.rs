use std::str::FromStr;

use teloxide::requests;
use teloxide::types::InlineKeyboardButton;
use teloxide::types::InlineKeyboardMarkup;
use teloxide::types::KeyboardButton;
use teloxide::types::LoginUrl;

use crate::runbot::CpanelJson;

pub fn start_inicial_bot() -> InlineKeyboardMarkup {
    let url_add_channell =
        reqwest::Url::from_str("http://t.me/OssintAndCheckBot?startchannel&admin=change_info+invite_users+restrict_members");
    let keyboard_markup = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::url(
            "➕ Add group ➕",
            url_add_channell.unwrap(),
        )],
        vec![
            InlineKeyboardButton::callback("tools / Ferramentas", "tools"),
            InlineKeyboardButton::callback("Suport", "support"),
        ],
        vec![InlineKeyboardButton::callback("🛒 Shop 🛒", "shop")],
        vec![
            InlineKeyboardButton::callback("Profile ", "profile"),
            InlineKeyboardButton::callback("💲Buy Points💲", "buy"),
        ],
        vec![InlineKeyboardButton::callback(" Databases ", "database")],
        
    ]);

    keyboard_markup
}
pub fn back_start() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "🔙 Back 🔙 ",
        "start",
    )]]);

    keyboard_markup
}

pub fn tools() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("url", "url"),
            InlineKeyboardButton::callback("Email 01", "mail"),
        ],
        vec![
            InlineKeyboardButton::callback("Users", "mail2"),
            InlineKeyboardButton::callback("Subdomain Search", "brute"),
        ],
        vec![InlineKeyboardButton::callback("🔙 Back 🔙 ", "start")],
    ]);
    keyboard_markup
}

pub fn url() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "🔙 Back 🔙 ",
        "tools",
    )]]);
    keyboard_markup
}
pub fn convite() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "🔙 Back 🔙 ",
        "start",
    )]]);
    keyboard_markup
}

pub fn Profile() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "🔙 Back 🔙 ",
        "start",
    )]]);
    keyboard_markup
}

pub fn delete_messagem() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "🗑️ Delete 🗑️",
        "delete",
    )]]);
    keyboard_markup
}

pub fn buy_money() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "💲Buy Points💲",
        "buy",
    )]]);
    keyboard_markup
}

pub fn payment() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("🇧🇷 Brasil 🇧🇷", "brasil"),
            InlineKeyboardButton::callback("🇺🇸 United States 🇺🇸", "us"),
        ],
        vec![InlineKeyboardButton::callback("🆓 Free points 🆓", "convite")],
        vec![InlineKeyboardButton::callback("🔙 Back 🔙 ", "start")],
    ]);
    keyboard_markup
}

pub fn back_buy() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "🔙 Back 🔙 ",
        "buy",
    )]]);
    keyboard_markup
}

pub fn databases_adcionadas() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("VK", "vk_leak"),
            InlineKeyboardButton::callback("Eye4Fraud", "fraud_leak"),
            InlineKeyboardButton::callback("LinkedIn", "linkedIn_leak"),
        ],
        vec![
            InlineKeyboardButton::callback("Piping Rock", "piping_leak"),
            InlineKeyboardButton::callback("MyPertamina", "mypertamina_leak"),
            InlineKeyboardButton::callback("LivPure", "livpure_leak"),
        ],
        vec![
            InlineKeyboardButton::callback("Hurb", "hub_leak"),
            InlineKeyboardButton::callback("Habib", "habib_leak"),
            InlineKeyboardButton::callback("Zurich", "zurich_leak"),
        ],
        vec![
            InlineKeyboardButton::callback("Zadig & Voltaire", "zadig_leak"),
            InlineKeyboardButton::callback("Cit0day", "cit0day"),
        ],
        vec![
            InlineKeyboardButton::callback("Zoomcar", "zoomcar_leak"),
            InlineKeyboardButton::callback("YouNow", "younow_leak"),
            InlineKeyboardButton::callback("XKCD", "xkcd_leak"),
        ],
        vec![InlineKeyboardButton::callback("🔙 Back 🔙 ", "start")],
    ]);

    keyboard_markup
}

pub fn back_databases() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "🔙 Back 🔙 ",
        "database",
    )]]);
    keyboard_markup
}
pub fn admin_buttons() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            "Total de Usuarios",
            "total_user",
        )],
        vec![InlineKeyboardButton::callback(
            "Informaçoes do sistema",
            "sistem_user",
        )],
    ]);
    keyboard_markup
}

pub fn shop() -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("📨 Cpanel and Webmail 📨", "cpanel"),
            
        ],
        vec![InlineKeyboardButton::callback("🔙 Back 🔙 ", "start")],
    ]);
    keyboard_markup
}

pub fn healp_bunton(data_callback: &str) -> InlineKeyboardMarkup {
    let keyboard_markup = InlineKeyboardMarkup::new(vec![
        
        vec![InlineKeyboardButton::callback("📖 Help 📖",
        data_callback)],
    ]);
    

    keyboard_markup
}

pub fn CpanelView() -> InlineKeyboardMarkup {
    let hash = CpanelJson();
    let mut vec = Vec::new();
    let mut vec_2 = Vec::new();
    let mut count_vec = 5;

    for prefix in hash {
        if count_vec > 0 {
            let input = InlineKeyboardButton::callback(&prefix, format!("cpanel_{}", prefix));
            vec_2.push(input);
            count_vec -= 1;
            continue;
        }

        vec.push(vec_2.clone());
        vec_2.clear();

        count_vec += 6;
    }
    vec.push(vec![InlineKeyboardButton::callback("🔙 Back 🔙 ", "shop")]);
    let keyboard_markup = InlineKeyboardMarkup::new(vec);
    keyboard_markup
}

pub fn admin_allow_permit_grupos(text: &str) -> InlineKeyboardMarkup {
    let keyboard_markup =
        InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(text, text)]]);
    keyboard_markup
}
