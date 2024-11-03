use std::collections::HashSet;
use std::error::Error;

use mongodb::bson::{doc, Document};
use mongodb::Cursor;
use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use mongodb::options::FindOptions;

use futures::stream::StreamExt;
use tokio;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct User {
    pub host: String,

    pub path: String,
    pub user: String,
    pub pass: String,
}

pub async fn searchLogin(
    url: &str,
    count: i32,
    collection: &str,
) -> Result<HashSet<User>, Box<dyn Error>> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    let client = Client::with_options(client_options)?;
    let database = client.database("login");
    let collection: Collection<User> = database.collection(collection);
    let filter = doc! { "host": url};
    let find_options = FindOptions::builder().limit(count as i64).build();
    let mut cursor = collection.find(filter, find_options).await?;
    let mut vec_hash_set = HashSet::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(user) => {
                vec_hash_set.insert(user);
            }
            Err(e) => {}
        }
    }
    Ok(vec_hash_set)
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct ser {
    pub email: String,
    // Outros campos que você precisa
}

pub async fn search_email_in_all_collections(
    email: &str,
    count: i32,
) -> Result<String, Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:9090").await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("leaks");

    let mut collections = database.list_collection_names(None).await?;

    let mut String = String::new();

    for collection_name in collections {
        let collection: Collection<Document> = database.collection(&collection_name);

        let filter = doc! { "email": email };
        let find_options = FindOptions::builder().limit(count as i64).build();
        let mut cursor = collection.find(filter, find_options).await?;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(user) => {
                    let mut create_string = String::new();

                    for (key, value) in user.iter() {
                        let entry = format!("{}: {}\n", key, value);
                        if key == "_id" {
                            continue;
                        }
                        create_string.push_str(&entry);
                    }
                    String.push_str(&format!("{}\n\n", create_string));
                }
                Err(e) => {
                    eprintln!("Erro ao buscar usuário: {}", e);
                }
            }
        }
    }

    Ok(String)
}

pub async fn search_email_to_url(email: &str, count: i32) -> Result<Vec<User>, Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("login");

    let collections = [
        "contry_com",
        "contry_net",
        "contry_br",
        "contry_in",
        "contry_org",
        "contry_id",
        "contry_fr",
        "contry_it",
        "contry_co",
        "contry_tv",
        "contry_io",
        "contry_vn",
        "contry_es",
        "contry_ar",
        "contry_mx",
        "contry_uk",
        "contry_de",
        "contry_tr",
        "contry_pl",
        "contry_pe",
        "contry_cl",
        "contry_ru",
        "contry_sa",
        "contry_me",
        "contry_th",
        "contry_eg",
        "contry_pk",
        "contry_us",
        "contry_hu",
        "contry_eu",
        "contry_ro",
        "contry_pt",
        "contry_ph",
        "contry_edu",
        "contry_ca",
        "contry_my",
        "contry_tw",
        "contry_kr",
        "contry_au",
        "contry_nl",
        "contry_ir",
        "contry_ve",
        "contry_bd",
        "contry_jp",
        "contry_ec",
        "contry_nz",
        "contry_1",
        "contry_za",
        "contry_cz",
        "contry_xyz",
        "contry_info",
        "contry_ma",
        "contry_gr",
        "contry_to",
        "contry_ng",
        "contry_ae",
        "contry_gg",
        "contry_cc",
        "contry_online",
        "contry_app",
        "contry_cn",
        "contry_dz",
        "contry_ke",
        "contry_gov",
        "contry_be",
        "contry_club",
        "contry_pro",
        "contry_ai",
        "contry_biz",
        "contry_bg",
        "contry_live",
        "contry_il",
        "contry_top",
        "contry_se",
        "contry_ua",
        "contry_sh",
        "contry_np",
        "contry_do",
        "contry_lt",
        "contry_bo",
        "contry_cloud",
        "contry_pw",
        "contry_tn",
        "contry_uy",
        "contry_sk",
        "contry_lk",
        "contry_rs",
        "contry_site",
        "contry_dk",
        "contry_vip",
        "contry_ch",
        "contry_at",
        "contry_fun",
        "contry_jo",
        "contry_bet",
        "contry_ee",
        "contry_ly",
        "contry_re",
        "contry_fm",
        "contry_hr",
        "contry_shop",
        "contry_sg",
        "contry_gh",
        "contry_tech",
        "contry_tz",
        "contry_lv",
        "contry_cr",
        "contry_ge",
        "contry_hk",
        "contry_games",
        "contry_no",
        "contry_network",
        "contry_gt",
        "contry_store",
        "contry_si",
        "contry_py",
        "contry_by",
        "contry_uz",
        "contry_mn",
        "contry_world",
        "contry_mobi",
        "contry_is",
        "contry_dev",
        "contry_link",
        "contry_asia",
        "contry_fi",
        "contry_ie",
        "contry_cat",
        "contry_kw",
        "contry_sbi",
        "contry_one",
        "contry_tk",
        "contry_win",
        "contry_space",
        "contry_su",
        "contry_game",
        "contry_pa",
        "contry_la",
        "contry_254",
        "contry_work",
        "contry_ws",
        "contry_global",
        "contry_ps",
        "contry_qa",
        "contry_ltd",
        "contry_sv",
        "contry_ug",
        "contry_so",
        "contry_host",
        "contry_local",
        "contry_jobs",
        "contry_rw",
        "contry_cu",
        "contry_ci",
        "contry_az",
        "contry_ml",
        "contry_red",
        "contry_2",
        "contry_hn",
        "contry_al",
        "contry_digital",
        "contry_mk",
        "contry_ba",
        "contry_ac",
        "contry_vc",
        "contry_academy",
        "contry_im",
        "contry_iq",
        "contry_ag",
        "contry_website",
        "contry_mz",
        "contry_xxx",
        "contry_life",
        "contry_bh",
        "contry_run",
        "contry_cash",
        "contry_casino",
        "contry_am",
        "contry_education",
        "contry_ao",
        "contry_zm",
        "contry_kz",
        "contry_bz",
        "contry_buzz",
        "contry_lat",
        "contry_exchange",
        "contry_ga",
        "contry_video",
        "contry_li",
        "contry_mu",
        "contry_zw",
        "contry_cm",
        "contry_email",
        "contry_android",
        "contry_wtf",
        "contry_aero",
        "contry_farm",
        "contry_trade",
        "contry_int",
        "contry_et",
    ];

    let mut String = Vec::new();
    for collection_name in collections {
        let collection: Collection<User> = database.collection(&collection_name);

        let filter = doc! { "user": email };
        let find_options = FindOptions::builder().limit(count as i64).build();
        let mut cursor = collection.find(filter, find_options).await?;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(user) => {
                    String.push(user);
                }
                Err(e) => {
                    eprintln!("Erro ao buscar usuário: {}", e);
                }
            }
        }
    }

    Ok(String)
}
