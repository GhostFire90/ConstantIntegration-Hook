use serenity::all::{ChannelId, CreateAttachment, CreateEmbed, CreateMessage, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;

use serenity::prelude::*;
use dotenv::dotenv;
use std::env;
use std::fs::{remove_file, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::process::Command;
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
        println!("Ready!");
       self.is_ready.load(std::sync::atomic::Ordering::Relaxed);
       tokio::spawn(async move{
        hook_listener(ctx).await
       });
    }
}

async fn hook_listener(ctx : Context){
    let port = env::var("SERVER_PORT").expect("Set $SERVER_PORT please");
    let socket = UdpSocket::bind(format!("0.0.0.0:{port}")).await.unwrap();
    let mut buf : [u8; 1500] = [0; 1500];

    loop{
        let (size, _) = socket.recv_from(&mut buf).await.unwrap();
        println!("hook triggered");
        unsafe {
            if let Some(c) = CHANNEL_ID{
                let mut message_vec = buf.to_vec();
                message_vec.resize(size, 0);
                let msg_str = String::from_utf8_unchecked(message_vec);
                let mut ws_itr = msg_str.split_whitespace();
                let embed = CreateEmbed::new()
                    .title("Push Recieved")
                    .field("Old-SHA", ws_itr.next().unwrap(), false)
                    .field("New-SHA", ws_itr.next().unwrap(), false)
                    .field("Refspec", ws_itr.next().unwrap(), false);
                let builder = CreateMessage::new().embed(embed);
                c.send_message(&ctx, builder).await.unwrap();

                let test_out = run_tests();
                let mut f = OpenOptions::new().create(true).write(true).open("./attachment.txt").unwrap();
                let _ = f.write_all(test_out.as_bytes());
                drop(f);

                let attachment = CreateAttachment::path("./attachment.txt").await.unwrap();
                let builder = CreateMessage::new().add_file(attachment);
                c.send_message(&ctx, builder).await.unwrap();
                let _ = remove_file("./attachment.txt");
                
            }
        }
    }

}


fn run_tests() -> String{

    let poetry_run_output = Command::new("bash").arg("./event-script").output().unwrap();



    unsafe{String::from_utf8_unchecked(poetry_run_output.stdout)}
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    unsafe {
        let mut f = OpenOptions::new().read(true).write(true).create(true).open("/data/chan").unwrap();
        let mut channel : String = String::new();
        f.read_to_string(&mut channel).unwrap();
        
        CHANNEL_ID = match channel.lines().next().unwrap().parse(){
            Ok(val) => Some(val),
            Err(_) => None
        };
    }

    let mut client = 
    Client::builder(env::var("DISCORD_TOKEN").expect("Woah woah woah, you need a token at $DISCORD_TOKEN"), GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT)
    .event_handler(Handler{is_ready : AtomicBool::new(true)}).await.expect("Error creating client");

    if let Err(why) = client.start().await{
        println!("Woops : {why:?}");
    }
    
}
