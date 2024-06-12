use std::any::Any;

use serenity::{
    all::{
        ButtonStyle, CreateActionRow, CreateButton, CreateInputText, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateMessage, CreateModal, CreateSelectMenu,
        CreateSelectMenuKind, CreateSelectMenuOption, EditInteractionResponse, EditMessage,
        InputText, InputTextStyle, Message, ReactionType,
    },
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    futures::StreamExt,
};

#[command]
#[only_in(guilds)]
#[aliases("d", "dummy", "longDummyCommand")]
async fn dummy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let component = msg
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content("Hola Dummy Menu")
                .button(
                    CreateButton::new("test")
                        .label("Test")
                        .custom_id("dummy_button")
                        .style(ButtonStyle::Secondary),
                )
                .button(
                    CreateButton::new("test2")
                        .label("Test2")
                        .custom_id("dummy_button2")
                        .style(ButtonStyle::Secondary),
                ),
        )
        .await
        .unwrap();

    let mut interaction_stream = component.await_component_interaction(&ctx).stream();

    while let Some(interaction) = interaction_stream.next().await {
        let id = &interaction.data.custom_id;
        
        interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::default()
                        .content(format!("Button Pressed **{id}**"))
                        .button(
                            CreateButton::new("testF")
                                .label("Test F")
                                .custom_id("dummy_button_f")
                                .style(ButtonStyle::Danger),
                        )
                        .button(
                            CreateButton::new("test2F")
                                .label("Test2 F")
                                .custom_id("dummy_button2_f")
                                .style(ButtonStyle::Primary),
                        ),
                ),
            )
            .await?;
    }

    println!("Dummy command executed"); 

    Ok(())
}
