# Mobile App

Build a mobile app with Dioxus!

Example: [Todo App](https://github.com/DioxusLabs/example-projects/blob/master/ios_demo)

## Support

Mobile is currently the least-supported renderer target for Dioxus. Mobile apps are rendered with either the platform's WebView or experimentally with [WGPU](https://github.com/DioxusLabs/blitz). WebView doesn't support animations, transparency, and native widgets.


Mobile support is currently best suited for CRUD-style apps, ideally for internal teams who need to develop quickly but don't care much about animations or native widgets.

## Getting Set up

Getting set up with mobile can be quite challenging. The tooling here isn't great (yet) and might take some hacking around to get things working.

### Setting up dependencies

#### Android

First, install the rust Android targets:

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

To develop on Android, you will need to [install Android Studio](https://developer.android.com/studio).

Once you have installed Android Studio, you will need to install the Android SDK and NDK:

1. Create a blank Android project
2. Select `Tools > SDK manager`
3. Navigate to the `SDK tools` window:

![NDK install window](/static/android_ndk_install.png)

Then select:
- The SDK
- The SDK Command line tools
- The NDK (side by side)
- CMAKE

4. Select `apply` and follow the prompts

> More details that could be useful for debugging any errors you encounter are available [in the official android docs](https://developer.android.com/studio/intro/update#sdk-manager)

Next set the Java, Android and NDK home variables:

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

> The NDK version in the paths should match the version you installed in the last step

#### IOS

First, install the rust IOS targets:

```sh
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

To develop on IOS, you will need to [install XCode](https://apps.apple.com/us/app/xcode/id497799835).

> Note: On Apple silicon you must run Xcode on rosetta. Goto Application > Right Click Xcode > Get Info > Open in Rosetta.
> If you are using M1, you will have to run `cargo build --target x86_64-apple-ios` instead of `cargo apple build` if you want to run in simulator.

### Setting up your project

First, we need to create a rust project:

```sh
cargo new dioxus-mobile-test
cd dioxus-mobile-test
```

Next, we can use `cargo-mobile2` to create a project for mobile:

```shell
cargo install --git https://github.com/tauri-apps/cargo-mobile2
cargo mobile init
```

When you run `cargo mobile init`, you will be asked a series of questions about your project. One of those questions is what template you should use. Dioxus currently doesn't have a template in Tauri mobile, instead you can use the `wry` template.

> You may also be asked to input your team ID for IOS. You can find your team id [here](https://developer.apple.com/help/account/manage-your-team/locate-your-team-id/) or create a team id by creating a developer account [here](https://developer.apple.com/help/account/get-started/about-your-developer-account)

Next, we need to modify our dependencies to include dioxus:

```sh
cargo add dioxus
cargo add dioxus-desktop --no-default-features --features tokio_runtime
```

Finally, we need to add a component to renderer. Modify your main function:

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

## Running 

From there, you'll want to get a build of the crate using whichever platform you're targeting (simulator or actual hardware). For now, we'll just stick with the simulator

### IOS

To build your project for IOS, you can run:
```sh
cargo build --target aarch64-apple-ios-sim
```

Next, open XCode (this might take awhile if you've never opened XCode before):
```sh
cargo apple open
```

This will open XCode with this particular project.

From there, just click the "play" button with the right target and the app should be running!

![ios_demo](/static/IOS-dioxus-demo.png)

Note that clicking play doesn't cause a new build, so you'll need to keep rebuilding the app between changes. The tooling here is very young, so please be patient. If you want to contribute to make things easier, please do! We'll be happy to help.


### Android

To build your project on Android you can run:
`cargo android build`

Next, open Android studio:
```sh
cargo android open
```

This will open an android studio project for this application.

Next we need to create a simulator in Android studio to run our app in. To create a simulator click on the phone icon in the top right of Android studio:

![android studio manage devices](/static/android-studio-simulator.png)

Then click the `create a virtual device` button and follow the prompts:

![android studio devices](/static/android-studio-devices.png)

Finally, launch your device by clicking the play button on the device you created:

![android studio device](/static/android-studio-device.png)

Now you can start your application from your terminal by running:

```sh
cargo android run
```

![android_demo](/static/Android-Dioxus-demo.png)

> More information is available in the Android docs:
> - https://developer.android.com/ndk/guides
> - https://developer.android.com/studio/projects/install-ndk
> - https://source.android.com/docs/setup/build/rust/building-rust-modules/overview
