# Mobile App

Build a mobile app with Dioxus!

## Support

The Rust ecosystem for mobile continues to mature, with Dioxus offering strong support for mobile applications. Mobile is a first-class target for Dioxus apps, with a robust WebView implementation that supports CSS animations and transparency effects.

Mobile apps are rendered with either the platform's WebView or experimentally with WGPU. While native Android animations and widgets aren't currently supported, CSS-based animations and styling provide a powerful alternative.

Mobile support is well-suited for most application types, from business tools to consumer apps, making it an excellent choice for teams looking to build cross-platform applications with a single codebase.

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

> The NDK version (`<version>`) needs to be replaced with for instance 30.0.16138531

Mac:

```sh
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/<version>"
export PATH="$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"
```

Linux:

> If using the AUR Android SDK packages, ensure the SDK/NDK installation is writable by your user; Android tooling needs to modify the SDK installation.

```sh
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_SDK_ROOT="$HOME/Android/Sdk"
export ANDROID_NDK_HOME="$HOME/Android/Sdk/ndk/<version>"
export PATH="$PATH:$ANDROID_HOME/emulator"
export PATH="$PATH:$ANDROID_HOME/platform-tools"
export PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin"
```

Windows:

```powershell
[System.Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Android\Android Studio\jbr", "User")
[System.Environment]::SetEnvironmentVariable("ANDROID_HOME", "$env:LocalAppData\Android\Sdk", "User")
[System.Environment]::SetEnvironmentVariable("NDK_HOME", "$env:LocalAppData\Android\Sdk\ndk\<version>", "User")
```


We manually set the PATH variable to include the Android emulator since some distributions of Android Studio include the emulator in the wrong location.

### Android emulator setup

By default there are no sdk packages installed required to “create” an emulator, so we need to first install the SDK. Make sure to match the API level with the Android version you plan to emulate, for instance 35 is Android 15 while 36 corresponds to Android 16, see [SDK Platform release notes](https://developer.android.com/tools/releases/platforms).

```sh
# Android 17 emulator requires api version 37.0:
android sdk install \
    platform-tools \
    emulator \
    platforms/android-37.0 \
    system-images/android-37.0/google_apis/x86_64
```

We then use the previously installed sdk packages to create a new emulator:

```sh
# omitting --profile results in it being named medium_phone
android emulator create
```

To make sure we configured and set everything up correctly we can start the emulator:

```sh
android emulator start medium_phone
```


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

```sh
android emulator start medium_phone
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
