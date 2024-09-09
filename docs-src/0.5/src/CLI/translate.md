# Translating existing HTML

Dioxus uses a custom format called RSX to represent the HTML because it is more concise and looks more like Rust code. However, it can be a pain to convert existing HTML to RSX. That's why Dioxus comes with a tool called `dx translate` that can automatically convert HTML to RSX!

Dx translate can make converting large chunks of HTML to RSX much easier! Lets try translating some of the HTML from the Dioxus homepage:

```sh
dx translate --raw  "<div class=\"relative w-full mx-4 sm:mx-auto text-gray-600\"><div class=\"text-[3em] md:text-[5em] font-semibold dark:text-white text-ghdarkmetal font-sans py-12 flex flex-col\"><span>Fullstack, crossplatform,</span><span>lightning fast, fully typed.</span></div><h3 class=\"text-[2em] dark:text-white font-extralight text-ghdarkmetal pt-4 max-w-screen-md mx-auto\">Dioxus is a Rust library for building apps that run on desktop, web, mobile, and more.</h3><div class=\"pt-12 text-white text-[1.2em] font-sans font-bold flex flex-row justify-center space-x-4\"><a href=\"/learn/0.5/getting_started\" dioxus-prevent-default=\"onclick\" class=\"bg-red-600 py-2 px-8 hover:-translate-y-2 transition-transform duration-300\" data-dioxus-id=\"216\">Quickstart</a><a href=\"/learn/0.5/reference\" dioxus-prevent-default=\"onclick\" class=\"bg-blue-500 py-2 px-8 hover:-translate-y-2 transition-transform duration-300\" data-dioxus-id=\"214\">Read the docs</a></div><div class=\"max-w-screen-2xl mx-auto pt-36\"><h1 class=\"text-md\">Trusted by top companies</h1><div class=\"pt-4 flex flex-row flex-wrap justify-center\"><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/futurewei_bw.png\"></div><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/airbuslogo.svg\"></div><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/ESA_logo.svg\"></div><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/yclogo.svg\"></div><div class=\"h-12 w-40 bg-black p-2 m-4 flex justify-center items-center\"><img src=\"static/satellite.webp\"></div></div></div></div>"
```

We get the following RSX you can easily copy and paste into your code:

```rs
div { class: "relative w-full mx-4 sm:mx-auto text-gray-600",
   div { class: "text-[3em] md:text-[5em] font-semibold dark:text-white text-ghdarkmetal font-sans py-12 flex flex-col",
      span { "Fullstack, crossplatform," }
      span { "lightning fast, fully typed." }
   }
   h3 { class: "text-[2em] dark:text-white font-extralight text-ghdarkmetal pt-4 max-w-screen-md mx-auto",
      "Dioxus is a Rust library for building apps that run on desktop, web, mobile, and more."
   }
   div { class: "pt-12 text-white text-[1.2em] font-sans font-bold flex flex-row justify-center space-x-4",
      a {
         href: "/learn/0.5/getting_started",
         data_dioxus_id: "216",
         dioxus_prevent_default: "onclick",
         class: "bg-red-600 py-2 px-8 hover:-translate-y-2 transition-transform duration-300",
         "Quickstart"
      }
      a {
         dioxus_prevent_default: "onclick",
         href: "/learn/0.5/reference",
         data_dioxus_id: "214",
         class: "bg-blue-500 py-2 px-8 hover:-translate-y-2 transition-transform duration-300",
         "Read the docs"
      }
   }
   div { class: "max-w-screen-2xl mx-auto pt-36",
      h1 { class: "text-md", "Trusted by top companies" }
      div { class: "pt-4 flex flex-row flex-wrap justify-center",
         div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
            img { src: "static/futurewei_bw.png" }
         }
         div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
            img { src: "static/airbuslogo.svg" }
         }
         div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
            img { src: "static/ESA_logo.svg" }
         }
         div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
            img { src: "static/yclogo.svg" }
         }
         div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
            img { src: "static/satellite.webp" }
         }
      }
   }
}
```

## Usage

The `dx translate` command has several flags you can use to control your html input and rsx output.

You can use the `--file` flag to translate an HTML file to RSX:

```sh
dx translate --file index.html
```

Or you can use the `--raw` flag to translate a string of HTML to RSX:

```sh
dx translate --raw "<div>Hello world</div>"
```

Both of those commands will output the following RSX:

```rs
div { "Hello world" }
```

The `dx translate` command will output the RSX to stdout. You can use the `--output` flag to write the RSX to a file instead.

```sh
dx translate --raw "<div>Hello world</div>" --output index.rs
```

You can automatically create a component with the `--component` flag.

```sh
dx translate --raw "<div>Hello world</div>" --component
```

This will output the following component:

```rs
fn component() -> Element {
   rsx! {
      div { "Hello world" }
   }
}
```

To learn more about the different flags `dx translate` supports, run `dx translate --help`.
