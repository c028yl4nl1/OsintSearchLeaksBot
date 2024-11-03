use std::{collections::HashSet, fs};

use serde_json::Value;
use teloxide::prelude::*;
pub fn run_bot() -> Bot {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["token_bot"].as_str().expect("Token ausente");

    let bot_runtime = teloxide::Bot::new(token_bot);
    bot_runtime
}

pub fn lang_welcome() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["welcome"].clone();

    let welcome = token_bot
        .as_str()
        .unwrap_or("error config json")
        .to_string();

    welcome
}

pub fn tools() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["tools"].clone();

    let welcome = token_bot
        .as_str()
        .unwrap_or("error config json")
        .to_string();

    welcome
}

pub fn support() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["support"].clone();

    let welcome = token_bot
        .as_str()
        .unwrap_or("error config json")
        .to_string();

    welcome
}
pub fn url() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["url"].clone();

    let welcome = token_bot
        .as_str()
        .unwrap_or("error config json")
        .to_string();

    welcome
}

pub fn mail() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["mail"].clone();

    let welcome = token_bot
        .as_str()
        .unwrap_or("error config json")
        .to_string();

    welcome
}

pub fn convite() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["convite"].clone();

    let welcome = token_bot
        .as_str()
        .unwrap_or("error config json")
        .to_string();

    welcome
}

pub fn saldo_inicial() -> i64 {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["saldo_inicial"].clone();

    let welcome = token_bot.as_i64().unwrap_or(12);

    welcome
}

pub fn saldo_convite() -> i64 {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["saldo_convite"].clone();

    let welcome = token_bot.as_i64().unwrap_or(35);

    welcome
}

pub fn id_owner() -> i64 {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["id_dono"].clone();

    let welcome = token_bot.as_i64().unwrap_or(35);

    welcome
}

pub fn pontos_para_realizar_consulta_url() -> i64 {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["pontos_consulta_url"].clone();

    let welcome = token_bot.as_i64().unwrap();

    welcome
}
pub fn pontos_subdomain() -> i64 {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["pontos_subdomain"].clone();

    let welcome = token_bot.as_i64().unwrap();

    welcome
}

pub fn pontos_cpanel() -> i64 {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["pontos_cpanel"].clone();

    let welcome = token_bot.as_i64().unwrap();

    welcome
}

pub fn pontos_para_realizar_consulta_email() -> i64 {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["pontos_colsulta_email01"].clone();

    let welcome = token_bot.as_i64().unwrap();

    welcome
}

pub fn saldo_convite_() -> i64 {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["saldo_convite"].clone();

    let welcome = token_bot.as_i64().unwrap();

    welcome
}


pub fn sem_saldo() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["insuficiente"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn buy_money() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["country"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn vk_data_base() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["VK"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn msg_database_help() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["database"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn Eye4Fraud() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["Eye4Fraud"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn piping() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["piping"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn mypertamina() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["mypertamina"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn linkedIn() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["linkedIn"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn livpure() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["livpure"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn hub() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["hub"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn habib() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["habib"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn zurich() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["zurich"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn zoomcar() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["zoomcar"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn zadig() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["zadig"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn younow() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["younow"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn xkcd() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["xkcd"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn email2() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["email2"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn brute() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["brute"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn left() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["left"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn add_your_chanell() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["add_your_chanell"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn cit0day() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["cit0day"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn shop() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["shop"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}

pub fn cpanelText() -> String {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["Cpanel"].clone();

    let welcome = token_bot.as_str().unwrap_or("Error json").to_string();

    welcome
}
pub fn oline_in_grups() -> bool {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["oline_in_grups"].clone();
    // oline_in_grups
    let welcome = token_bot.as_bool().unwrap_or(false);

    welcome
}
pub fn CpanelJson() -> Vec<String> {
    let key_json = fs::read_to_string("cpanel.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["Cpanel"].clone();
    let array = token_bot.as_array().unwrap();
    let mut prefix = Vec::new();
    let mut len = 98;
    for object in array {
        let host = object["host"].as_str().unwrap();

        if let Some(url_prefix) = extract_tld_suffix(host) {
            if prefix.contains(&url_prefix) {
                continue;
            }
            prefix.push(url_prefix);
            len -= 1;
            if len == 0 {
                break;
            }
        }
    }
    prefix
}

pub fn Cpanel_collect_document(preffix: &str) -> Option<Value> {
    let key_json = fs::read_to_string("cpanel.json").expect("Config ausente");

    let json = serde_json::from_str::<Value>(&key_json).expect("A config não está correta");

    let token_bot = json["Cpanel"].clone();
    let array = token_bot.as_array().unwrap();
    for object in array {
        let host = object["host"].as_str().unwrap();

        if let Some(url_prefix) = extract_tld_suffix(host) {
            if url_prefix.contains(&preffix) {
                return Some(object.to_owned());
            }
        }
    }
    None
}

pub fn removeLoginCpanel(host: &str, username: &str) {
    use serde_json::*;
    // Ler o arquivo JSON
    let key_json = fs::read_to_string("cpanel.json").expect("Config ausente");

    // Parsear o conteúdo do JSON
    let mut json = from_str::<Value>(&key_json).expect("A config não está correta");

    // Verificar se existe a chave "Cpanel" e se é uma lista
    if let Some(cpanel_array) = json["Cpanel"].as_array_mut() {
        // Manter todos os objetos, exceto aquele que corresponde ao host e username
        cpanel_array.retain(|obj| !(obj["host"] == host && obj["username"] == username));
    }

    // Converter de volta para string e salvar no arquivo
    let updated_data = to_string_pretty(&json).expect("Erro ao converter JSON para string");
    fs::write("cpanel.json", updated_data).expect("Erro ao salvar o arquivo JSON");
}

use serde_json::json;

use crate::tld_extract::{extract_tld, extract_tld_suffix};
pub fn oline_in_grups_config(valor: bool) {
    let key_json = fs::read_to_string("config.json").expect("Config ausente");

    let mut json: Value = serde_json::from_str(&key_json).expect("A config não está correta");

    json["oline_in_grups"] = json!(valor);

    // Salva as alterações de volta no arquivo
    let json_string =
        serde_json::to_string_pretty(&json).expect("Erro ao converter JSON para string");
    fs::write("config.json", json_string).expect("Erro ao salvar a configuração");
}
