# ⚠️AWS環境での運用のためリポジトリを変更しました
## ↓↓↓対象リポジトリ↓↓↓

https://github.com/Ometeor-Zheero-OMZ/Ataria

理由：
バックエンドをRustでWebフレームワークのActix Webを使用して開発を進めていましたが、現段階でActix Web と AWS Lambdaとの連携は難しく、別のウェブフレームワークである Axum では AWS Lambdaとの連携は可能であったため、
大幅にソースコードの改修を行いました。また、今回でソース改修の範囲が広かったことを反省して、クリーンアーキテクチャ (DDD)で実装しなおしました。

変更点：
- Axum の採用
- lambda_http クレートを使用した HTTP APIの実装
- Actix Webの廃止
- DIの廃止（低レイヤーで問題が発生したため）
- クリーンアーキテクチャ (DDD)の採用

## 機能一覧

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
