use std::any::Any;
use std::fmt::format;
use std::process::id;

use leak::callbacks;
use leak::filterMensagens;
use leak::runbot;
use leak::runbot::add_your_chanell;
use leak::runbot::id_owner;
use leak::runbot::left;
use leak::runbot::pontos_cpanel;
use leak::runbot::removeLoginCpanel;
use leak::runbot::run_bot;
use leak::runbot::saldo_inicial;
use leak::runbot::Cpanel_collect_document;
use leak::users::atualizar_saldo;
use leak::users::atualizar_saldo_menos;
use leak::users::ver_todos_id;
use leak::users::verificar_user_existe;
use leak::GrupsPermitid;
use mongodb::bson::document;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::dptree::case;
use teloxide::dptree::entry;
use teloxide::payloads::SendMessageSetters;
use teloxide::types::ChatKind;
use teloxide::types::InputFile;
use teloxide::types::MessageCommon;
use teloxide::types::MessageKind;
use teloxide::types::PublicChatGroup;
use teloxide::types::ReplyParameters;
use teloxide::types::User;
use teloxide::ApiError;
use teloxide::RequestError;
use teloxide::{
    prelude::*,
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup},
};
use tokio::fs;
use tokio::time::{sleep, Duration};
pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    if let Err(e) = set_max_open_files(65535) {
        eprintln!("Erro ao ajustar o limite de arquivos abertos: {}", e);
    }

    let bot = leak::runbot::run_bot();
    let schema = Update::filter_message()
        .filter_map(|update: Update| update.from().cloned())
        .branch(Message::filter_text().endpoint(
            |bot, user, message_text, msg: Message| async move {
                // Cria uma nova tarefa para cada mensagem recebida
                tokio::spawn(async move {
                    if let Err(e) = process_text_message(bot, user, message_text, msg).await {}
                });

                Ok::<(), Error>(()) // Retorna Ok() para o endpoint
            },
        ));

    let schema2 = Update::filter_callback_query().endpoint(|bot, query| async move {
        if let Err(e) = handle_callback(bot, query).await {
            eprintln!("Error ao processar callback: {}", e);
        } else {
            // println!("{}", "Foi clicado");
        }
        Ok::<(), Error>(())
    });

    //filter_chat_join_request
    // filter_chat_member

    // filter_my_chat_member
    let schema3 =
        Update::filter_my_chat_member().endpoint(|bot: Bot, msg: ChatMemberUpdated| async move {
            add_gruop(bot, msg).await;
            Ok::<(), Error>(())
        });

    loop {
        match Dispatcher::builder(
            bot.clone(),
            entry()
                .branch(schema.clone())
                .branch(schema2.clone())
                .branch(schema3.clone()),
        )
        .build()
        .dispatch()
        .await
        {
            _ => {}
        }
    }

    Ok(())
}

async fn add_gruop(bot: Bot, msg: ChatMemberUpdated) {
    let new = msg.new_chat_member;

    let id_chat_from_remove = msg.from.id;
    let id_chat = msg.chat.id;

    let fisrt_name = msg.from.full_name();

    let user = new.is_left();
    if user {
        bot.send_message(id_chat_from_remove, left()).await;
        return;
    }

    bot.send_message(id_chat, add_your_chanell()).await;
    let text = msg.chat.kind;

    match text {
        ChatKind::Public(valor) => {
            let total = bot.get_chat_member_count(id_chat).await;

            let var = "unknow".to_string();
            let title = valor.title.unwrap_or(var.clone());
            let link = valor.invite_link.unwrap_or(var.clone());
            let description = valor.description.unwrap_or(var.clone());

            bot.send_message(
                ChatId(id_owner().into()),
                format!(
                    "New Add Group: {}\n\nNome: {}\nLink: {}\nDescrição: {}\nTotal De usuarios: {}",
                    title,
                    fisrt_name,
                    link,
                    description,
                    total.unwrap_or(0)
                ),
            )
            .await;
        }

        _ => {}
    }

    return;
}

