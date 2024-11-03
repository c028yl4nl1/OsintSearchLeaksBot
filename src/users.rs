use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs::{self, File};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub saldo: i64,
    pub data_de_cadastro: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserList {
    users: Vec<User>,
}

// Função para salvar os usuários no arquivo
fn save_to_file(users: &UserList, filename: &str) {
    let json_data = serde_json::to_string_pretty(users).expect("Erro ao formatar o JSON");
    let mut file = File::create(filename).expect("Erro ao criar o arquivo");
    file.write_all(json_data.as_bytes())
        .expect("Erro ao gravar no arquivo");
}

// Função para carregar os usuários do arquivo, criando um novo arquivo se não existir
fn load_from_file(filename: &str) -> UserList {
    if let Ok(mut file) = File::open(filename) {
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();
        serde_json::from_str(&json_str).unwrap_or_else(|_| UserList { users: vec![] })
    } else {
        // Se o arquivo não existir, cria um novo com uma estrutura vazia
        let empty_list = UserList { users: vec![] };
        save_to_file(&empty_list, filename);
        empty_list
    }
}

// Função para adicionar um novo usuário ao arquivo JSON
pub fn create_new_user(user: User) {
    let mut user_list = load_from_file("users.json");

    // Verifica se o usuário já existe pelo ID
    if user_list.users.iter().any(|u| u.id == user.id) {
    } else {
        // Adiciona o novo usuário à lista
        user_list.users.push(user);
        save_to_file(&user_list, "users.json");
    }
}

// Função para verificar se o usuário existe pelo ID
pub fn verificar_user_existe(id: i64) -> Option<User> {
    let file = load_from_file("users.json");
    file.users.into_iter().find(|user| user.id == id)
}

// Função para atualizar o saldo de um usuário
pub fn atualizar_saldo(id: i64, novo_saldo: i64) -> bool {
    let mut user_list = load_from_file("users.json");

    // Busca o usuário pelo ID e atualiza o saldo
    if let Some(user) = user_list.users.iter_mut().find(|u| u.id == id) {
        user.saldo += novo_saldo;
        save_to_file(&user_list, "users.json");
        true
    } else {
        false
    }
}

pub fn atualizar_saldo_menos(id: i64, novo_saldo: i64) -> bool {
    let mut user_list = load_from_file("users.json");

    // Busca o usuário pelo ID e atualiza o saldo
    if let Some(user) = user_list.users.iter_mut().find(|u| u.id == id) {
        user.saldo -= novo_saldo;
        save_to_file(&user_list, "users.json");
        true
    } else {
        false
    }
}

use rand::{distributions::Alphanumeric, Rng};

#[derive(Serialize, Deserialize, Debug)]
pub struct Gift {
    pub code: String,
    pub valor: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct GiftList {
    gifts: Vec<Gift>,
}

// Função para salvar os gifts no arquivo JSON
fn save_gifts_to_file(gifts: &GiftList, filename: &str) {
    let json_data = serde_json::to_string_pretty(gifts).expect("Erro ao formatar o JSON");
    let mut file = File::create(filename).expect("Erro ao criar o arquivo");
    file.write_all(json_data.as_bytes())
        .expect("Erro ao gravar no arquivo");
}

// Função para carregar os gifts do arquivo JSON
fn load_gifts_from_file(filename: &str) -> GiftList {
    if let Ok(mut file) = File::open(filename) {
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();
        serde_json::from_str(&json_str).unwrap_or_else(|_| GiftList { gifts: vec![] })
    } else {
        let empty_list = GiftList { gifts: vec![] };
        save_gifts_to_file(&empty_list, filename);
        empty_list
    }
}

// Função para gerar um código de gift aleatório
fn gerar_gift() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10) // Gera uma string de 10 caracteres aleatórios
        .map(char::from)
        .collect()
}

pub fn adicionar_gift(valor: i64) -> String {
    let mut gift_list = load_gifts_from_file("gifts.json");
    let gift = gerar_gift();
    let novo_gift = Gift {
        code: gift.clone(),
        valor,
    };

    gift_list.gifts.push(novo_gift);
    save_gifts_to_file(&gift_list, "gifts.json");

    gift
}

// Função para consultar um gift pelo código
pub fn consultar_gift(code: &str) -> Option<Gift> {
    let gift_list = load_gifts_from_file("gifts.json");
    gift_list.gifts.into_iter().find(|gift| gift.code == code)
}

// Função para apagar um gift pelo código
pub fn apagar_gift(code: &str) -> bool {
    let mut gift_list = load_gifts_from_file("gifts.json");

    let tamanho_inicial = gift_list.gifts.len();
    gift_list.gifts.retain(|gift| gift.code != code);

    if gift_list.gifts.len() < tamanho_inicial {
        save_gifts_to_file(&gift_list, "gifts.json");
        true
    } else {
        false
    }
}

pub fn ver_todos_id() -> Vec<i64> {
    let user_list = load_from_file("users.json");
    let mut Vec_users_id = Vec::new();
    let users = user_list.users;
    for user in users {
        let id = user.id;
        Vec_users_id.push(id);
    }

    Vec_users_id
}
