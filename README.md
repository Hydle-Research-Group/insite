<img src="assets/icon-wide.svg" width="25%">
    
Insite is an assembly-like programming language for building static websites. 

```asm
; define reusable routines
.section navigation_bar:
    ld horizontal, center ; load a layout
    md "# This is a website!" ; first-class markdown support
    exp
    md "[Go Home](/)"
    str ; store a layout

; define individual pages
.page "Home", "/" ; title, path
    jmp navigation_bar
    ld vertical, center
    md "# Welcome to my home page!"
    str
```

## Why Insite?

Insite was built to give developers full control over static websites, much similar to the prophecies of lower-level languages. Rather than typing raw HTML though, Insite is built with full markdown support in mind. 

```asm
.page "Example", "/":
    md "# This is markdown"
    md "**bold text**, maybe some *italic*"
    md "- Check out this list!"
    md "> Look at my quote"

    ; compile files too!
    mdf "path/to/markdown.md"
```

When building a site with Insite, there are no templates or default configurations. Sites are pure HTML, and it is up to the user to style, layout, and modify their site. The Insite compiler is setup by the rules defined in an `insite.toml` configuration file.

```toml
# Example insite.toml file
[build]
output = "docs"

[resources] 
favicon = "assets/icon.svg"
stylesheet = "style.css"

[compiler]
allow-markdown-html = true
```

At its core, there are only two files: `insite.toml`, and `index.site`. The `index.site` file contains site code, and an `insite.toml` file configures the compiler.

## Usage

There are full instructions on how to install and use Insite in the [official documentation](https://hydle-research-group.github.io/insite/docs/). 

Documentation is actively updated according to Insite, so expect changes on the regular.

## Free & Open-Source

Insite is 100% free with no drawbacks or limitations. There is no "premium" version; you get the latest and greatest, all licensed under the GPL-3.0.

All source code is public, to anyone. There is no "hidden mechanism" included in this repository; every reference and used factor exists completely and fully.