async fn process_text_message(
    bot: Bot,
    user: User,
    message_text: String,
    msg: Message,
) -> Result<(), Error> {
    logs(&msg);
    
    if let Some(text) = msg.text(){
        if text.contains("/start "){
            let split: Vec<&str> =  text.split(" ").collect();
            let len = split.len();

            if len > 1{
            let id_pessoa_que_convidou: i64 = split[1].parse().unwrap_or(0);
            let id = msg.chat.id.0;
            let user_new = leak::users::verificar_user_existe(id);

            if user_new.is_none() {
                let first_name = user.first_name.clone();
                let id = user.id.0 as i64;
                let username = user.username.clone().unwrap_or("unknow".to_string());
                let saldo = saldo_inicial();
                let date_and_houra = format!("{} > {}", msg.date.date_naive(), msg.date.time());
                let user = leak::users::User {
                    username: username,
                    id: id,
                    name: first_name,
                    data_de_cadastro: date_and_houra,
                    saldo: saldo,
                };
                leak::users::create_new_user(user);


                if let Some(user) = leak::users::verificar_user_existe(id_pessoa_que_convidou){
                    let keyboard_start_inicial = leak::callbacks::callbackbot::start_inicial_bot();
                    bot.send_message(msg.chat.id, leak::runbot::lang_welcome())
                .reply_markup(keyboard_start_inicial)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
                    let convite_pontos = runbot::saldo_convite_();
                    atualizar_saldo(id_pessoa_que_convidou, convite_pontos );
                    bot.send_message(ChatId(id_pessoa_que_convidou), format!("You received {} points for sharing", convite_pontos))
                    .reply_markup(callbacks::callbackbot::delete_messagem())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                }

            }
            else {
                let keyboard_start_inicial = leak::callbacks::callbackbot::start_inicial_bot();
                    bot.send_message(msg.chat.id, leak::runbot::lang_welcome())
                .reply_markup(keyboard_start_inicial)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
                return Ok(());
            }
            }
        }

        
    }

    match message_text.as_str() {
        "/start" | "/start@OssintAndCheckBot" | "/menu" | "/help" | "/ajuda" => {
            let id = msg.chat.id.0;
            let user_new = leak::users::verificar_user_existe(id);

            if user_new.is_none() {
                let first_name = user.first_name;
                let id = user.id.0 as i64;
                let username = user.username.unwrap_or("unknow".to_string());
                let saldo = saldo_inicial();
                let date_and_houra = format!("{} > {}", msg.date.date_naive(), msg.date.time());
                let user = leak::users::User {
                    username: username,
                    id: id,
                    name: first_name,
                    data_de_cadastro: date_and_houra,
                    saldo: saldo,
                };
                leak::users::create_new_user(user);
            }
            let keyboard_start_inicial = leak::callbacks::callbackbot::start_inicial_bot();
            bot.send_message(msg.chat.id, leak::runbot::lang_welcome())
                .reply_markup(keyboard_start_inicial)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
        }
        "/url" => {
            bot.send_message(msg.chat.id, "Click Help").reply_markup(callbacks::callbackbot::healp_bunton("url")).await;
            return Ok(());
        }
        "/email1" => {
            bot.send_message(msg.chat.id, "Click Help").reply_markup(callbacks::callbackbot::healp_bunton("mail")).await;
            return Ok(());
        }
        "/email2" => {
            bot.send_message(msg.chat.id, "Click Help").reply_markup(callbacks::callbackbot::healp_bunton("mail2")).await;
            return Ok(());
        }
        "/brute" => {
            bot.send_message(msg.chat.id, "Click Help").reply_markup(callbacks::callbackbot::healp_bunton("brute")).await;
            return Ok(());
        }
        "/on" => {
            if msg.from.unwrap().id.0 as i64 == runbot::id_owner() {
                let chat_id = msg.chat.id;
                let ver = runbot::oline_in_grups();
                let mut string = String::new();
                if ver {
                    string.push_str("❌ Offline ❌");
                    bot.send_message(chat_id, format!("Desligar"))
                        .reply_markup(callbacks::callbackbot::admin_allow_permit_grupos(&string))
                        .await;
                } else {
                    string.push_str("✅ Online ✅");
                    bot.send_message(chat_id, format!("Ligar"))
                        .reply_markup(callbacks::callbackbot::admin_allow_permit_grupos(&string))
                        .await;
                }
            }
            return Ok(());
        }

        _ => {
            let text = msg.text().unwrap_or("");
            if text.contains("/create") {
                let create_gift = filterMensagens::create_gift_Filter(text, user.id.0 as i64);
                bot.send_message(
                    msg.chat.id,
                    create_gift.unwrap_or("You not adm".to_string()),
                )
                .reply_parameters(ReplyParameters::new(msg.id))
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
                return Ok(());
            }

            if text.contains("/gift") {
                let create_gift = filterMensagens::use_gift(text, user.id.0 as i64);

                bot.send_message(
                    msg.chat.id,
                    create_gift.clone().unwrap_or("Gift Not exist".to_string()),
                )
                .reply_parameters(ReplyParameters::new(msg.id))
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
                if let Some(OPT) = create_gift {
                    let user = msg.chat.id;
                    let user = verificar_user_existe(user.0);
                    if let Some(StructUser) = user {
                        let username = StructUser.username;
                        let id = StructUser.id;
                        let firstName = StructUser.name;
                        let format = format!("Usuario adicionou pontos:\n\nUsername:@{}\n\nId:{}\n\nFirstName:{}\n\n{}", username, id , firstName, OPT);
                        bot.send_message(ChatId(id_owner()), format).await;
                    }
                }

                return Ok(());
            }

            if text.contains("/url") {
                let chat_id = msg.chat.id;

                if msg.chat.is_supergroup() {
                    if runbot::oline_in_grups() {
                        if let Some(valor) = leak::filterMensagens::searchlogin_url(text, 30).await
                        {
                            bot.send_document(msg.chat.id, InputFile::file(valor.clone()))
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .reply_markup(leak::callbacks::callbackbot::delete_messagem()).caption("Há um limite de informações, apenas 30 logins. Compre o acesso e você não terá mais limites.\n\nThere’s a limit of information, only 30 logins. Buy the access, and you won’t have any more limits.")
                            .await;
                            //println!("Filename: ");
                            fs::remove_file(valor).await;
                            return Ok(());
                        }
                        let msg_result = bot
                            .send_message(chat_id, "Sorry not found")
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await;
                        if let Ok(id_msg) = msg_result {
                            let sleep_durration = std::time::Duration::from_secs(3);
                            std::thread::sleep(sleep_durration);
                            let id = id_msg.id;
                            bot.delete_message(msg.chat.id, id).await;
                        }
                        return Ok(());
                    }
                }

                if msg.chat.is_chat() {
                    // privado
                    bot.send_message(chat_id.clone(), "🔍").await;
                    let pontos_url = leak::runbot::pontos_para_realizar_consulta_url();
                    match verificar_user_existe(chat_id.0) {
                        Some(user) => {
                            if user.saldo >= pontos_url {
                                if let Some(valor) =
                                    leak::filterMensagens::searchlogin_url(text, 15000).await
                                {
                                    bot.send_document(msg.chat.id, InputFile::file(valor.clone()))
                                        .reply_parameters(ReplyParameters::new(msg.id))
                                        .reply_markup(
                                            leak::callbacks::callbackbot::delete_messagem(),
                                        )
                                        .await;
                                    fs::remove_file(valor).await;
                                    leak::users::atualizar_saldo_menos(chat_id.0, pontos_url);

                                    return Ok(());
                                }

                                bot.send_message(chat_id, "Sorry not found")
                                    .reply_parameters(ReplyParameters::new(msg.id))
                                    .await;

                                return Ok(());
                            } else {
                                bot.send_message(
                                    chat_id,
                                    format!(
                                        "{}\n\nValue {} Points",
                                        leak::runbot::sem_saldo(),
                                        pontos_url
                                    ),
                                )
                                .reply_parameters(ReplyParameters::new(msg.id))
                                .reply_markup(leak::callbacks::callbackbot::buy_money())
                                .await;
                                return Ok(());
                            }
                        }
                        _ => {
                            bot.send_message(
                                chat_id,
                                "Buy points to make queries to the bot, I am not currently operating in example mode\n\nCompre pontos para fazer consultas ao bot, não estou funcionando no modo de exemplo no momento\n\nКупите баллы, чтобы делать запросы к боту, в данный момент я не работаю в режиме примера\n\nBuy points: @forclogs\n\n/start",
                            )
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await;
                            return Ok(());
                        }
                    }
                }
            }

            if text.contains("/email1") {
                let chat_id = msg.chat.id;
                if msg.chat.is_supergroup() {
                    if runbot::oline_in_grups() {
                        if let Some(valor) = leak::filterMensagens::search_mail_01(text, 2).await {
                            bot.send_document(msg.chat.id, InputFile::file(valor.clone()))
                                .reply_parameters(ReplyParameters::new(msg.id))
                                .reply_markup(leak::callbacks::callbackbot::delete_messagem())
                                .await;
                            fs::remove_file(valor).await;
                            return Ok(());
                        }
                        let msg_result = bot
                            .send_message(chat_id, "Sorry not found")
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await;

                        if let Ok(id_msg) = msg_result {
                            let sleep_durration = std::time::Duration::from_secs(3);
                            std::thread::sleep(sleep_durration);
                            let id = id_msg.id;
                            bot.delete_message(msg.chat.id, id).await;
                        }
                        return Ok(());
                    }
                }

                if msg.chat.is_chat() {
                    bot.send_message(chat_id.clone(), "🔍").await;
                    let pontos_email = leak::runbot::pontos_para_realizar_consulta_email();
                    match verificar_user_existe(chat_id.0) {
                        Some(user) => {
                            if user.saldo >= pontos_email {
                                if let Some(valor) =
                                    leak::filterMensagens::search_mail_01(text, 50).await
                                {
                                    bot.send_document(msg.chat.id, InputFile::file(valor.clone()))
                                        .reply_parameters(ReplyParameters::new(msg.id))
                                        .reply_markup(
                                            leak::callbacks::callbackbot::delete_messagem(),
                                        )
                                        .await;
                                    fs::remove_file(valor).await;
                                    leak::users::atualizar_saldo_menos(chat_id.0, pontos_email);

                                    return Ok(());
                                }
                                bot.send_message(chat_id, "Sorry not found")
                                    .reply_parameters(ReplyParameters::new(msg.id))
                                    .await;
                                return Ok(());
                            } else {
                                bot.send_message(
                                    chat_id,
                                    format!(
                                        "{}\n\nValue {} Points",
                                        leak::runbot::sem_saldo(),
                                        pontos_email
                                    ),
                                )
                                .reply_parameters(ReplyParameters::new(msg.id))
                                .reply_markup(leak::callbacks::callbackbot::buy_money())
                                .await;
                                return Ok(());
                            }
                        }
                        _ => {
                            bot.send_message(
                                chat_id,
                                "Buy points to make queries to the bot, I am not currently operating in example mode\n\nCompre pontos para fazer consultas ao bot, não estou funcionando no modo de exemplo no momento\n\nКупите баллы, чтобы делать запросы к боту, в данный момент я не работаю в режиме примера\n\nBuy points: @forclogs\n\n/start",
                            )
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await;
                            return Ok(());
                        }
                    }
                }
            }

            if text.contains("/email2") {
                let chat_id = msg.chat.id;
                if msg.chat.is_supergroup() {
                    if runbot::oline_in_grups() {
                        if let Some(valor) =
                            leak::filterMensagens::searchlogin_email02(text, 2).await
                        {
                            bot.send_document(msg.chat.id, InputFile::file(valor.clone()))
                                .reply_parameters(ReplyParameters::new(msg.id))
                                .reply_markup(leak::callbacks::callbackbot::delete_messagem())
                                .await;
                            fs::remove_file(valor).await;
                            return Ok(());
                        }
                        let msg_result = bot
                            .send_message(chat_id, "Sorry not found")
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await;

                        if let Ok(id_msg) = msg_result {
                            let sleep_durration = std::time::Duration::from_secs(3);
                            std::thread::sleep(sleep_durration);
                            let id = id_msg.id;
                            bot.delete_message(msg.chat.id, id).await;
                        }
                        return Ok(());
                    }
                }
                if msg.chat.is_chat() {
                    bot.send_message(chat_id.clone(), "🔍").await;
                    let pontos_email = leak::runbot::pontos_para_realizar_consulta_email();
                    match verificar_user_existe(chat_id.0) {
                        Some(user) => {
                            if user.saldo >= pontos_email {
                                if let Some(valor) =
                                    leak::filterMensagens::searchlogin_email02(text, 50).await
                                {
                                    bot.send_document(msg.chat.id, InputFile::file(valor.clone()))
                                        .reply_parameters(ReplyParameters::new(msg.id))
                                        .reply_markup(
                                            leak::callbacks::callbackbot::delete_messagem(),
                                        )
                                        .await;
                                    fs::remove_file(valor).await;
                                    leak::users::atualizar_saldo_menos(chat_id.0, pontos_email);

                                    return Ok(());
                                }
                                bot.send_message(chat_id, "Sorry not found")
                                    .reply_parameters(ReplyParameters::new(msg.id))
                                    .await;
                                return Ok(());
                            } else {
                                bot.send_message(
                                    chat_id,
                                    format!(
                                        "{}\n\nValue {} Points",
                                        leak::runbot::sem_saldo(),
                                        pontos_email
                                    ),
                                )
                                .reply_parameters(ReplyParameters::new(msg.id))
                                .reply_markup(leak::callbacks::callbackbot::buy_money())
                                .await;
                                return Ok(());
                            }
                        }
                        _ => {
                            bot.send_message(
                                chat_id,
                                "Buy points to make queries to the bot, I am not currently operating in example mode\n\nCompre pontos para fazer consultas ao bot, não estou funcionando no modo de exemplo no momento\n\nКупите баллы, чтобы делать запросы к боту, в данный момент я не работаю в режиме примера\n\nBuy points: @forclogs\n\n/start",
                            )
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await;
                            return Ok(());
                        }
                    }
                }
            }

            if text.contains("/brute") | text.contains("/subdomain") {
                let chat_id = msg.chat.id;
                if msg.chat.is_supergroup() {
                    if runbot::oline_in_grups() {
                        if let Some(valor) =
                            leak::filterMensagens::Subdomainfinders(text, 100).await
                        {
                            let bot_send = bot
                                .send_document(msg.chat.id, InputFile::file(valor.pathFile.clone()))
                                .reply_parameters(ReplyParameters::new(msg.id))
                                .reply_markup(leak::callbacks::callbackbot::delete_messagem())
                                .caption(format!(
                                    "Total Subdomain: {}\n\n{}",
                                    valor.total, valor.Domains
                                ))
                                .await;
                            if let Err(_) = bot_send {
                                bot.send_message(chat_id, "Sorry not found")
                                    .reply_parameters(ReplyParameters::new(msg.id))
                                    .await;
                            }
                            fs::remove_file(valor.pathFile).await;
                            return Ok(());
                        }
                        let msg_result = bot
                            .send_message(chat_id, "Sorry not found")
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await;

                        if let Ok(id_msg) = msg_result {
                            let sleep_durration = std::time::Duration::from_secs(3);
                            std::thread::sleep(sleep_durration);
                            let id = id_msg.id;
                            bot.delete_message(msg.chat.id, id).await;
                        }
                        return Ok(());
                    }
                }

                if msg.chat.is_chat() {
                    let pontos_email = leak::runbot::pontos_subdomain();
                    bot.send_message(chat_id.clone(), "🔍").await;
                    match verificar_user_existe(chat_id.0) {
                        Some(user) => {
                            if user.saldo >= pontos_email {
                                if let Some(valor) =
                                    leak::filterMensagens::Subdomainfinders(text, 300).await
                                {
                                    let bot_send = bot
                                        .send_document(
                                            msg.chat.id,
                                            InputFile::file(valor.pathFile.clone()),
                                        )
                                        .reply_parameters(ReplyParameters::new(msg.id))
                                        .reply_markup(
                                            leak::callbacks::callbackbot::delete_messagem(),
                                        )
                                        .caption(format!(
                                            "Total Subdomain: {}\n\n{}",
                                            valor.total, valor.Domains
                                        ))
                                        .await;
                                    if let Err(e) = bot_send {
                                        println!("{:?}", e);
                                        bot.send_message(chat_id, "Sorry not found")
                                            .reply_parameters(ReplyParameters::new(msg.id))
                                            .await;
                                        return Ok(());
                                    }
                                    fs::remove_file(valor.pathFile).await;
                                    leak::users::atualizar_saldo_menos(chat_id.0, pontos_email);

                                    return Ok(());
                                }
                                bot.send_message(chat_id, "Sorry not found")
                                    .reply_parameters(ReplyParameters::new(msg.id))
                                    .await;
                                return Ok(());
                            } else {
                                bot.send_message(
                                    chat_id,
                                    format!(
                                        "{}\n\nValue {} Points",
                                        leak::runbot::sem_saldo(),
                                        pontos_email
                                    ),
                                )
                                .reply_parameters(ReplyParameters::new(msg.id))
                                .reply_markup(leak::callbacks::callbackbot::buy_money())
                                .await;
                                return Ok(());
                            }
                        }
                        _ => {
                            bot.send_message(
                                chat_id,
                                "Buy points to make queries to the bot, I am not currently operating in example mode\n\nCompre pontos para fazer consultas ao bot, não estou funcionando no modo de exemplo no momento\n\nКупите баллы, чтобы делать запросы к боту, в данный момент я не работаю в режиме примера\n\nBuy points: @forclogs\n\n/start",
                            )
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await;
                            return Ok(());
                        }
                    }
                }
            }
            match msg.clone().kind {
                MessageKind::Common(MensagemCommon) => {
                     if let Some(BoxMessage) =  MensagemCommon.reply_to_message{
                        let id_owner = BoxMessage.chat.id.0;
                        let id_msg = BoxMessage.id;
                        let from_chat = BoxMessage.chat.id;
                        let msg_ = msg.text().unwrap();
                        if msg_.contains("/send") && id_owner == runbot::id_owner(){
                            let todos_id = ver_todos_id();
                            let mut total_sucess = 0;
                            for id in todos_id{
                                if let Ok(_) = bot.forward_message(ChatId(id),from_chat ,id_msg).await{
                                    total_sucess +=1; 
                                    sleep(Duration::from_secs(2));
                                }
                            }
                            bot.send_message(msg.chat.id
                            ,
                                format!("Total people recev message: {} ", total_sucess),
                            )
                            .await;
                        }   
                     }
                }
                _ =>  {
                    return Ok(());
                }
            }
      
        }
    }

    Ok(())
}

