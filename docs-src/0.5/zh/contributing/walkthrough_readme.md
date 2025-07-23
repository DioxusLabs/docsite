# Hello World示例内部的操作步骤

本操作步骤将带您了解Hello World示例程序的内部操作。它将解释Dioxus内部的主要部分如何相互作用，以将readme示例从源文件转换为运行中的应用程序。本指南应该作为Dioxus内部操作的高级概述。它不是一份全面的指南。

核心crate的大致工作方式如下：

![](https://mermaid.ink/img/pako:eNqNk01v2zAMhv8KocsuTQ876lCgWAb0sGDD0mMAg7PoWogsBvpwWhT976MlJ3OKbKtOEvmIfEWRr6plQ0qrmDDR2uJTwGE1ft55kBXIGwqNHQYyVvywWt3BA3rjKGj4gs5BX0-V_1n4QtUthW_Mh6WzWgryg537OpJPsQJ_zsX9PrmG0fBwWxM2NIH1nmdRFuxTn4C7K4mn9djTpYAjWsnTcQBaSJiWxIcULEVILCIiu5Egyf3RhpTRwfr75tOC73LKggGmQkUcBLcDVUJyFoF_qcEkoxEVzZHDvjIXpnOhtm1PJp8rvcGw37Z8oPu4FlkvhVvbrivGypyP_3dWXRo2WdrAsp-fN391Qd5n1BBnSU0-GDy9sHyGo678xcOyOU7fMHcMHINNtcgIPfP-Wr2WAu6NeeRzGTS0z7fxgEd_7T3_Zi8b5kp1T1IxvvgWfjlu9x-SexHqo1VTN2qgMKA1MoavU6CdkkaSBlJatoY6zC7t1M6_CYo58VZUKZ1CphtVo8yDq3SHLopVJiZx2NTRLhP-9htxEk8q?type=png)

## 源文件

我们从一个Hello World程序开始。该程序在webview中呈现一个带有文本“Hello World”的桌面应用程序。

```rust, no_run
{{#include ../docs-router/src/doc_examples/readme.rs}}
```

[![](https://mermaid.ink/img/pako:eNqNkT1vwyAQhv8KvSlR48HphtQtqjK0S6tuSBGBS0CxwcJHk8rxfy_YVqxKVdR3ug_u4YXrQHmNwOFQ-bMyMhB7fReOJbVxfwyyMSy0l7GSpW1ARda727ksUy5MuSyKgvBC5ULA1h5N8WK_kCkfHWHgrBuiXsBynrvdsY9E3u1iM_eyvFOVVadMnELOap-o1911JLPHZ1b-YqLTc3LjTt7WifTZMJPsPdx1ov3Z_ellfcdL8R8vmTy5eUqsTUpZ-vzZzjAEK6gx1NLqtJwuNwSQwRoF8BRqGU4ChOvTORnJf3w7BZxCxBXERkvCjZXpQTXwg6zaVEVtyYe3cdvD0vsf4bucgw)](https://mermaid.live/edit#pako:eNqNkT1vwyAQhv8KvSlR48HphtQtqjK0S6tuSBGBS0CxwcJHk8rxfy_YVqxKVdR3ug_u4YXrQHmNwOFQ-bMyMhB7fReOJbVxfwyyMSy0l7GSpW1ARda727ksUy5MuSyKgvBC5ULA1h5N8WK_kCkfHWHgrBuiXsBynrvdsY9E3u1iM_eyvFOVVadMnELOap-o1911JLPHZ1b-YqLTc3LjTt7WifTZMJPsPdx1ov3Z_ellfcdL8R8vmTy5eUqsTUpZ-vzZzjAEK6gx1NLqtJwuNwSQwRoF8BRqGU4ChOvTORnJf3w7BZxCxBXERkvCjZXpQTXwg6zaVEVtyYe3cdvD0vsf4bucgw)

## rsx!宏

在Rust编译器运行程序之前，它将展开所有[宏](https://doc.rust-lang.org/reference/procedural-macros.html)。这是hello world示例展开后的样子：

```rust, no_run
{{#include ../docs-router/src/doc_examples/readme_expanded.rs}}
```

rsx宏将rsx的静态部分（模板）和动态部分（[dynamic_nodes](https://docs.rs/dioxus-core/0.5.0/dioxus_core/prelude/struct.VNode.html#structfield.dynamic_nodes)和[dynamic_attributes](https://docs.rs/dioxus-core/0.5.0/dioxus_core/prelude/struct.VNode.html#structfield.dynamic_attrs)）分开。

静态模板只包含rsx的在运行时不会改变的部分，其中包含动态部分的占位符：

[![](https://mermaid.ink/img/pako:eNqdksFuwjAMhl8l8wkkKtFx65njdtm0E0GVSQKJoEmVOgKEeHecUrXStO0wn5Lf9u8vcm6ggjZQwf4UzspiJPH2Ib3g6NLuELG1oiMkp0TsLs9EDu2iUeSCH8tz2HJmy3lRFPrqsXGq9mxeLzcbCU6LZSUGXWRdwnY7tY7Tdoko-Dq1U64fODgiUfzJMeuOe7_ZGq-ny2jNhGQu9DqT8NUK6w72RcL8dxgdzv4PnHLAKf-Fk80HoBUDrfkqeBkTUd8EC2hMbNBpXtYtJySQNQ0PqPioMR4lSH_nOkwUPq9eQUUxmQWkViOZtUN-UwPVHk8dq0Y7CvH9uf3-E9wfrmuk1A)](https://mermaid.live/edit#pako:eNqdksFuwjAMhl8l8wkkKtFx65njdtm0E0GVSQKJoEmVOgKEeHecUrXStO0wn5Lf9u8vcm6ggjZQwf4UzspiJPH2Ib3g6NLuELG1oiMkp0TsLs9EDu2iUeSCH8tz2HJmy3lRFPrqsXGq9mxeLzcbCU6LZSUGXWRdwnY7tY7Tdoko-Dq1U64fODgiUfzJMeuOe7_ZGq-ny2jNhGQu9DqT8NUK6w72RcL8dxgdzv4PnHLAKf-Fk80HoBUDrfkqeBkTUd8EC2hMbNBpXtYtJySQNQ0PqPioMR4lSH_nOkwUPq9eQUUxmQWkViOZtUN-UwPVHk8dq0Y7CvH9uf3-E9wfrmuk1A)

动态节点和动态属性是在运行时可以更改的rsx部分：

[![](https://mermaid.ink/img/pako:eNp1UcFOwzAM_RXLVzZpvUbighDiABfgtkxTlnirtSaZUgc0df130hZEEcwny35-79nu0EZHqHDfxA9bmyTw9KIDlGjz7pDMqQZ3DsazhVCQ7dQbwnEiKxwDvN3NqhN4O4C3q_VaIztYKXjkQ7184HcCG3MQSgq6Mes1bjbTPAV3RdqIJN5l-V__2_Fcf5iY68dgG7ZHBT4WD5ftZfIBN7dQ_Tj4w1B9MVTXGZa_GMYdcIGekjfsymW7oaFRavKkUZXUmXTUqENfcCZLfD0Hi0pSpgXmkzNC92zKATyqvWnaUiXHEtPz9KrxY_0nzYOPmA)](https://mermaid.live/edit#pako:eNp1UcFOwzAM_RXLVzZpvUbighDiABfgtkxTlnirtSaZUgc0df130hZEEcwny35-79nu0EZHqHDfxA9bmyTw9KIDlGjz7pDMqQZ3DsazhVCQ7dQbwnEiKxwDvN3NqhN4O4C3q_VaIztYKXjkQ7184HcCG3MQSgq6Mes1bjbTPAV3RdqIJN5l-V__2_Fcf5iY68dgG7ZHBT4WD5ftZfIBN7dQ_Tj4w1B9MVTXGZa_GMYdcIGekjfsymW7oaFRavKkUZXUmXTUqENfcCZLfD0Hi0pSpgXmkzNC92zKATyqvWnaUiXHEtPz9KrxY_0nzYOPmA)

## 启动应用

通过调用`launch`函数并提供根组件来启动应用程序。在内部，此函数将使用[wry](https://docs.rs/wry/latest/wry/)创建一个新的web视图，并使用readme示例中的根组件（`fn app()`）。本指南不会详细解释渲染器，但您可以在[自定义渲染器](/guide/custom-renderer)部分了解更多信息。

## 虚拟DOM

在我们深入讨论虚拟DOM中的初始渲染之前，我们需要讨论一下什么是虚拟DOM。虚拟DOM是DOM的表示，用于将当前DOM与新DOM进行差异比较。然后使用此差异创建需要应用于DOM以使其与虚拟DOM同步的变异列表。

虚拟DOM大致如下：

```rust, no_run
pub struct VirtualDom {
    // All the templates that have been created or set during hot reloading
    pub(crate) templates: FxHashMap<TemplateId, FxHashMap<usize, Template<'static>>>,

    // A slab of all the scopes that have been created
    pub(crate) scopes: ScopeSlab,

    // All scopes that have been marked as dirty
    pub(crate) dirty_scopes: BTreeSet<DirtyScope>,

    // Every element is actually a dual reference - one to the template and the other to the dynamic node in that template
    pub(crate) elements: Slab<ElementRef>,

    // This receiver is used to receive messages from hooks about what scopes need to be marked as dirty
    pub(crate) rx: futures_channel::mpsc::UnboundedReceiver<SchedulerMsg>,

    // The changes queued up to be sent to the renderer
    pub(crate) mutations: Mutations<'static>,
}
```

> 什么是 [slab](https://docs.rs/slab/latest/slab/)？
>
> slab 就像一个具有整数键的 hashmap，如果你不关心键的值。它内部由一个密集的向量支持，这使得它比 hashmap 更高效。当你将一个值插入到 slab 中时，它会返回一个整数键，你可以用它来以后检索值。

> Dioxus 如何使用 slab？
>
> Dioxus 使用“同步的 slab”来在渲染器和 VDOM 之间进行通信。当在虚拟 DOM 中创建一个节点时，会将 (elementId, mutation) 对传递给渲染器来标识该节点，然后渲染器将在实际 DOM 中渲染该节点。这些 id 也被虚拟 DOM 用于引用未来的突变，比如在节点上设置属性或移除节点。当渲染器向虚拟 DOM 发送事件时，它会发送事件触发的节点的 ElementId。虚拟 DOM 使用此 id 在 slab 中找到该节点，然后运行必要的事件处理程序。

虚拟 DOM 是一个作用域树。当组件首次渲染时，每个组件都会创建一个新的 `Scope`，并在组件卸载时回收。

作用域有三个主要用途：

1. 它们存储组件使用的 hook 的状态。
2. 它们为上下文 API 存储状态（例如：使用 [use_context_provider](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_context_provider.html)）。
3. 它们存储渲染的当前版本和先前版本的 `VNode`，以便对其进行差异比较以生成重新渲染所需的变化集。

### 初始渲染

创建并重建根作用域：

1. 运行根组件
2. 根组件返回一个 `VNode`
3. 为此 `VNode` 创建并添加突变到突变列表（这可能涉及创建新的子组件）
4. 将 `VNode` 存储在根的 `Scope` 中。

在构建完根的 `Scope` 后，所有生成的突变都会发送到渲染器，渲染器会将其应用到 DOM 上。

初始渲染后，根 `Scope` 如下所示：

[![](https://mermaid.ink/img/pako:eNqtVE1P4zAQ_SuzPrWikRpWXCLtBRDisItWsOxhCaqM7RKricdyJrQV8N93QtvQNCkfEnOynydv3nxkHoVCbUQipjnOVSYDwc_L1AFbWd3dB-kzuEQkuFLoDUwDFkCZAek9nGDh0RlHK__atA1GkUUHf45f0YbppAqB_aOzIAvz-t7-chN_Y-1bw1WSJKsglIu2w9tktWXxIIuHURT5XCqTYa5NmDguw2R8c5MKq2GcgF46WTB_jafi9rZL0yi5q4jQTSrf9altO4okCn1Ratwyz55Qxuku2ITlTMgs6HCQimsPmb3PvqVi-L5gjXP3QcnxWnL8JZLrwGvR31n0KV-Bx6-r-oVkT_-3G1S-NQLbk9i8rj7udP2cixed2QcDCitHJiQw7ub3EVlNecrPjudG2-6soFO5VbMECmR9T5OnlUY4-AFxfw9aTFst3McU9TK1Otm6NEn_DubBYlX2_dglLXOz48FgwJmJ5lZTlhz6xWgNaFnyDgpymcARHO0W2a9J_l5w2wYXvHuGPcqaQ-rESBQmFNJq3nCPNZoK3l4sUSR81DLMUpG6Z_aTFeHV0imRUKjMSFReSzKnVnKGhUimMi8ZNdoShl-rlfmyOUfCS_cPcePz_B_Wl4pc?type=png)](https://mermaid.live/edit#pako:eNqtVE1P4zAQ_SuzPrWikRpWXCLtBRDisItWsOxhCaqM7RKricdyJrQV8N93QtvQNCkfEnOynydv3nxkHoVCbUQipjnOVSYDwc_L1AFbWd3dB-kzuEQkuFLoDUwDFkCZAek9nGDh0RlHK__atA1GkUUHf45f0YbppAqB_aOzIAvz-t7-chN_Y-1bw1WSJKsglIu2w9tktWXxIIuHURT5XCqTYa5NmDguw2R8c5MKq2GcgF46WTB_jafi9rZL0yi5q4jQTSrf9altO4okCn1Ratwyz55Qxuku2ITlTMgs6HCQimsPmb3PvqVi-L5gjXP3QcnxWnL8JZLrwGvR31n0KV-Bx6-r-oVkT_-3G1S-NQLbk9i8rj7udP2cixed2QcDCitHJiQw7ub3EVlNecrPjudG2-6soFO5VbMECmR9T5OnlUY4-AFxfw9aTFst3McU9TK1Otm6NEn_DubBYlX2_dglLXOz48FgwJmJ5lZTlhz6xWgNaFnyDgpymcARHO0W2a9J_l5w2wYXvHuGPcqaQ-rESBQmFNJq3nCPNZoK3l4sUSR81DLMUpG6Z_aTFeHV0imRUKjMSFReSzKnVnKGhUimMi8ZNdoShl-rlfmyOUfCS_cPcePz_B_Wl4pc)

### 等待事件

只有在作用域被标记为脏时，虚拟 DOM 才会重新渲染一个 `Scope`。每个 hook 负责在状态发生变化时将作用域标记为脏。Hook 可以通过向虚拟 DOM 的通道发送消息来将作用域标记为脏。你可以在 dioxus 默认包含的 hook 的 [实现](https://github.com/DioxusLabs/dioxus/tree/main/packages/hooks) 中看到这是如何实现的。调用 `needs_update()` 也会导致 hook 将其作用域标记为脏。

一般有两种方式标记作用域为脏：

1. 渲染器触发事件：此事件上的事件监听器可能会被调用，如果处理事件导致任何生成的突变，则可能会将组件标记为脏。
2. 渲染器调用 [`wait_for_work`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.VirtualDom.html#method.wait_for_work)：这会轮询 dioxus 内部的未来队列。其中一个未来可能会将组件标记为脏。

一旦至少有一个作用域被标记为脏，渲染器就可以调用 [`render_immediate`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.VirtualDom.html#method.render_immediate) 来对脏作用域进行差异化处理。

### 差异化处理作用域

当用户点击“up high”按钮时，根 `Scope` 将由 `use_signal` hook 标记为脏。桌面渲染器将调用 `render_immediate`，这将对根 `Scope` 进行差异化处理。

要开始差异化处理过程，会运行组件函数。在运行根组件后，根 `Scope` 将如下所示：

[![](https://mermaid.ink/img/pako:eNrFVlFP2zAQ_iuen0BrpCaIl0i8AEJ72KQJtpcRFBnbJVYTn-U4tBXw33dpG5M2CetoBfdkny_ffb67fPIT5SAkjekkhxnPmHXk-3WiCVpZ3T9YZjJyDeDIDQcjycRCQVwmCTOGXEBhQEvtVvG1CWUldwo0-XX-6vVIF5W1GB9cWVbI1_PNL5v8jW3uPFbpmFOc2HK-GfA2WG1ZeJSFx0EQmJxxmUEupE01liEd394mVAkyjolYaFYgfu1P6N1dF8Yzua-cA51WphtTWzsLc872Zan9CnEGUkktuk6fFm_i5NxFRwn9bUimHrIvCT3-N2EBM70j5XBNOTwI5TrxmvQJkr7ELcHx67Jeggz0v92g8q0RaE-iP1193On6NyxecKUeJeFQaSdtTMLu_Xah5ctT_u94Nty2ZwU0zxWfxqQA5PecPq84kq9nfRw7SK0WDiEFZ4O37d34S_-08lFBVfb92KVb5HIrAp0WpjKYKeGyODLz0dohWIkaZNkiJqfkdLvIH6oRaTSoEmm0n06k0a5K0ZdpL61Io0Yt0nfpxc7UQ0_9cJrhyZ8syX-6brS706Mc489Vjja7fbWj3cxDqIdfJJqOaCFtwZTAV8hT7U0ovjBQRmiMS8HsNKGJfsE4Vjm4WWhOY2crOaKVEczJS8WwgAWNJywv0SuFcmB_rJ41y9fNiBqm_wA0MS9_AUuAiy0?type=png)](https://mermaid.live/edit#pako:eNrFVlFP2zAQ_iuen0BrpCaIl0i8AEJ72KQJtpcRFBnbJVYTn-U4tBXw33dpG5M2CetoBfdkny_ffb67fPIT5SAkjekkhxnPmHXk-3WiCVpZ3T9YZjJyDeDIDQcjycRCQVwmCTOGXEBhQEvtVvG1CWUldwo0-XX-6vVIF5W1GB9cWVbI1_PNL5v8jW3uPFbpmFOc2HK-GfA2WG1ZeJSFx0EQmJxxmUEupE01liEd394mVAkyjolYaFYgfu1P6N1dF8Yzua-cA51WphtTWzsLc872Zan9CnEGUkktuk6fFm_i5NxFRwn9bUimHrIvCT3-N2EBM70j5XBNOTwI5TrxmvQJkr7ELcHx67Jeggz0v92g8q0RaE-iP1193On6NyxecKUeJeFQaSdtTMLu_Xah5ctT_u94Nty2ZwU0zxWfxqQA5PecPq84kq9nfRw7SK0WDiEFZ4O37d34S_-08lFBVfb92KVb5HIrAp0WpjKYKeGyODLz0dohWIkaZNkiJqfkdLvIH6oRaTSoEmm0n06k0a5K0ZdpL61Io0Yt0nfpxc7UQ0_9cJrhyZ8syX-6brS706Mc489Vjja7fbWj3cxDqIdfJJqOaCFtwZTAV8hT7U0ovjBQRmiMS8HsNKGJfsE4Vjm4WWhOY2crOaKVEczJS8WwgAWNJywv0SuFcmB_rJ41y9fNiBqm_wA0MS9_AUuAiy0)

接下来，虚拟 DOM 将比较新的 VNode 与以前的 VNode，并仅更新树中发生更改的部分。由于这种方法，在重新渲染组件时，只会将树中发生更改的部分通过渲染器更新到 DOM 中。

差异化算法会遍历动态属性和节点的列表，并将它们与以前的 VNode 进行比较。如果属性或节点发生了更改，将会向突变列表中添加描述更改的突变。

这是根 `Scope` 的差异化算法的示例（红线表示生成了突变，绿线表示没有生成突变）

[![](https://mermaid.ink/img/pako:eNrFlFFPwjAQx7_KpT7Kko2Elya8qCE-aGLAJ5khpe1Yw9Zbug4k4He3OJjbGPig0T5t17tf_nf777aEo5CEkijBNY-ZsfAwDjW4kxfzhWFZDGNECxOOmYTIYAo2lsCyDG4xzVBLbcv8_RHKSG4V6orSIN0Wxrh8b2RYKr_uTyubd1W92GiWKg7aac6bOU3G803HbVk82xfP_Ok0JEqAT-FeLWJvpFYSOBbaSkMhCMnra5MgtfhWFrPWqHlhL2urT6atbU-oa0PNE8WXFFJ0-nazXakRroddGk9IwYEUnCd5w7Pddr5UTT8ZuVJY5F0fM7ebRLYyXNDgUnprJWxM-9lb7xAQLHe-M2xDYQCD9pD_2hez_kVn-P_rjLq6n3qjYv2iO5qz9DyvPdyv1ETp5eTTJ_7BGvQq8v1TVtl5jXUcRRcrqFh-dI4VtFlBN6t_ynLNkh5JpUmZEm5rbvfhkLiN6H4BQt2jYGYZklC_uzxWWJxsNCfUmkL2SJEJZuWdYs4cKaERS3IXlUJZNI_lGv7cxj2SMf2CeMx5_wBcbK19?type=png)](https://mermaid.live/edit#pako:eNrFlFFPwjAQx7_KpT7Kko2Elya8qCE-aGLAJ5khpe1Yw9Zbug4k4He3OJjbGPig0T5t17tf_nf777aEo5CEkijBNY-ZsfAwDjW4kxfzhWFZDGNECxOOmYTIYAo2lsCyDG4xzVBLbcv8_RHKSG4V6orSIN0Wxrh8b2RYKr_uTyubd1W92GiWKg7aac6bOU3G803HbVk82xfP_Ok0JEqAT-FeLWJvpFYSOBbaSkMhCMnra5MgtfhWFrPWqHlhL2urT6atbU-oa0PNE8WXFFJ0-nazXakRroddGk9IwYEUnCd5w7Pddr5UTT8ZuVJY5F0fM7ebRLYyXNDgUnprJWxM-9lb7xAQLHe-M2xDYQCD9pD_2hez_kVn-P_rjLq6n3qjYv2iO5qz9DyvPdyv1ETp5eTTJ_7BGvQq8v1TVtl5jXUcRRcrqFh-dI4VtFlBN6t_ynLNkh5JpUmZEm5rbvfhkLiN6H4BQt2jYGYZklC_uzxWWJxsNCfUmkL2SJEJZuWdYs4cKaERS3IXlUJZNI_lGv7cxj2SMf2CeMx5_wBcbK19)

## 结论

这仅是虚拟 DOM 工作原理的简要概述。本指南尚未涵盖的几个方面包括：

* 虚拟 DOM 如何处理异步组件
* 键控差异化
* 使用 [bump allocation](https://github.com/fitzgen/bumpalo) 来高效分配 VNode。

如果您需要更多关于虚拟 DOM 的信息，可以阅读 [core](https://github.com/DioxusLabs/dioxus/tree/main/packages/core) crate 的代码，或在 [Discord](https://discord.gg/XgGxMSkvUM) 上联系我们。
