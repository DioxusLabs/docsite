# Mobile App

Build a mobile app with Dioxus!

## Support

The Rust ecosystem for mobile is still in its infancy. Mobile is a 1st-class target for Dioxus apps, but there are very few packages that are battle-tested and ready to use.

Mobile apps are rendered with either the platform's WebView or experimentally with [WGPU](https://github.com/DioxusLabs/blitz). WebView doesn't support animations, transparency, and native widgets.


Mobile support is currently best suited for CRUD-style apps, ideally for internal teams who need to develop quickly but don't care much about animations or native widgets.

## Getting Set up

## Android

Android devices run a different executable architecture than desktop and web. We need to install these toolchains to build Dioxus apps for Android.

First, install the Rust Android targets:

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

To develop on Android, you will need to [install Android Studio](https://developer.android.com/studio).

Once you have installed Android Studio, you will need to install the Android SDK and NDK:

1. Create a blank Android project
2. Select `Tools > SDK manager`
3. Navigate to the `SDK tools` window:

![NDK install window](/assets/static/android_ndk_install.png)

Then select:
- The SDK
- The SDK Command line tools
- The NDK (side by side)
- CMAKE

4. Select `apply` and follow the prompts

> More details that could be useful for debugging any errors you encounter are available [in the official android docs](https://developer.android.com/studio/intro/update#sdk-manager)

Next set the Java, Android, NDK, and PATH variables:

Mac:
```sh
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/25.2.9519653"
export PATH="$PATH:$ANDROID_HOME/emulator"
```

Windows:
```powershell
[System.Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Android\Android Studio\jbr", "User")
[System.Environment]::SetEnvironmentVariable("ANDROID_HOME", "$env:LocalAppData\Android\Sdk", "User")
[System.Environment]::SetEnvironmentVariable("NDK_HOME", "$env:LocalAppData\Android\Sdk\ndk\25.2.9519653", "User")
```

> The NDK version in the paths should match the version you installed in the last step

We manually set the PATH variable to include the Android emulator since some distributions of Android Studio include the emulator in the wrong location.

## IOS

To develop on IOS, you will need to [install XCode](https://apps.apple.com/us/app/xcode/id497799835). Also make sure to install the iOS targets

```sh
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

> If you are using M1, you will have to run `cargo build --target x86_64-apple-ios` instead of `cargo apple build` if you want to run in simulator.

You will also need to install the iOS SDK and the Xcode command line tools.

## Running your app

Starting with Dioxus 0.6, `dx` ships with built-in support for mobile.

Simply create a new Dioxus project:

```sh
dx new my-app
```

Make sure to launch the relevant mobile simulator. For Android, you can use the Android Studio emulator, or the Android Emulator in the terminal. Make sure to adjust the device name depending on which emulator you installed.

```shell
emulator -avd Pixel_6_API_34  -netdelay none -netspeed full
```

For iOS, you can use the iOS simulator. You can launch it with:

```sh
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
xcrun simctl boot "iPhone 15 Pro Max"
```

And then run the app with:

```sh
cd my-app
dx serve
```

This will start the app in development mode.
