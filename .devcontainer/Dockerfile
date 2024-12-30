FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
  git \
  curl \
  unzip \
  gcc \
  xz-utils
  # sudo \
  
# Node.jsのインストール
RUN curl -fsSL https://deb.nodesource.com/setup_23.x | bash - \
&& apt-get install -y nodejs

# Pythonのインストール
RUN apt-get install -y python3

# aptのキャッシュ削除
RUN apt-get clean \
&& rm -rf /var/lib/apt/lists/*

# non-root-userの追加
ARG USER_NAME=user
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USER_NAME \
  && useradd --gid $USER_GID --uid $USER_UID -m $USER_NAME -s /bin/bash
  # && echo $USER_NAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USER_NAME \
  # && chmod 0440 /etc/sudoers.d/$USER_NAME

# non-root-userに切り替え
USER $USER_NAME

# Rustのインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
# wasmランタイムのインストール
RUN curl https://wasmtime.dev/install.sh -sSf | bash
# wasm開発用ツールのインストール
# MEMO: wasm-packは実行可能形式でインストールしたいため、ここで明示的にインストールする
ENV PATH="/home/$USER_NAME/.cargo/bin:$PATH"
RUN cargo install wasm-pack
