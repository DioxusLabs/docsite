Today, we are announcing that the Dioxus team is joining Cognition to accelerate the development of Devin, Cognition’s autonomous cloud coding agent.

In addition to helping advance Devin, the Dioxus team will also continue to work on Dioxus, Blitz, Taffy, and Subsecond, and to maintain our wide array of open source projects for the benefit of the community.

## What got us here

Late in 2025, we noticed that LLM code generation got really, really good at writing Rust. This caught us off guard: the models, seemingly from improvements in reinforcement learning, could finally emit passable Rust code. We saw this in downloads too; more and more users were now picking up Rust and Dioxus to build their apps. The biggest ask from users was, and still is, a more comprehensive ecosystem of Rust crates for things like camera, Bluetooth, and storage.

Given the surge in Dioxus use, our own increased AI-assisted ambition, and scale of OS-specific work on the roadmap, we decided to build our own cloud coding harness called SkyVM. SkyVM was engineered to support all major operating systems – specifically macOS/iOS, Android, all Linux variants, and Windows. We handrolled new virtualization technologies to achieve secure VM snapshotting, forking, and rollbacks, so new VMs could spawn and resume in <50ms.

Concurrently, Cognition was hard at work shipping their new Devin CLI. We learned that the Devin CLI terminal renderer is built with Dioxus; our old deprecated TUI renderer had been rebuilt and reimagined! We were invited to the Cognition office for a demo and it was immediately clear that both teams shared a vision for the future. Our team was eager to help shape the future of software engineering, and Cognition proved to be a great fit.

## Continuing our Dioxus work: A focus on Blitz and Dioxus-Native

By joining Cognition, we will naturally have less time to devote entirely to Dioxus. Fortunately, Cognition is extremely supportive of our work and, given that Dioxus is a dependency in their stack, is excited for the team to continue to push Dioxus forward. Our lead engineer on Blitz, Nico Burns, will be devoted full time to maintaining and improving Dioxus. Since joining Cognition, Nico’s productivity has actually increased *substantially* – the development of Blitz, Taffy, and Parley have been accelerating.

Going forward, we will be investing heavily in Dioxus-Native and Blitz, our native HTML/CSS renderer. While we are still very excited about Dioxus itself, we do recognize that technologies like Blitz, Subsecond, wasm-split, and DX will do more in the hands of everyone, not just developers who use Dioxus. Cloudflare, for example, recently leveraged Blitz as the core engine of their new agentic web browser.

## No need to monetize

Originally, Dioxus was just a side project to pass time during the last summer of Covid. It then grew into a budding open source project, then to a full time job, and then into a venture-backed startup. Collectively we raised $3.5M in venture funding. Many people were curious: how would we eventually monetize Dioxus?

By joining Cognition, we get the opportunity not only to continue our development of Dioxus and Blitz without financial pressure, but also to shape the future of software engineering with Devin. We are able to take many of the great ideas we had for Dioxus - like simplified cross-platform app development, rapid development speeds, a reimagined software deployment system - and build them for all developers. If you are interested in continuing to support the development of Dioxus, consider using Devin to build your next project!

## Thanks to Everyone Involved

Finally, I want to say thank you to everyone involved, from our investors at Y Combinator and Khosla Ventures, to our champions at Futurewei, and to the huge community we’ve built and fostered over the years. I’m extremely excited about the future of Dioxus, and, more broadly, software engineering. The industry is changing rapidly and we’re excited to help shape it.
