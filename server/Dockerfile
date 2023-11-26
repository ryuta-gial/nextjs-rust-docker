# 開発環境の設定
FROM rust

# ステージ1: ビルド環境の設定
FROM rust as builder
WORKDIR /usr/src/myapp

# ビルド時の環境変数の定義
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Diesel CLIのインストール
RUN cargo install diesel_cli --no-default-features --features postgres

# 依存関係のコピーとビルド
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/myapp*
COPY . .
RUN cargo build --release

# ステージ2: 実行環境の設定
FROM rust
RUN apt-get update && apt-get install -y libssl-dev pkg-config libpq-dev

# ビルドステージからのファイルのコピー
COPY --from=builder /usr/src/myapp/target/release/myapp /usr/local/bin/myapp

# 実行コマンド
CMD ["myapp"]
