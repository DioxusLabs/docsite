# 贡献

开发工作在 [Dioxus GitHub 仓库](https://github.com/DioxusLabs/dioxus)中进行。如果您发现了 bug 或者有新功能的想法，请提交一个 issue（但请先检查是否已经有人提交过了）。

[GitHub 讨论](https://github.com/DioxusLabs/dioxus/discussions)可以用作寻求帮助或讨论功能的地方。您也可以加入 [我们的 Discord 频道](https://discord.gg/XgGxMSkvUM)，在那里有一些开发讨论。

## 改进文档

如果您想改进文档，欢迎提供 PR！Rust 文档（[源代码](https://github.com/DioxusLabs/dioxus/tree/main/packages)）和这个指南（[源代码](https://github.com/DioxusLabs/docsite/tree/main/docs-src/0.5/en)）可以在各自的 GitHub 仓库中找到。

## 生态系统工作

让 React 变得伟大的部分是丰富的生态系统。我们也希望 Dioxus 有同样的生态系统！因此，如果您有一个想要编写的库，并且许多人会从中受益，那么将会受到欢迎。您可以在 [npm.js](https://www.npmjs.com/search?q=keywords:react-component) 上查找灵感。完成后，请将您的库添加到 [awesome dioxus](https://github.com/DioxusLabs/awesome-dioxus) 列表中，或在 [Discord](https://discord.gg/XgGxMSkvUM) 的 `#I-made-a-thing` 频道中分享。

## Bug & Features

如果您修复了 [一个开放的问题](https://github.com/DioxusLabs/dioxus/issues)，请随时提交 PR！您也可以查看 [路线图](./roadmap.md) 并开始处理其中的某个任务。考虑先 [联系团队](https://discord.gg/XgGxMSkvUM) ，以确保所有人都在同一页面上，并且您不会做无用的工作！

所有的 Pull Request（包括团队成员提交的）都必须由至少一名其他团队成员批准。
关于设计、架构、重大更改、权衡等更大、更微妙的决策是通过团队共识来做出的。

## 贡献前

您可能会惊讶地发现，在首次提交 PR 时，许多检查都会失败。
这就是为什么在贡献之前您应该首先运行这些命令，它将为您节省 *大量* 的时间，因为 GitHub CI 执行所有这些命令要比您的 PC 慢得多。

- 使用 [rustfmt](https://github.com/rust-lang/rustfmt) 格式化代码：

```sh
cargo fmt -- src/**/**.rs
```

- 您可能需要在 Linux（Ubuntu/deb）上安装一些软件包，然后以下命令才能成功完成（存储库根目录中还有一个 Nix flake）：

```sh
sudo apt install libgdk3.0-cil libatk1.0-dev libcairo2-dev libpango1.0-dev libgdk-pixbuf2.0-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev libwebkit2gtk-4.1-dev
```

- 检查所有代码 [cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html)：

```sh
cargo check --workspace --examples --tests
```

- 检查 [Clippy](https://doc.rust-lang.org/clippy/) 是否生成任何警告。请修复这些警告！

```sh
cargo clippy --workspace --examples --tests -- -D warnings
```

- 使用 [cargo-test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) 测试所有代码：

```sh
cargo test --all --tests
```

- 更多测试，这次使用 [cargo-make](https://sagiegurari.github.io/cargo-make/)。以下是所有步骤，包括安装：

```sh
cargo install --force cargo-make
cargo make tests
```

- 使用 [MIRI](https://github.com/rust-lang/miri) 测试不安全的 crate。目前，这用于 `dioxus-core` 和 `dioxus-native-core` 中的两个 MIRI 测试：

```sh
cargo miri test --package dioxus-core --test miri_stress
cargo miri test --package dioxus-native-core --test miri_native
```

- 使用 Playwright 进行测试。这在浏览器中直接测试 UI。以下是所有步骤，包括安装：
  **免责声明：这可能会在您的计算机上无缘无故失败，而不是您的错。** 无论如何，请提交 PR！

```sh
cd playwright-tests
npm ci
npm install -D @playwright/test
npx playwright install --with-deps
npx playwright test
```

## 如何使用本地 crate 测试 dioxus
如果您正在开发一个功能，您应该在提交 PR 之前在您的本地设置中测试它。这个过程可以确保您了解您的代码功能，然后由同行审查。

- Fork 下面的 github 存储库 (DioxusLabs/dioxus):

`https://github.com/DioxusLabs/dioxus`

- 创建一个新的或者使用一个已经存在的 rust crate (如果您将使用一个已经存在的 rust crate，请忽略此步骤):
  这是我们将测试 forked 中的功能的地方

```sh
cargo new --bin demo
```

- 在 Cargo.toml 中为您的 rust crate（新的/现有的）添加 dioxus 依赖项：

```toml
dioxus = { path = "<path to forked dioxus project>/dioxus/packages/dioxus", features = ["web", "router"] }
```

上面的示例是为了 dioxus-web，使用 dioxus-router。要了解不同渲染器的依赖关系，请访问[此处](https://dioxuslabs.com/learn/0.5/getting_started)。

- 运行和测试您的功能

```sh
dx serve
```

如果这是您第一次使用 dioxus，请阅读 [指南](https://dioxuslabs.com/learn/0.5/guide) 以熟悉 dioxus。
