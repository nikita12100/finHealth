use teloxide::macros::BotCommands;

#[derive(Clone, Debug, BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "В начало")]
    Start,
    #[command(description = "Показать все команды")]
    Help,
}
