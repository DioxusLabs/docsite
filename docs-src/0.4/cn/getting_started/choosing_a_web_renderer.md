# 选择一个网络渲染器

Dioxus有三种不同的渲染器来针对网络项目:

- [dioxus-web](wasm.md)允许您在客户端上使用[WebAssembly](https://rustwasm.github.io/docs/book/)将程序渲染为HTML
- [dioxus-liveview](liveview.md)允许您在服务器上运行应用程序，并使用websocket在客户端上将其渲染到HTML
- [dioxus-fullstack](fullstack.md)允许您在服务器上初始简单渲染静态HTML,然后使用[WebAssembly](https://rustwasm.github.io/docs/book/)从客户端更新该HTML

每种方法都有其权衡和利弊：

### Dioxus Liveview

- Liveview渲染通过WebSocket连接与服务器通信。它本质上将客户端渲染所做的所有工作移动到服务器完成。

- 这使得 **与服务器通信变得容易，但与客户端/浏览器的APIS交互更加困难**。

- 每次交互还需要将消息发送到服务器并返回，这可能会导致**延迟问题**.

- 由于Liveview使用websocket进行渲染，因此在建立WebSocket连接并从websocket发送第一个渲染之前，页面将是空白的。就像客户端渲染一样，这使得页面对**SEO不那么友好**。

- 由于页面在服务器上渲染，并且页面逐块发送到客户端，因此你永远无需将整个应用程序发送到客户端。初始加载时间可能比大型应用程序的客户端渲染更快，因为Liveview只需要发送一个恒定的小websocket脚本，无论应用程序的大小。

> Liveview非常适合已经需要经常与服务器通信（如实时协作应用程序）的应用程序，但不需要太多与客户端/浏览器API通信的情况。

[![](https://mermaid.ink/img/pako:eNplULFOw0AM_RXLc7Mw3sBQVUIMRYgKdcli5ZzkRHIuPl8QqvrvXJICRXiy3nt-9-6dsRHP6DAZGe8CdUpjNd3VEcpsVT4SK1TVPRxYJ1YHL_yeOdkqWMGF3w4U32Y6nSQmXvknMQYNXW8g7bfk2JPBg0g3MCTmdH1rJhenx2is1FiYri43wJ8or3O2H1Liv0w3hw724kMb2MMzdcUYNziyjhR8-f15Pq3Reh65RldWzy3lwWqs46VIKZscPmODzjTzBvPJ__aFrqUhFZR9MNH92uhS7OULYSF1lw?type=png)](https://mermaid.live/edit#pako:eNplULFOw0AM_RXLc7Mw3sBQVUIMRYgKdcli5ZzkRHIuPl8QqvrvXJICRXiy3nt-9-6dsRHP6DAZGe8CdUpjNd3VEcpsVT4SK1TVPRxYJ1YHL_yeOdkqWMGF3w4U32Y6nSQmXvknMQYNXW8g7bfk2JPBg0g3MCTmdH1rJhenx2is1FiYri43wJ8or3O2H1Liv0w3hw724kMb2MMzdcUYNziyjhR8-f15Pq3Reh65RldWzy3lwWqs46VIKZscPmODzjTzBvPJ__aFrqUhFZR9MNH92uhS7OULYSF1lw)

### Dioxus Web

- 通过客户端渲染，将应用程序发送到客户端，然后客户端动态生成页面的所有HTML。

- 这意味着页面将是空白的，直到JavaScript捆绑包加载并初始化应用程序。这可能会导致**首次渲染时间较慢，SEO性能差**。

> SEO代表搜索引擎优化(Search Engine Optimization)。它指的是使您的网站更有可能出现在搜索引擎结果中的做法。像谷歌和必应这样的搜索引擎使用网络爬虫对网站内容进行索引。这些爬虫大多无法运行JavaScript，因此如果页面在客户端渲染，他们将无法索引您的页面内容。

- 客户端渲染应用程序需要使用**弱类型请求与服务器通信**。

> 客户端渲染是大多数应用程序的良好起点。它得到了很好的支持，并且很容易与客户端/浏览器的API进行通信。

[![](https://mermaid.ink/img/pako:eNpVkDFPwzAQhf-KdXOzMHpgqJAQAwytEIsXK35JLBJfez4Xoar_HSemQtzke9_z2e-u1HMAWcrqFU_Rj-KX7vLgkqm1F_7KENN1j-YIuUCsOeBckLUZmrjx_ezT54rziVNG42-sMBLHSQ0Pd8vH5NU8M48zTAby71sr3CYdkAIEoen37h-y5n3910tSiO81cqIdLZDFx1DDXNerjnTCAke2HgMGX2Z15NKtWn1RPn6nnqxKwY7KKfzFJzv4OVcVISrLa1vQtqfbDzd0ZKY?type=png)](https://mermaid.live/edit#pako:eNpVkDFPwzAQhf-KdXOzMHpgqJAQAwytEIsXK35JLBJfez4Xoar_HSemQtzke9_z2e-u1HMAWcrqFU_Rj-KX7vLgkqm1F_7KENN1j-YIuUCsOeBckLUZmrjx_ezT54rziVNG42-sMBLHSQ0Pd8vH5NU8M48zTAby71sr3CYdkAIEoen37h-y5n3910tSiO81cqIdLZDFx1DDXNerjnTCAke2HgMGX2Z15NKtWn1RPn6nnqxKwY7KKfzFJzv4OVcVISrLa1vQtqfbDzd0ZKY)

### Dioxus Fullstack

全栈渲染分为两部分：
1. 页面在服务器上呈现。这可以包括获取渲染页面所需的任何数据。
2. 页面在客户端上“注水”。(“注水”是指从服务器上获取HTML页面，并在客户端上添加Dioxus所需的所有事件侦听器)。在此之后，对页面的任何更新都会发生在客户端上。

由于页面最初是在服务器上呈现的，因此页面在发送到客户端时将完全呈现。这导致首次渲染时间更快，并使页面对搜索引擎优化更友好。

- **快速初始渲染**
- **SEO友好**
- **与服务器以一种安全轻松的方式通信**
- **可以访问客户端/浏览器API**
- **快速交互性**

最后，我们可以使用[服务器功能](../reference/fullstack/server_functions.md)以类型安全的方式与服务器通信。

这种方法同时使用dioxus-web和dioxus-ssr这两个crates。为了整合这两个软件包和`axum`, `warp`, 或 `salvo`, Dioxus提供了`dioxus-fullstack` crate。

全栈应用程序可能会更复杂，因为您的代码在两个不同的地方运行。Dioxus试图通过服务函数和其他方法来缓解这种情况。

[![](https://mermaid.ink/img/pako:eNpdkL1uwzAMhF9F4BwvHTV0KAIUHdohQdFFi2CdbQG2mFCUiyDIu9e2-hOUE3H34UDelVoOIEtZvWIffS9-auYHl8wyT8KfGWKa5tEcITPEmgPOBVkrUMXNPyAFCMJK5BOnjIq8scJI7Ac13N1RH4NX88zcjzAZyJX-8bfIl6QQ32qcv7PuhP-ANe_rpb8KJ9rRBJl8DMt71zXAkQ6Y4Mgua0Dny6iOXLotqC_Kx0tqyaoU7Kicwl8hZDs_5kVFiMryWivbmrt9AacxbGg?type=png)](https://mermaid.live/edit#pako:eNpdkL1uwzAMhF9F4BwvHTV0KAIUHdohQdFFi2CdbQG2mFCUiyDIu9e2-hOUE3H34UDelVoOIEtZvWIffS9-auYHl8wyT8KfGWKa5tEcITPEmgPOBVkrUMXNPyAFCMJK5BOnjIq8scJI7Ac13N1RH4NX88zcjzAZyJX-8bfIl6QQ32qcv7PuhP-ANe_rpb8KJ9rRBJl8DMt71zXAkQ6Y4Mgua0Dny6iOXLotqC_Kx0tqyaoU7Kicwl8hZDs_5kVFiMryWivbmrt9AacxbGg)
