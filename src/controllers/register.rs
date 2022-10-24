use std::error::Error;

use teloxide::{Bot, types::{Message, MediaKind, MessageKind}};

use crate::{models::user::{NewUser, User}, core::game_aggregator::GameAggregator};

use super::database::user_operations::insert_user;


pub fn register(
    _: &Bot,
    message: Message,
) -> String {
    let chat_id = message.chat.id;
    // check message type
    let mes = match &message.kind {
        MessageKind::Common(mes) => mes,
        _ => return "Not common message".into(),
    };
    // extract media (text)
    let media = match &mes.media_kind {
        MediaKind::Text(media) => media,
        _ => return "Not text media".into(),
    };
    // extract amount to be loaned to recievers
    let names = extract_names(&media.text);
    let new_users: Vec<NewUser> = names.iter().map(|n| NewUser::from(
        n.to_uppercase().to_string(), 
        chat_id.to_string())
    ).collect();

    let checked_new_users: Vec<NewUser> = new_users.into_iter()
        .map(|mut u| { GameAggregator::new().validate_user(&mut u); u })
        .collect();

    let mut valid_new_users = vec![];
    let mut invalid_new_users = vec![];

    for user in checked_new_users.into_iter() {
        if user.is_valid() {
            valid_new_users.push(user);
        } else {
            invalid_new_users.push(user);
        }
    }

    let users_to_insert: Vec<User> = valid_new_users.into_iter()
        .filter_map(|u| User::from(u).ok())
        .collect();

    let validate_messages: Vec<String> = invalid_new_users.into_iter()
        .map(|u| format!("Username {} is on a reserved list. Choose another name.", u.name))
        .collect();

    let insert_messages: Vec<String> = users_to_insert.into_iter()
        .map(|u| match insert_user(u){
            Ok(u) => format!("User {} created!", u.name),
            Err(e) => format!("Something ent wrong crating user: {}", e.to_string()),
        })
        .collect();
    let part_one = insert_messages.join("\n");
    let part_two = validate_messages.join("\n");
    format!("{}\n{}", part_one, part_two)

}


fn extract_names(text: &str) -> Vec<&str> {
    text.split_whitespace().skip(1).collect::<Vec<&str>>()
}