use serenity::all::{ChannelId, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;

use serenity::prelude::*;
use dotenv::dotenv;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write};
use std::sync::atomic::AtomicBool;
use tokio::net::UdpSocket;

static mut CHANNEL_ID : Option<ChannelId> = None;

struct Handler{
    is_ready : AtomicBool
}
#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, ctx: Context, msg : Message){
        if msg.content == ">set_channel"{
            
            unsafe {
                let mut f = BufWriter::new(OpenOptions::new().read(true).write(true).create(true).open("/data/chan").unwrap());
                CHANNEL_ID = Some(msg.channel_id.clone());
                write!(f, "{}", msg.channel_id).unwrap();
                CHANNEL_ID.unwrap().say(&ctx.http, "Set channel!").await.unwrap();
            }
        }
    }
    async fn ready(&self, ctx : Context, _ : Ready){
       self.is_ready.load(std::sync::atomic::Ordering::Relaxed);
       tokio::spawn(async move{
        hook_listener(ctx).await
       });
    }
}

async fn hook_listener(ctx : Context){
    let port = env::var("SERVER_PORT").expect("Set $SERVER_PORT please");
    let socket = UdpSocket::bind(format!("0.0.0.0:{port}")).await.unwrap();
    let mut buf : [u8; 10] = [0; 10];

    loop{
        let _ = socket.recv_from(&mut buf).await.unwrap();
        println!("hook triggered");
        unsafe {
            if let Some(c) = CHANNEL_ID{
                c.say(&ctx, "Hook Triggered").await.unwrap();
            }
        }
    }

}

#[tokio::main]
async fn main() {
    dotenv().ok();
    unsafe {
        let mut f = OpenOptions::new().read(true).write(true).create(true).open("/data/chan").unwrap();
        let mut channel : String = String::new();
        f.read_to_string(&mut channel).unwrap();
        
        CHANNEL_ID = Some(channel.lines().next().unwrap().parse().unwrap());
    }

    let mut client = 
    Client::builder(env::var("DISCORD_TOKEN").expect("Woah woah woah, you need a token at $DISCORD_TOKEN"), GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT)
    .event_handler(Handler{is_ready : AtomicBool::new(true)}).await.expect("Error creating client");

    if let Err(why) = client.start().await{
        println!("Woops : {why:?}");
    }
    
}
