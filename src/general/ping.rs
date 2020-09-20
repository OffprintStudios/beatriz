use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command
    }
};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}