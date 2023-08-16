# このリポジトリについて

このリポジトリはRust開発用のテンプレート用リポジトリです。基本的なエディタ設定やサブディレクトリに配置した複数モジュール構成などが初期状態で組み込まれています。このリポジトリの内容を元にして新規のリポジトリを作成するために利用したり、Cargo.tomlやその他設定ファイルの記述方法を確認するためのサンプルとして活用してください。

このリポジトリが提供する構成:

- サブディレクトリ内に配置した複数のモジュール構成
- 複数のバイナリクレート
- ライブラリクレート
- テストケース(ドキュメンテーションテスト、単体テスト、結合テスト)

## Rustの開発用にエディタ(VSCode)を設定する

- VSCodeにrust-analyzerプラグインをインストールする。
- rust用のフォーマットツールrustfmtをインストールする。
- ファイル保存時にrustfmtで自動的にフォーマットする設定を行う。

### rustfmtのインストール

下記のコマンドを実行してrustfmtをインストールします。

```
$ rustup component add rustfmt
```

### ファイル保存時に自動でrustfmtを実行する設定を行う

[`.vscode/settings.json`](.vscode/settings.json)に下記の設定を追加する。

```json
"editor.formatOnSave": true
```

### Rustのソースコードフォーマットを調整する

基本的にはデフォルトの設定をそのまま使えば問題ないので調整は必要ない。それでも調整を行いたい場合は、[`.rustfmt.toml`](.rustfmt.toml)ファイルに設定を追加することで、デフォルトの設定に対する上書き、または追加を行うことができる。

## プロジェクトをビルドする

下記コマンドでプロジェクトのビルドを行う。

```
$ cargo build
```

ビルドに成功すると3つのバイナリと1つのライブラリが`target`ディレクトリ内に生成される。

## プロジェクトを実行する

複数バイナリが存在する場合は、どのバイナリを実行するかを指定してcargo runを実行する。

```
$ cargo run --bin main2
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/main2`
Hello, main2!
```

## テストを実行する

下記のコマンドで全てのテストを実行する

```
$ cargo test
```