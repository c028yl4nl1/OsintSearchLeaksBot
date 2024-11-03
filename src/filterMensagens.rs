use std::{fs, io::Write};

use crate::{
    mongodbConnect::{self, searchLogin},
    runbot,
    tld_extract::extract_tld,
    users,
    SubdomainFinder::SubdmainGetURlApi,
};

pub fn create_gift_Filter(msg: &str, id: i64) -> Option<String> {
    let id_dono = runbot::id_owner();
    if id_dono == id {
        let split_msg: Vec<&str> = msg.split_ascii_whitespace().collect();
        if split_msg.len() == 1 {
            return None;
        } else {
            let number = split_msg[1].parse::<i64>().unwrap_or(0);
            let gift = users::adicionar_gift(number);
            return Some(format!(
                "Gift Create\nValue: {}\nCopy and Paste: <pre><code>/gift {}</code></pre>\n\nBot: @OssintAndCheckBot 
",
                number, gift
            ));
        }
    }
    None
}

pub fn use_gift(msg: &str, id: i64) -> Option<String> {
    let gift: Vec<&str> = msg.split_ascii_whitespace().collect();
    if gift.len() == 1 {
        return None;
    } else {
        let gift = gift[1];

        if let Some(gift) = users::consultar_gift(gift) {
            let saldo = gift.valor;
            if users::atualizar_saldo(id, saldo) {
                users::apagar_gift(&gift.code);
                return Some(format!(
                    "Congratulations! You have redeemed your Poits of {} . Feel free to use it.",
                    saldo
                ));
            }
        }
    }

    None
}

use regex::Regex;
use reqwest::Url;

fn extract_domain(url: &str) -> Option<String> {
    let re = Regex::new(r"(?:https?:\/\/)?((?:[\w-]+\.)+[\w-]+)").unwrap();

    if let Some(captures) = re.captures(url) {
        Some(captures[1].to_string()) // Captura o domínio e subdomínio
    } else {
        None
    }
}

fn salveFile(name: &str, buffer: String) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(name)
        .unwrap();
    file.write(buffer.as_bytes());
}

pub async fn searchlogin_url(msg: &str, count: i64) -> Option<String> {
    let split: Vec<&str> = msg.split_ascii_whitespace().collect();

    if split.len() == 1 {
        return None;
    }
    match extract_domain(split[1]) {
        Some(url) => {
            let split: Vec<&str> = url.split(".").collect();

            if split.len() == 1 {
                return None;
            }
            let size_len = split.len();

            let collection = format!("contry_{}", split[size_len - 1]);

            let valor =
                mongodbConnect::searchLogin(&url.to_lowercase(), count as i32, &collection).await;
            match valor {
                Ok(hashset) => {
                    let name_file = format!("{}.txt", url);
                    let mut buffer_string = String::new();
                    for user in hashset {
                        let user = format!(
                            "\nURL: {}/{}\nUSERNAME: {}\nPASSWORD: {}\n",
                            user.host, user.path, user.user, user.pass
                        );
                        buffer_string.push_str(&user);
                    }
                    if buffer_string.len() < 1 {
                        return None;
                    }
                    salveFile(&name_file, buffer_string);
                    return Some(name_file);
                }

                _ => {
                    return None;
                }
            }
        }

        _ => {}
    }

    None
}

pub async fn search_mail_01(emai: &str, count: i32) -> Option<String> {
    let v: Vec<&str> = emai.split_ascii_whitespace().collect();
    if v.len() == 1 {
        return None;
    }
    let emai = v[1];

    match mongodbConnect::search_email_in_all_collections(emai, count).await {
        Ok(mut buffer) => {
            if buffer.len() < 5 {
                return None;
            } else {
                let filename = format!("{}.txt", emai);
                buffer.push_str("\n\n@forclogs\nhttps://t.me/OssintAndCheckBot\n\n");
                salveFile(&filename, buffer);
                return Some(filename);
            }
        }
        _ => {}
    }
    None
}

pub async fn searchlogin_email02(email: &str, count: i64) -> Option<String> {
    let split: Vec<&str> = email.split_ascii_whitespace().collect();

    if split.len() == 1 {
        return None;
    }
    let valor = mongodbConnect::search_email_to_url(split[1], count as i32).await;
    match valor {
        Ok(hashset) => {
            if hashset.len() < 1 {
                return None;
            }
            let name_file = format!("{}.txt", split[1]);
            let mut buffer_string = String::new();
            for user in hashset {
                let user = format!(
                    "\nURL: {}/{}\nUSERNAME: {}\nPASSWORD: {}\n",
                    user.host, user.path, user.user, user.pass
                );

                buffer_string.push_str(&user);
            }

            salveFile(&name_file, buffer_string);
            return Some(name_file);
        }

        _ => {
            return None;
        }
    }

    None
}

#[derive(Debug)]

pub struct SubResponse {
    pub Domains: String,
    pub total: i32,
    pub pathFile: String,
}
pub async fn Subdomainfinders(msg: &str, count: i32) -> Option<SubResponse> {
    let split: Vec<&str> = msg.split_whitespace().collect();
    if split.len() == 2 {
        let url = split[1].to_ascii_lowercase();
        let url = extract_domain(&url).unwrap_or(url);
        let url = extract_tld(&url).unwrap_or(url);
        let collection: Vec<&str> = url.split(".").collect();
        let mut collection_contry = String::new();
        if collection.len() >= 2 {
            let len = collection.len();

            let collection = collection[len - 1];
            collection_contry.push_str(&format!("contry_{}", collection));
        } else {
            return None;
        }

        let mut domains_string = String::new();
        let mut buffer_domains2 = String::new();
        let mut totalView = 0;
        match SubdmainGetURlApi(&url).await {
            Some(sub) => {
                let sub = sub;
                let is_sucess = sub.success;
                if is_sucess {
                    let mut vec_hash = Vec::new();
                    for subdomain in sub.subdomains {
                        let value =
                            searchLogin(&subdomain.subdomain, count, &collection_contry).await;
                        totalView += 1;
                        if totalView < 20 {
                            domains_string.push_str(&format!("\n{}", &subdomain.subdomain));
                        }
                        buffer_domains2.push_str(&format!("{}\n", &subdomain.subdomain));

                        match value {
                            Ok(hash_set) => {
                                for user in hash_set {
                                    vec_hash.push(user);
                                }
                            }
                            _ => {}
                        }
                    }

                    if vec_hash.len() < 1 {
                        return None;
                    }
                    let mut buffer_salve_file = String::new();
                    for user in vec_hash {
                        let user = format!(
                            "\nURL: {}/{}\nUSERNAME: {}\nPASSWORD: {}\n\n",
                            user.host, user.path, user.user, user.pass
                        );

                        buffer_salve_file.push_str(&user);
                    }
                    buffer_salve_file.push_str(&buffer_domains2);
                    let filename = format!("{}.txt", &url);
                    salveFile(&filename, buffer_salve_file);

                    let option_success = SubResponse {
                        Domains: domains_string,
                        total: sub.count as i32,
                        pathFile: filename,
                    };
                    return Some(option_success);
                }
            }

            _ => {}
        }
    }

    None
}
