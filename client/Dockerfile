# ステージ1: ビルド環境の設定
FROM node:18.17.0 AS builder
WORKDIR /app

# プロジェクトの package.json と package-lock.json をコピー
COPY package*.json ./

# 依存関係のインストール
RUN npm ci

# クライアントアプリケーションのソースコードをコピー
COPY . .

# 開発サーバーを起動
CMD ["npm", "run", "dev"]

# # ビルド
# RUN npm run build
# 
# # ステージ2: 実行環境の設定
# FROM node:18.17.0
# WORKDIR /app
# 
# # ビルドステージから成果物をコピー
# COPY --from=builder /app/.next ./.next
# COPY --from=builder /app/public ./public
# COPY --from=builder /app/package*.json ./
# 
# # 本番環境の依存関係のみをインストール
# RUN npm ci --only=production
# 
# # ここでアプリケーションを実行するコマンドを指定する
# CMD ["npm", "start"]
