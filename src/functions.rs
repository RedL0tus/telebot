//! Available telegram functions, copied from https://core.telegram.org/bots/api#available-methods
//!
//! telebot-derive implements setter, setter and send methods to each struct

use bot::{Bot, RcBot};
use serde_json;
use objects;
use objects::Integer;
use file;
use failure::{Error, Fail};
use error::ErrorKind;
use futures::Future;
use std::rc::Rc;
use std::convert::{From, TryInto};
use erased_serde::Serialize;

/// The strongly typed version of the parse_mode field which indicates the type of text
pub enum ParseMode {
    Markdown,
    HTML,
    Text,
}

impl Into<String> for ParseMode {
    fn into(self) -> String {
        let tmp = match self {
            ParseMode::Markdown => "Markdown",
            ParseMode::HTML => "HTML",
            ParseMode::Text => "Text",
        };

        tmp.into()
    }
}

/// The strongly typed version of the action field which indicates the type of action
pub enum Action {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
}

/// Possible types of reply markups
pub enum ReplyMarkup {
    InlineKeyboardMarkup(objects::InlineKeyboardMarkup),
    ReplyKeyboardMarkup(objects::ReplyKeyboardMarkup),
    ReplyKeyboardRemove(objects::ReplyKeyboardRemove),
    ForceReply(objects::ForceReply),
}

impl ::serde::Serialize for ReplyMarkup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        use self::ReplyMarkup::*;

        match self {
            &InlineKeyboardMarkup(ref x) => x.serialize(serializer),
            &ReplyKeyboardMarkup(ref x) => x.serialize(serializer),
            &ReplyKeyboardRemove(ref x) => x.serialize(serializer),
            &ForceReply(ref x) => x.serialize(serializer),
        }
    }
}

impl From<objects::InlineKeyboardMarkup> for ReplyMarkup {
    fn from(f: objects::InlineKeyboardMarkup) -> Self {
        ReplyMarkup::InlineKeyboardMarkup(f)
    }
}

impl From<objects::ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(f: objects::ReplyKeyboardMarkup) -> Self {
        ReplyMarkup::ReplyKeyboardMarkup(f)
    }
}

impl From<objects::ReplyKeyboardRemove> for ReplyMarkup {
    fn from(f: objects::ReplyKeyboardRemove) -> Self {
        ReplyMarkup::ReplyKeyboardRemove(f)
    }
}

impl From<objects::ForceReply> for ReplyMarkup {
    fn from(f: objects::ForceReply) -> Self {
        ReplyMarkup::ForceReply(f)
    }
}

impl Into<String> for Action {
    fn into(self) -> String {
        let tmp = match self {
            Action::Typing => "Typing",
            Action::UploadPhoto => "UploadPhoto",
            Action::RecordVideo => "RecordVideo",
            Action::UploadVideo => "UploadVideo",
            Action::RecordAudio => "RecordVideo",
            Action::UploadAudio => "UploadAudio",
            Action::UploadDocument => "UploadDocument",
            Action::FindLocation => "FindLocation",
        };

        tmp.into()
    }
}

/// A simple method for testing your bot's auth token. Requires no parameters. Returns basic
/// information about the bot in form of a User object.
#[derive(TelegramFunction, Serialize)]
#[call = "getMe"]
#[answer = "User"]
#[function = "get_me"]
pub struct GetMe;

#[derive(TelegramFunction, Serialize)]
#[call = "getUpdates"]
#[answer = "Updates"]
#[function = "get_updates"]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<Vec<String>>,
}

