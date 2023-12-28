// `Tweet` トレイトを定義します
trait Tweet {
    // ツイートするためのメソッド
    fn tweet(&self);

    // ツイートを二度行うためのデフォルト実装
    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    // 大声で叫ぶためのデフォルト実装
    fn shout(&self) {
        println!("Uooooohhh!!!!");
    }
}

// 鳩（Dove）構造体の定義
struct Dove;

// `Dove` 構造体に `Tweet` トレイトを実装します
impl Tweet for Dove {
    fn tweet(&self) {
        // ドーブ（鳩）のツイートの具体的な実装
        println!("くるっくー");
    }
}

// アヒル（Duck）構造体の定義
struct Duck;

// `Duck` 構造体に `Tweet` トレイトを実装します
impl Tweet for Duck {
    fn tweet(&self) {
        // ダック（アヒル）のツイートの具体的な実装
        println!("ガーガー")
    }
}

// メイン関数
fn main() {
    // 鳩 のインスタンスを作成
    let dove = Dove {};

    // 鳩のツイートを表示
    dove.tweet();

    // 鳩のツイートを二度行う
    dove.tweet_twice();

    // 鳩が大声で叫ぶ
    dove.shout();
}
