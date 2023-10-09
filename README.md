# Introduction

This is a mini-project intended as self-study for HTMX + Askama + Axum.

It uses as motivation a
[coding challenge](https://github.com/bioritmo/front-end-code-challenge-smartsite)
from a brazilian fitness company called [SmartFit](https://www.smartfit.com.br).

You can see it deployed with [Shuttle](https://www.shuttle.rs/)
here: https://aloizio-smartfit.shuttleapp.rs/.

# Technologies

We use the templating engine [Askama](https://github.com/djc/askama) for Rust in order
to build the HTML together with [HTMX](https://htmx.org/) when we update the page.
The whole project runs as a single service running [Axum](https://github.com/tokio-rs/axum).

If built as a Docker image, it weighs around 30 MB.

# How to run

You can run the project directly with

```bash
cargo run --bin smartfit
```

in the root of the repo in case you have [Rust](https://www.rust-lang.org/) installed.

If you don't, you can build it as a Docker image and run it directly.

The app gets served at port 3000, and the page can be accessed at "/".

PS: In case you run it as a Docker container, remember to set the ports, e.g.
`docker run -p 3000:3000 {name}`.

# Commentary about the challenge

It is unclear how the hours filter that the challenge proposes is supposed to work.

The interpretation that at first seems to make most sense is the following:

> If the user marked a time slot X, then an entry should be showed
> as long as X intersects any interval of hours of any schedule of that entry.

However, with this interpretation, the filter ends up being useless: any selection of
"morning", "afternoon" or "evening" gives the same entries.

The other filter (the one about showing closed units) does something, even though
it is very minor, as only three units seem to be closed in the provided file.
