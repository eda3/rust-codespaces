use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

// イベントハンドラを定義する構造体
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // `message` イベント用のハンドラを設定します。新しいメッセージが受信されるたびに、指定されたクロージャ（または関数）が呼び出されます。
    // イベントハンドラはスレッドプールを介してディスパッチされ、複数のイベントが同時にディスパッチされる可能性があります。
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // メッセージの送信は、ネットワークエラー、認証エラー、またはチャンネルに投稿する権限がないため、失敗する可能性があります。エラーが発生した場合は、その詳細をstdoutにログ出力します。
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("メッセージの送信エラー: {why:?}");
            }
        }
    }

    // `ready` イベントに呼び出されるハンドラを設定します。
    // これはシャードが起動され、DiscordからREADYペイロードが送信されたときに呼び出されます。
    // このペイロードには、現在のユーザーのギルドID、現在のユーザーデータ、プライベートチャンネルなどが含まれています。
    // この場合、現在のユーザーのユーザー名を印刷します。
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} が接続しました！", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Discordボットトークンを環境変数から取得してクライアントを構成します。
    let token = env::var("DISCORD_TOKEN").expect("環境変数にトークンが必要です");
    // ゲートウェイインテントを設定し、ボットが通知を受けるイベントを決定します
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // 新しいクライアントのインスタンスを作成し、ボットとしてログインします。
    // これにより、ボットトークンは自動的に「Bot 」で始まるようになり、これはDiscordによるボットユーザーの要件です。
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("クライアントの作成に失敗しました");

    // 最後に、単一のシャードを開始し、イベントのリスニングを開始します。
    // シャードは自動的に再接続を試み、再接続するまで指数関数的なバックオフを行います。
    if let Err(why) = client.start().await {
        println!("クライアントエラー: {why:?}");
    }
}
