use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

// Serenityは透過的なシャーディングを実装しており、個別にプロセスや接続を手動で処理する必要はありません。
//
// 透過的なシャーディングは共有キャッシュに役立ちます。重複したデータを持つ代わりに、共有キャッシュはすべてのデータに簡単にアクセスできるようにします。
//
// ボットが多くのギルドにいる場合、または最大の2500を超える場合は、ギルドシャーディングを使用する必要があります。
//
// これはギルドシャーディングがどのように動作するかを示す例のファイルです。これが効果的に見えるようにするには、ボットは少なくとも2つのギルドにいる必要があります。
//
// 2つのギルドのシナリオを考えてみましょう。片方のギルドで「!ping」と言ってみてください。コンソールには「0」または「1」が表示されるはずです。もう片方のギルドで「!ping」と言ってみてください。これにより、コンソールには他の数字がキャッシュされます。これにより、ギルドシャーディングが動作していることが確認できます。
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            println!("シャード {}", ctx.shard_id);

            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("メッセージの送信エラー: {why:?}");
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
    let token = env::var("DISCORD_TOKEN").expect("環境変数にトークンが必要です");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("クライアントの作成に失敗しました");

    // 使用するシャードの総数。シャードの「現在のシャード番号」、つまり割り当てられたシャードのインデックスは0から始まり、
    // シャードの合計数のインデックスは1から始まります。
    //
    // これは、例えば5つのシャードがある場合、シャードの合計数は5になり、各シャードは0から4までの番号が割り当てられます。
    if let Err(why) = client.start_shards(2).await {
        println!("クライアントエラー: {why:?}");
    }
}
