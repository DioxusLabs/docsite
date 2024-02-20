# 移动应用程序

使用Dioxus构建一个移动应用程序！

示例: [Todo App](https://github.com/DioxusLabs/example-projects/blob/master/ios_demo)

## 支持

Mobile是目前Dioxus最不受支持的渲染目标。移动应用程序使用平台的WebView或使用[WGPU](https://github.com/DioxusLabs/blitz)进行实验性渲染。WebView不支持动画、透明度和原生小部件。


移动支持目前最适合CRUD风格的应用程序，非常适合需要快速开发但不太关心动画或原生小部件的内部团队。

## 开始设置

设置手机可能非常具有挑战性。这里的工具不是很好(还没有)，可能需要一些黑客手段才能让事情正常运行。

### 设置依赖项

#### Android

首先，安装Rust的安卓目标：

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

要在Android上开发，你需要[安装Android Studio](https://developer.android.com/studio).

在你安装了Android Studio后,您需要安装Android SDK和NDK：

1. 创建一个空白的Android项目
2. 选择 `工具 > SDK 管理器`
3. 导航到`SDK 工具` 窗口:

![NDK install window](/static/android_ndk_install.png)

然后选择：
- The SDK
- The SDK Command line tools
- The NDK (并排)
- CMAKE

4. 选择 `apply` 并按照提示操作

> 可能有助于调试您遇到的任何错误的更多详细信息可在[官方安卓文档](https://developer.android.com/studio/intro/update#sdk-manager)中找到

接下来设置Java、Android和NDK的家(Home)变量：

Mac:
```sh
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/25.2.9519653"
```

Windows:
```powershell
[System.Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Android\Android Studio\jbr", "User")
[System.Environment]::SetEnvironmentVariable("ANDROID_HOME", "$env:LocalAppData\Android\Sdk", "User")
[System.Environment]::SetEnvironmentVariable("NDK_HOME", "$env:LocalAppData\Android\Sdk\ndk\25.2.9519653", "User")
```

> 路径中的NDK版本应与您在最后一步中安装的版本相匹配

#### IOS

首先，安装Rust的IOS目标：

```sh
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

要在IOS上开发，您需要[安装XCode](https://apps.apple.com/us/app/xcode/id497799835).

> 注意: 在Apple silicon设备上你必须运行Xcode在rosetta上。转到应用程序 > 右键单击 Xcode > 获取信息 > 在Rosetta中打开。
> 如果你使用的是的M1,如果您想在模拟器中运行你必须先运行 `cargo build --target x86_64-apple-ios`而不是`cargo apple build`。

### 设置你的项目

首先，我们需要创建一个Rust项目:

```sh
cargo new dioxus-mobile-test
cd dioxus-mobile-test
```

接下来，我们可以使用`cargo-mobile2`创建一个移动项目：

```shell
cargo install --git https://github.com/tauri-apps/cargo-mobile2
cargo mobile init
```

当您运行`cargo mobile init`时，您会被问到一系列关于您项目的问题。其中一个问题是你应该使用什么模板。Dioxus目前在Tauri mobile中没有模板，相反，您可以使用`wry`模板。

> 可能还会被要求输入IOS的团队ID。您可以[在这里](https://developer.apple.com/help/account/manage-your-team/locate-your-team-id/)找到您的团队ID，或通过[在这里](https://developer.apple.com/help/account/get-started/about-your-developer-account)创建开发人员帐户来创建团队ID

接下来，我们需要修改我们的依赖项，以包括dioxus：

```sh
cargo add dioxus
cargo add dioxus-desktop --no-default-features --features tokio_runtime
```

最后，我们需要向渲染器添加一个组件。修改您的主要功能：

```rust
use dioxus::prelude::*;

pub fn main() -> Result<()> {
    // Right now we're going through dioxus-desktop but we'd like to go through dioxus-mobile
    // That will seed the index.html with some fixes that prevent the page from scrolling/zooming etc
    dioxus_desktop::launch_cfg(
        app,
        // Note that we have to disable the viewport goofiness of the browser.
        // Dioxus_mobile should do this for us
        dioxus_desktop::Config::default().with_custom_index(r#"<!DOCTYPE html>
        <html>
          <head>
            <title>Dioxus app</title>
            <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no" />
            <!-- CUSTOM HEAD -->
          </head>
          <body>
            <div id="main"></div>
            <!-- MODULE LOADER -->
          </body>
        </html>
       "#.into()),
    );

    Ok(())
}

fn app(cx: Scope) -> Element {
    let items = cx.use_hook(|| vec![1, 2, 3]);

    log::debug!("Hello from the app");

    render! {
        div {
            h1 { "Hello, Mobile"}
            div { margin_left: "auto", margin_right: "auto", width: "200px", padding: "10px", border: "1px solid black",
                button {
                    onclick: move|_| {
                        println!("Clicked!");
                        items.push(items.len());
                        cx.needs_update_any(ScopeId(0));
                        println!("Requested update");
                    },
                    "Add item"
                }
                for item in items.iter() {
                    div { "- {item}" }
                }
            }
        }
    }
}
```

## 运行

从这开始, 您需要使用您目标的任何平台（模拟器或实际硬件）来构建crate。现在，我们只坚持使用模拟器

### IOS

要为IOS构建项目，您可以运行：
```sh
cargo build --target aarch64-apple-ios-sim
```

接下来，打开XCode(如果您以前从未打开过XCode，这可能需要一段时间)：
```sh
cargo apple open
```

这将打开这个特定项目的XCode。

从那里，只需单击具有正确目标的“播放”按钮，应用程序应该就会运行！

![ios_demo](/static/IOS-dioxus-demo.png)

请注意，单击播放不会导致新的构建，因此您需要在更改之间继续重建应用程序。这里的工具很早期，所以请耐心等待。如果您想为让事情变得更容易做出贡献，请这样做！我们很乐意提供帮助。


### Android

要在Android上构建您的项目，您可以运行：
`cargo android build`

接下来，打开Android studio:
```sh
cargo android open
```

这将为该应用程序打开一个安卓工作室项目。

接下来，我们需要在Android工作室中创建一个模拟器来运行我们的应用程序。要创建模拟器，请单击Android工作室右上角的手机图标：

![android studio manage devices](/static/android-studio-simulator.png)

然后单击`create a virtual device`按钮，然后按照提示操作：

![android studio devices](/static/android-studio-devices.png)

最后，通过单击您创建的设备上的播放按钮来启动您的设备：

![android studio device](/static/android-studio-device.png)

现在，您可以通过运行以下内容从终端启动应用程序：

```sh
cargo android run
```

![android_demo](/static/Android-Dioxus-demo.png)

> 更多信息可在Android文档中找到：
> - https://developer.android.com/ndk/guides
> - https://developer.android.com/studio/projects/install-ndk
> - https://source.android.com/docs/setup/build/rust/building-rust-modules/overview
