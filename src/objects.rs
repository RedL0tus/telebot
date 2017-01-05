//! The complete list of telegram types, copied from:
//! https://core.telegram.org/bots/api#available-types
//!
//! on each struct getter, setter and send function will be implemented

/// These objects are redefinitions of basic types. telebot-derive will scope every object in
/// answer, so we need to redefine them here.
pub type Boolean = bool;
pub type Integer = i64;
pub type Vector<T> = Vec<T>;
pub type NotImplemented = ();

/// This object represents a Telegram user or bot.
#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: Integer,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>
}

/// This object represents a chat.
#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: Integer,
    #[serde(rename="type")]
    pub kind: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>
}

/// This object represents one special entity in a text message. For example, hashtags, usernames,
/// URLs, etc. 
#[derive(Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename="type")]
    pub kind: String,
    pub offset: Integer,
    pub length: Integer,
    pub url: Option<String>,
    pub user: Option<User>
}

/// This object represents a message.
#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: Integer,
    pub from: Option<User>,
    pub date: Integer,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<User>,
    pub forward_from_message_id: Option<Integer>,
    pub forward_date: Option<Integer>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<Integer>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub game: Option<NotImplemented>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub new_chat_member: Option<User>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<Integer>,
    pub migrate_from_chat_id: Option<Integer>,
    pub pinned_message: Option<Box<Message>>
}

#[derive(Deserialize, Debug)]
pub struct Updates(pub Vec<Update>);

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: Integer,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub inline_query: Option<()>,
    pub chosen_inline_result: Option<()>,
    pub callback_query: Option<()>
}

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Deserialize, Debug, Clone)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub file_size: Option<Integer>
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Deserialize, Debug)]
pub struct Audio {
    pub file_id: String,
    pub duration: Integer,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Deserialize, Debug)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>
}

/// This object represents a sticker.
#[derive(Deserialize, Debug)]
pub struct Sticker {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub file_size: Option<Integer>
}

/// This object represents a video file.
#[derive(Deserialize, Debug)]
pub struct Video {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub duration: Integer,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<String>
}

/// This object represents a voice note.
#[derive(Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub duration: Integer,
    pub mime_type: Option<String>,
    pub file_size: Option<String>
}

/// This object represents a phone contact.
#[derive(Deserialize, Debug)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: String,
    pub user_id: Integer
}

/// This object represents a point on the map.
#[derive(Deserialize, Debug)]
pub struct Location {
    pub longitude: f32,
    pub latitude: f32
}

/// This object represents a venue.
#[derive(Deserialize, Debug)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>
}

/// This object represent a user's profile pictures.
#[derive(Deserialize, Debug)]
pub struct UserProfilePhotos {
    pub total_count: Integer,
    pub photos: Vec<Vec<PhotoSize>>
}

/// This object represents a file ready to be downloaded. The file can be downloaded via the link
/// https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be
/// valid for at least 1 hour. When the link expires, a new one can be requested by calling
/// getFile.
#[derive(Deserialize, Debug)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<Integer>,
    pub file_path: Option<String>
}

/// This object represents a custom keyboard with reply options (see Introduction to bots for
/// details and examples).
#[derive(Deserialize, Debug)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<KeyboardButton>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub selective: Option<bool>
}

/// This object represents one button of the reply keyboard. For simple text buttons String can be
/// used instead of this object to specify text of the button. Optional fields are mutually
/// exclusive.
#[derive(Deserialize, Debug)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>
}

/// Upon receiving a message with this object, Telegram clients will remove the current custom
/// keyboard and display the default letter-keyboard. By default, custom keyboards are displayed
/// until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are
/// hidden immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Deserialize, Debug)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    pub selective: Option<bool>
}

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<InlineKeyboardButton>
}

/// This object represents one button of an inline keyboard. You must use exactly one of the
/// optional fields.
#[derive(Deserialize, Debug)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub callback_game: Option<CallbackGame>
}

/// This object represents an incoming callback query from a callback button in an inline keyboard.
/// If the button that originated the query was attached to a message sent by the bot, the field
/// message will be present. If the button was attached to a message sent via the bot (in inline
/// mode), the field inline_message_id will be present. Exactly one of the fields data or
/// game_short_name will be present.
#[derive(Deserialize, Debug)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: Option<String>,
    pub data: Option<String>,
    pub game_short_name: Option<String>
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface to
/// the user (act as if the user has selected the bot‘s message and tapped ’Reply'). This can be
/// extremely useful if you want to create user-friendly step-by-step interfaces without having to
/// sacrifice privacy mode.
#[derive(Deserialize, Debug)]
pub struct ForceReply {
    pub force_reply: bool,
    pub selective: Option<bool>
}
    
/// This object contains information about one member of the chat.
#[derive(Deserialize, Debug)]
pub struct ChatMember {
    pub user: User,
    pub status: String
}

/// Contains information about why a request was unsuccessfull.
#[derive(Deserialize, Debug)]
pub struct ResponseParameter {
    pub migrate_to_chat_id: Option<Integer>,
    pub retry_after: Option<Integer>
}

/// A placeholder, currently holds no information. Use BotFather to set up your game.
#[derive(Deserialize, Debug)]
pub struct CallbackGame;