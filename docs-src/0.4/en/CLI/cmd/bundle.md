# Bundle

The `dioxus bundle` command can help you bundle a dioxus project.

```
dioxus-bundle
Bundle the Rust desktop app and all of its assets

USAGE:
    dioxus bundle [OPTIONS]

OPTIONS:
        --example <EXAMPLE>      [default: ""]
        --platform <PLATFORM>    [default: "default_platform"]
        --release                [default: false]
        --package <PACKAGE>      [default: platform specific]
```

You can use this command to create an installer for a project in the `out_dir`:

```
dioxus bundle --release
```

## Target Platform

Use option `platform` choose build target platform:

```
# for desktop project
dioxus bundle --platform desktop
```

`platform` only supports `desktop` & `web`.

```
# for web project
dioxus bundle --platform web
```

## Bundle Example

You can use `--example {name}` to build a example code.

```
# bundle the `example/test`
dioxus bundle --exmaple test
```

## Target Package

Depending on the target platform, the following packages are available:
Windows:
`msi`: (.msi)
Macos:
`macos`: (.app)
`ios`: IOS app bundle
`dmg`: macOS DMG bundle (.dmg)
Linux:
`deb`: Debian package bundle (.deb)
`rpm`: RPM package bundle (.rpm)
`appimage`: AppImage bundle (.AppImage)

```
# bundle a msi package for windows
dioxus bundle --package msi
```
