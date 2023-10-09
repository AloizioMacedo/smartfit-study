# Introduction

This is a mini-project intended as self-study for HTMX + Askama + Axum.

It uses as motivation a
[coding challenge](https://github.com/bioritmo/front-end-code-challenge-smartsite)
from a brazilian fitness company called [SmartFit](https://www.smartfit.com.br).

# Technologies

We use the templating engine [Askama](https://github.com/djc/askama) for Rust in order
to build the HTML together with [HTMX](https://htmx.org/) when we update the page.
The whole project runs as a single service running [Axum](https://github.com/tokio-rs/axum).

If built as a Docker image, it weighs 29.5 MB.

# How to run

You can run the project directly with

```bash
cargo run
```

in the root of the repo in case you have [Rust](https://www.rust-lang.org/) installed.

If you don't, you can build it as a Docker image and run it directly.

The app gets served at port 3000, and the page can be accessed at "/".

PS: In case you run it as a Docker container, remember to set the ports, e.g.
`docker run -p 3000:3000 {name}`.