async fn handle_callback(bot: Bot, query: CallbackQuery) -> Result<(), Error> {
    let maybe = query.clone().message.unwrap();
    let id_pessoa = query.from.id.0;

    let msg = match maybe.regular_message() {
        Some(msg) => msg,
        None => return Ok(()), // Se não houver mensagem, sai da função
    };

    let id_mensagem = msg.id;
    let chat_id = msg.chat.chat_id().unwrap();

    // verificar se a pessoa que está clicando é a verdadeira pessoa
    let id_query = query.from.id;

    let mut id_msg = None;
    match msg.reply_to_message() {
        Some(callL) => {
            let id_reply = callL.clone().from.unwrap().id;
            if id_query != id_reply {
                bot.answer_callback_query(query.id)
                    .text("⚠️ You do not have permission to delete this message. ⚠️ ")
                    .show_alert(true) // Mostra o alerta
                    .await;

                return Ok(());
            }
            id_msg = Some(callL.id);
        }

        _ => {}
    }

    if let Some(ref data) = query.data {
        match data.as_str() {
            "tools" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::tools())
                    .reply_markup(leak::callbacks::callbackbot::tools())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "start" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::lang_welcome())
                    .reply_markup(leak::callbacks::callbackbot::start_inicial_bot())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }

            "support" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::support())
                    .reply_markup(leak::callbacks::callbackbot::back_start())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "url" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::url())
                    .reply_markup(leak::callbacks::callbackbot::url())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }

            "mail" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::mail())
                    .reply_markup(leak::callbacks::callbackbot::url())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }

            "convite" => {
                bot.edit_message_text(
                    chat_id,
                    id_mensagem,
                    format!(
                        "{} https://t.me/OssintAndCheckBot?start={} ",
                        leak::runbot::convite(),
                        chat_id.0
                    ),
                )
                .reply_markup(leak::callbacks::callbackbot::convite())
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
            }

            "profile" => {
                if let Some(user) = leak::users::verificar_user_existe(id_pessoa as i64) {
                    let mut string_format = format!(
                        "Username: {}\nID: {}\nName: {}\nAccount Create: {}\nPoits: {}\n\n",
                        user.username, user.id, user.name, user.data_de_cadastro, user.saldo
                    );
                   let share_convite = format!(
                        "https://t.me/OssintAndCheckBot?start={} ",
                        
                        chat_id.0
                    );
                    string_format.push_str(&share_convite);
                    bot.edit_message_text(chat_id, id_mensagem, string_format)
                        .reply_markup(leak::callbacks::callbackbot::Profile())
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await;
                } else {
                    bot.edit_message_text(
                        chat_id,
                        id_mensagem,
                        "Click /start , user no exist ".to_string(),
                    )
                    .reply_markup(leak::callbacks::callbackbot::url())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                }
            }
            "delete" => {
                if let Some(valor) = id_msg {
                    bot.delete_message(chat_id, valor).await;
                }

                bot.delete_message(chat_id, msg.id).await;
                return Ok(());
            }

            "buy" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::buy_money())
                    .reply_markup(leak::callbacks::callbackbot::payment())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }

            "brasil" => {
                bot.edit_message_text(
                    chat_id,
                    id_mensagem,
                    format!("Colocando R$ 20 de saldo, você receberá 38 pontos. Ao adicionar R$ 50, serão 150 pontos, e com R$ 100, você acumulará 400 pontos. Não perca essa oportunidade de aumentar seu saldo e seus pontos!  @forclogs\n\n\nEntre em contato no privado e eu disponibilizarei a chave PIX para o pagamento."),
                )
                .reply_markup(leak::callbacks::callbackbot::back_buy())
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
            }

            "us" => {
                bot.edit_message_text(
                    chat_id,
                    id_mensagem,
                    format!("By adding $3.64 to your balance, you will receive 38 points. If you add $9.09, you will get 150 points, and with $18.18, you will accumulate 400 points. Don't miss this opportunity to boost your balance and points!   
Cryptocurrency Keys

USDT (TRC-20): <code>TN4n7b5j2b3zWGVBaCgZ93qog9WNYBwHyK</code>
Ethereum (ETH): <code>0x0a78d521B14f7B34217c2B9E03b41126414c606F</code>
Litecoin (LTC): <code>LfaPkUBfNT95EuLC6Qyv1X7HT5uaAwEinz</code>
Toncoin: <code>EQAGUGANsqtAbSFdc7VBgVmQt2lFTgfxl3aFxcntS2CYrR_8</code>

After making the payment, please send me the link or a photo of the payment.

https://shopbotossint.mysellix.io/products payment automatic 

@forclogs
"),
                )
                .reply_markup(leak::callbacks::callbackbot::back_buy())
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;
            }

            "database" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::msg_database_help())
                    .reply_markup(leak::callbacks::callbackbot::databases_adcionadas())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }

            "vk_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::vk_data_base())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "fraud_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::Eye4Fraud())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "piping_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::piping())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "cit0day" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::cit0day())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }

            "mypertamina_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::mypertamina())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "linkedIn_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::linkedIn())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "livpure_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::livpure())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }

            "hub_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::hub())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "habib_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::habib())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "zurich_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::zurich())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "zoomcar_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::zoomcar())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "zadig_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::zadig())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "younow_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::younow())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "xkcd_leak" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::xkcd())
                    .reply_markup(leak::callbacks::callbackbot::back_databases())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "mail2" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::email2())
                    .reply_markup(leak::callbacks::callbackbot::url())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "brute" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::brute())
                    .reply_markup(leak::callbacks::callbackbot::url())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }
            "shop" => {
                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::shop())
                    .reply_markup(leak::callbacks::callbackbot::shop())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }

            "cpanel" => {
                bot.answer_callback_query(query.id.clone())
                    .text("✅ Loading ... ✅")
                    .show_alert(true) // Mostra o alerta
                    .await;

                bot.edit_message_text(chat_id, id_mensagem, leak::runbot::cpanelText())
                    .reply_markup(leak::callbacks::callbackbot::CpanelView())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
            }

            "❌ Offline ❌" => {
                bot.edit_message_text(chat_id, id_mensagem, "Grupos desligado")
                    .reply_markup(leak::callbacks::callbackbot::admin_allow_permit_grupos(
                        "✅ Online ✅",
                    ))
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                runbot::oline_in_grups_config(false);
            }
            "✅ Online ✅" => {
                bot.edit_message_text(chat_id, id_mensagem, "Grupos ligados")
                    .reply_markup(leak::callbacks::callbackbot::admin_allow_permit_grupos(
                        "❌ Offline ❌",
                    ))
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await;
                runbot::oline_in_grups_config(true);
            }
            _ => {
                if data.contains("cpanel_") {
                    if let Some(string_venda) = shop_cp_anel(id_query.0 as i64, &data) {
                        bot.answer_callback_query(query.id.clone())
                            .text("✅ Purchase approved successfully.  ✅")
                            .show_alert(true) // Mostra o alerta
                            .await;

                        bot.edit_message_text(chat_id, id_mensagem, &string_venda)
                            .reply_markup(leak::callbacks::callbackbot::shop())
                            .parse_mode(teloxide::types::ParseMode::Html)
                            .await;
                        bot.send_message(chat_id, format!("✅ Provided by your purchase, below are the access credentials. Exchange limit in 5 minutes ✅\n\n{}\n\n", string_venda)).await;
                        bot.send_message(
                            ChatId(id_owner()),
                            format!("nova compra efetuada de um Cpanel \n\n{}\n\n", string_venda),
                        )
                        .await;
                    } else {
                        bot.answer_callback_query(query.id.clone())
                            .text("❌ Insufficient points or finished  ❌")
                            .show_alert(true) // Mostra o alerta
                            .await;
                    }
                }
            }
        }
    }

    Ok(())
}

