use poise::{PrefixFrameworkOptions, serenity_prelude as serenity};

type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Display your or another user's account creation date
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or(ctx.author());
    ctx.say(format!("{}'s account was created at {}", user.name, user.created_at())).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() {
    poise::Framework::build()
        .token(std::env::var("DISCORD_BOT_TOKEN").unwrap())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(()) }))
        .options(poise::FrameworkOptions {
            // configure framework here
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .command(age(), |f| f)
        .run().await.unwrap();
}