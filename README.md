# HLHSInfo Backend Server (HLHSBS)

這是一套基於網頁爬蟲的學校資料擷取系統，這個儲存庫僅包含伺服器端(API端)的代碼。

## Installation

> **Note**  
> 我們建議您直接使用`release`裡的建置版本。

> **Warning**  
> 目前的版本我們僅對Linux版本有Service安裝工具，Windows無安裝工具。

### Linux

您可以直接於Linux系統上將`HLHSBS`安裝成系統的服務。使用以下指令安裝

```shell
./service_setting.sh install
```

安裝過後，`HLHSBS`將會自動啟動。如果您需要禁用服務，請使用以下指令

```shell
./service_setting.sh disable
```

啟用服務

```shell
./service_setting.sh enable
```

若您像要解除安裝`HLHSBS`服務，請使用以下指令

```shell
./service_setting.sh uninstall
```

### Windows

> **Note**  
> 我們並無特別於Windows系統上設定服務檔案。若您需要將`HLHSBS`設定為Windows上的服務，請自行查詢安裝方法。

您可以直接執行`hlhsinfo_backend_server.exe`來啟動`HLHSBS`

## Configuration

系統       | 位置
---------- | --------------------
`Linux`    | `/usr/etc/hlhsinfo_backend_server/config.yaml`
`Windows`  | `%APPDATA%\hlhsinfo_backend_server\config.yaml`

您可以設定以下參數 (預設自動生成設定檔)

> **Warning**  
> 所有設定皆為必填。若沒有滿足所有參數，`HLHSBS`將不會執行

<!-- TODO -->