/// Use this method to send text messages. On success, the sent Message is returned.
#[derive(TelegramFunction, Serialize)]
#[call = "sendMessage"]
#[answer = "Message"]
#[function = "message"]
pub struct Message {
    chat_id: Integer,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notificaton: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send photos. On success, the sent Message is returned.
#[derive(TelegramFunction, Serialize)]
#[call = "sendPhoto"]
#[answer = "Message"]
#[function = "photo"]
#[file_kind = "photo"]
pub struct SendPhoto {
    chat_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send audio files, if you want Telegram clients to display them in the music
/// player. Your audio must be in the .mp3 format. On success, the sent Message is returned. Bots
/// can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
///
/// For sending voice messages, use the sendVoice method instead.
#[derive(TelegramFunction, Serialize)]
#[call = "sendAudio"]
#[answer = "Message"]
#[function = "audio"]
#[file_kind = "audio"]
pub struct SendAudio {
    chat_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send general files. On success, the sent Message is returned. Bots can
/// currently send files of any type of up to 50 MB in size, this limit may be changed in the
/// future.
#[derive(TelegramFunction, Serialize)]
#[call = "sendDocument"]
#[answer = "Message"]
#[function = "document"]
#[file_kind = "document"]
pub struct SendDocument {
    chat_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    document: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send .webp stickers. On success, the sent Message is returned.
#[derive(TelegramFunction, Serialize)]
#[call = "sendSticker"]
#[answer = "Message"]
#[function = "sticker"]
#[file_kind = "sticker"]
pub struct SendSticker {
    chat_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send video files, Telegram clients support mp4 videos (other formats may be
/// sent as Document). On success, the sent Message is returned. Bots can currently send video
/// files of up to 50 MB in size, this limit may be changed in the future.
#[derive(TelegramFunction, Serialize)]
#[call = "sendVideo"]
#[answer = "Message"]
#[function = "video"]
#[file_kind = "video"]
pub struct SendVideo {
    chat_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send audio files, if you want Telegram clients to display the file as a
/// playable voice message. For this to work, your audio must be in an .ogg file encoded with OPUS
/// (other formats may be sent as Audio or Document). On success, the sent Message is returned.
/// Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the
/// future.
#[derive(TelegramFunction, Serialize)]
#[call = "sendVoice"]
#[answer = "Message"]
#[function = "voice"]
#[file_kind = "voice"]
pub struct SendVoice {
    chat_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send point on the map. On success, the sent Message is returned.
#[derive(TelegramFunction, Serialize)]
#[call = "sendLocation"]
#[answer = "Message"]
#[function = "location"]
pub struct SendLocation {
    chat_id: Integer,
    latitude: f32,
    longitude: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send information about a venue. On success, the sent Message is returned.
#[derive(TelegramFunction, Serialize)]
#[call = "sendVenue"]
#[answer = "Message"]
#[function = "venue"]
pub struct SendVenue {
    chat_id: Integer,
    latitude: f32,
    longitude: f32,
    title: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method to send phone contacts. On success, the sent Message is returned.
#[derive(TelegramFunction, Serialize)]
#[call = "sendContact"]
#[answer = "Message"]
#[function = "contact"]
pub struct SendContact {
    chat_id: Integer,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Use this method when you need to tell the user that something is happening on the bot's side.
/// The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients
/// clear its typing status). Returns True on success.
#[derive(TelegramFunction, Serialize)]
#[call = "sendChatAction"]
#[answer = "Boolean"]
#[function = "chat_action"]
pub struct SendAction {
    chat_id: Integer,
    action: String,
}

#[derive(TelegramFunction, Serialize)]
#[call = "sendGame"]
#[answer = "Message"]
#[function = "send_game"]
pub struct SendGame {
    chat_id: Integer,
    game_short_name: String,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<objects::InlineKeyboardMarkup>,
}

#[derive(TelegramFunction, Serialize)]
#[call = "setGameScore"]
#[answer = "Message"]
#[function = "set_game_score"]
pub struct SetGameScore {
    user_id: Integer,
    score: Integer,
    force: Option<bool>,
    disable_edit_message: Option<bool>,
    chat_id: Option<Integer>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
}

#[derive(TelegramFunction, Serialize)]
#[call = "getGameHighScores"]
#[answer = "GameHighScore"]
#[function = "get_game_high_scores"]
pub struct GetGameHighScores {
    user_id: Integer,
    chat_id: Option<Integer>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
}

/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos
/// object.
#[derive(TelegramFunction, Serialize)]
#[call = "getUserProfilePhotos"]
#[answer = "UserProfilePhotos"]
#[function = "get_user_profile_photos"]
pub struct GetUserProfilePhotos {
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
}

/// Use this method to get basic info about a file and prepare it for downloading. For the moment,
/// bots can download files of up to 20MB in size. On success, a File object is returned. The file
/// can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where
/// <file_path> is taken from the response. It is guaranteed that the link will be valid for at
/// least 1 hour. When the link expires, a new one can be requested by calling getFile again.
#[derive(TelegramFunction, Serialize)]
#[call = "getFile"]
#[answer = "File"]
#[function = "get_file"]
pub struct GetFile {
    file_id: String,
}

/// Use this method to kick a user from a group or a supergroup. In the case of supergroups, the
/// user will not be able to return to the group on their own using invite links, etc., unless
/// unbanned first. The bot must be an administrator in the group for this to work. Returns True on
/// success.
#[derive(TelegramFunction, Serialize)]
#[call = "kickChatMember"]
#[answer = "Boolean"]
#[function = "kick_chat_member"]
pub struct KickChatMember {
    chat_id: Integer,
    user_id: Integer,
}

/// Use this method for your bot to leave a group, supergroup or channel. Returns True on
/// success.
#[derive(TelegramFunction, Serialize)]
#[call = "leaveChat"]
#[answer = "Boolean"]
#[function = "leave_chat"]
pub struct LeaveChat {
    chat_id: Integer,
}

/// Use this method to unban a previously kicked user in a supergroup. The user will not return to
/// the group automatically, but will be able to join via link, etc. The bot must be an
/// administrator in the group for this to work. Returns True on success.
#[derive(TelegramFunction, Serialize)]
#[call = "unbanChatMember"]
#[answer = "Boolean"]
#[function = "unban_chat_member"]
pub struct UnbanChatMember {
    chat_id: Integer,
    user_id: Integer,
}

/// Use this method to get up to date information about the chat (current name of the user for
/// one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat
/// object on success.
#[derive(TelegramFunction, Serialize)]
#[call = "getChat"]
#[answer = "Chat"]
#[function = "get_chat"]
pub struct GetChat {
    chat_id: Integer,
}

/// Use this method to get a list of administrators in a chat. On success, returns an Array of
/// ChatMember objects that contains information about all chat administrators except other bots.
/// If the chat is a group or a supergroup and no administrators were appointed, only the creator
/// will be returned.
#[derive(TelegramFunction, Serialize)]
#[call = "getChatAdministrators"]
#[answer = "Vector<objects::ChatMember>"]
#[function = "unban_chat_administrators"]
pub struct GetChatAdministrators {
    chat_id: Integer,
}

/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(TelegramFunction, Serialize)]
#[call = "getChatMembersCount"]
#[answer = "Integer"]
#[function = "get_chat_members_count"]
pub struct GetChatMemberCounts {
    chat_id: Integer,
}

/// Use this method to get information about a member of a chat. Returns a ChatMember object on
/// success.
#[derive(TelegramFunction, Serialize)]
#[call = "getChatMember"]
#[answer = "ChatMember"]
#[function = "get_chat_member"]
pub struct GetChatMember {
    chat_id: Integer,
    user_id: Integer,
}

/// Use this method to send answers to callback queries sent from inline keyboards. The answer will
/// be displayed to the user as a notification at the top of the chat screen or as an alert. On
/// success, True is returned.
#[derive(TelegramFunction, Serialize)]
#[call = "answerCallbackQuery"]
#[answer = "Boolean"]
#[function = "answer_callback_query"]
pub struct AnswerCallbackQuery {
    callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
}

/// Use this method to send answers to an inline query. On success, True is returned.
/// No more than 50 results per query are allowed.
#[derive(TelegramFunction, Serialize)]
#[call = "answerInlineQuery"]
#[answer = "Boolean"]
#[function = "answer_inline_query"]
pub struct AnswerInlineQuery {
    inline_query_id: String,
    results: Vec<Box<Serialize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_parameter: Option<String>,
}

///Use this method to create new sticker set owned by a user.
///The bot will be able to edit the created sticker set. Returns True on success.
#[derive(TelegramFunction, Serialize)]
#[call = "createNewStickerSet"]
#[answer = "Boolean"]
#[function = "create_new_sticker_set"]
#[file_kind = "png_sticker"]
pub struct CreateNewStickerSet {
    user_id: Integer,
    name: String,
    title: String,
    emojis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    png_sticker: Option<String>,
}

///Use this method to add a new sticker to a set created by the bot. Returns True on success.
#[derive(TelegramFunction, Serialize)]
#[call = "addStickerToSet"]
#[answer = "Boolean"]
#[function = "add_sticker_to_set"]
#[file_kind = "png_sticker"]
pub struct AddStickerToSet {
    user_id: Integer,
    name: String,
    emojis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    png_sticker: Option<String>,
}
