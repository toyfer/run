# run
## 概要
* runは仮想的な環境変数を提供するためのアプリケーションです
* run.exeを定義済みのPathへコピーし`config.toml`と`env.toml`を定義すれば、`run <alias>`コマンドを実行して特定のアプリケーションを起動することができます。
### 使い方
1. `run.exe`を定義済みPath内のディレクトリに配置します。
2. run.exeと同じディレクトリで`config.toml`を作成します。
```config.toml
# ここでは仮想的な環境変数を定義しているファイルに対するパスを指定します。
env_vars_path = "env.toml"
```
3. `env.toml`を作成します。
```env.toml
# aliasセクションでは、エイリアス名とそれに対応する実行ファイルのパスを指定します。
[alias]
example_alias = "C:\\Path\\Tp\\Application.exe"
