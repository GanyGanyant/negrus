use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    Result as SerenityResult,
};

mod video;

mod join;
pub use join::*;
mod play;
pub use play::*;
mod deafen;
pub use deafen::*;
mod leave;
pub use leave::*;
mod mute;
pub use mute::*;

#[command]
async fn ping(context: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&context.http, "Pong!").await);

    Ok(())
}



/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
