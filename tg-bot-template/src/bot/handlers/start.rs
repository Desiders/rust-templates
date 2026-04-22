use telers::{
    Bot,
    event::{EventReturn, telegram::HandlerResult},
    methods::SendMessage,
    types::{Message, ReplyParameters},
};
use tracing::instrument;

#[instrument(skip_all)]
pub async fn start(bot: Bot, message: Message) -> HandlerResult {
    bot.send(
        SendMessage::new(
            message.chat().id(),
            "Hello. This template is ready for Telegram bot use-cases.",
        )
        .reply_parameters(
            ReplyParameters::new(message.message_id()).allow_sending_without_reply(true),
        ),
    )
    .await?;

    Ok(EventReturn::Finish)
}
