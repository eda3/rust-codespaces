## update and install some things we should probably have
apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  vim \
  build-essential \
  openssl

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y 
rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy 
rustup component add clippy --toolchain nightly

cargo install cargo-expand
cargo install cargo-edit

## setup and install oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
cp -R /root/.oh-my-zsh /home/$USERNAME
cp /root/.zshrc /home/$USERNAME
sed -i -e "s/\/root\/.oh-my-zsh/\/home\/$USERNAME\/.oh-my-zsh/g" /home/$USERNAME/.zshrc
chown -R $USER_UID:$USER_GID /home/$USERNAME/.oh-my-zsh /home/$USERNAME/.zshrc

git clone https://github.com/eda3/dotfiles .

# dotfiles ディレクトリの絶対パスを取得
DOTFILES_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# インストール対象の設定やツールのリスト
FILES_TO_INSTALL=(
    .vimrc
    .bashrc
    # 追加のファイルやディレクトリをここに追加
)

# ドットファイルのシンボリックリンクを作成
create_symlinks() {
    for file in "${FILES_TO_INSTALL[@]}"; do
        source_file="$DOTFILES_DIR/$file"
        target_file="$HOME/$file"

        # 既存のファイルがある場合はバックアップを作成
        if [ -e "$target_file" ]; then
            mv "$target_file" "$target_file.bak"
            echo "Backup created: $target_file.bak"
        fi

        # シンボリックリンクを作成
        ln -s "$source_file" "$target_file"
        echo "Symlink created: $target_file -> $source_file"
    done
}

# インストール対象の設定やツールのインストール処理を追加
install_dependencies() {
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    sudo snap install docker

    # gitの初期処理
    git config --global user.email "gerorin1010@gmail.com"
    git config --global user.name "eda3"

}

# 実際のインストール処理を実行
create_symlinks
install_dependencies


echo "Installation completed."