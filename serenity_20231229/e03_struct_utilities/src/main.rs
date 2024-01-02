use std::env;

use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!messageme" {
            // `utils`フィーチャが有効な場合、モデル構造体にはContextを使わずに便利なメソッドが多数実装されています。
            // これにより、Contextや低レベルの`rest`メソッドを頻繁に使う必要がありません。
            //
            // この場合、ユーザーにダイレクトメッセージを送信するには、単にそのインスタンスに対してメソッドを呼び出し、
            // メッセージの内容を指定します。
            let builder = CreateMessage::new().content("こんにちは！");
            let dm = msg.author.dm(&context, builder).await;

            if let Err(why) = dm {
                println!("ユーザーにダイレクトメッセージを送信中にエラーが発生しました: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} が接続しました！", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Discordボットトークンを環境変数から取得してクライアントを構成します。
    let token = env::var("DISCORD_TOKEN").expect("環境変数にトークンが指定されていません");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("クライアントの作成中にエラーが発生しました");

    if let Err(why) = client.start().await {
        println!("クライアントエラー: {why:?}");
    }
}
