# ⚠️AWS環境での運用のためリポジトリを変更しました。⚠️
## ↓↓↓対象リポジトリ↓↓↓

https://github.com/Ometeor-Zheero-OMZ/Ataria

理由：
バックエンドをRustでWebフレームワークのActix Webを使用して開発を進めていましたが、Actix Webのソースコードのまま AWS Lambdaでの処理方法が異なるため、
大幅にソースコードの改修を行う必要がありました。AWS Lambdaのサーバーレスのアプリケーションを作成するにあたって、別のWebフレームワークである Axum との相性が良いことが
わかったため、Axum の学習および大幅な改修を行いました。

変更点：
- クリーンアーキテクチャ (DDD)の採用
- lambda_http クレートを使用した HTTP APIの実装
- DIの廃止（低レイヤーで問題が発生したため）

### 機能一覧

- ゲームストア　インディーズゲームの配布または販売をサポート（実装予定）
- コミュニティ
- スレッド

## 技術スタック：

言語・フレームワーク
- Rust (Actix Web)
- TypeScript (Next.js)

DB
- PostgreSQL

インフラ
- Nginx
- Docker
- AWS (現在環境設定中)

## テーブル設計
エンティティやカラム等追加がある場合は随時更新しています。

![Ataria drawio](https://github.com/user-attachments/assets/5053d5e5-318d-48b3-8c79-a48e2bab7c1c)
