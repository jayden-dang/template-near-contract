---
author: Dang Quang Vu
header-includes:
- "`<style>pre.src{background:#343131;color:white;} </style>`{=html}"
title: Started Template - Near Smartcontract
---

`<style>pre.src{background:#343131;color:white;} </style>`{=html}

```{=org}
#+setupfile: ~/theme-readtheorg.setup
```
-   Quickly build apps backed by the NEAR blockchain
-   Prerequisites

> Make sure you have a current version of Node.js & Rust installed =\>
> We recommend versions 18+ & rust 1.69

# Getting Started

## Clone the repository

``` {.bash org-language="sh"}
git clone https://github.com/eamondang/started-near-app.git && cd started-near-app
```

## Requirements Install

-   Cargo Make

``` {.bash org-language="sh"}
cargo install cargo-make
```

-   Install near Cli

``` {.bash org-language="sh"}
npm install -g near-cli
```

# Step by Step

-   Prepare

``` {.bash org-language="sh"}
cargo make prepare
```

-   Build Contract

``` {.bash org-language="sh"}
cargo make build
```

-   deploy Contract

``` {.bash org-language="sh"}
cargo make dev-deploy
```