fn logs(msg: &Message) {
    use colored::*;
    let from = msg.clone().from.unwrap();
    let username = from.username.unwrap_or("unknow".to_string());
    let fisrt_name = from.first_name;
    let id = from.id.0;
    let text = msg.text().unwrap_or("X-No_Message");

    let print = format!(
        "Username: {}\nFirstName: {}\nId: {}\nText: {}\n",
        username.bright_blue(),
        fisrt_name.bright_magenta(),
        id.to_string().bright_yellow(),
        text.bright_white()
    );

    println!("{}", print);
}

fn set_max_open_files(limit: u64) -> Result<(), std::io::Error> {
    use libc::{rlimit, setrlimit, RLIMIT_NOFILE};
    use std::io::{Error, ErrorKind};
    let rlim = rlimit {
        rlim_cur: limit,
        rlim_max: limit,
    };

    // Chamando setrlimit para ajustar o limite de arquivos abertos
    let result = unsafe { setrlimit(RLIMIT_NOFILE, &rlim) };

    if result != 0 {
        return Err(Error::new(ErrorKind::Other, "Failed to set file limit"));
    }
    Ok(())
}

fn shop_cp_anel(id_usuario: i64, cpanel: &str) -> Option<String> {
    // verificar se o saldo do usuario é igual ou maior do valor que ele quer comprar
    let pontos_cpanel = pontos_cpanel();
    let user = verificar_user_existe(id_usuario);
    if let Some(user) = user {
        if user.saldo >= pontos_cpanel {
            let prefix: Vec<&str> = cpanel.split("_").collect();

            let prefix = prefix[1];
            let document = Cpanel_collect_document(prefix);
            if let Some(documet) = document {
                let host = documet["host"].as_str().unwrap();
                let username = documet["username"].as_str().unwrap();
                let password = documet["password"].as_str().unwrap();

                let format = format!(
                    "\nHost: {}\nUsername: {}\nPassword: {}\n",
                    host, username, password
                );
                removeLoginCpanel(host, username);
                atualizar_saldo_menos(id_usuario, pontos_cpanel);
                return Some(format);
            }
        } else {
            return None;
        }
    }
    None
}
