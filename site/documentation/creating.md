## Creating Sites

You need to create an `index.site` and `insite.toml` file to begin compiling sites. Inside `insite.toml`, add the following:

```toml
[build]
output = "my/output/directory/" # path to your output directory
name = "example_website" # name of your website 
```

The `insite.toml` file is configures how the Insite compiler builds a site. More information on configuring your `insite.toml` can be found [here](/insite/docs/configuring).

### The Syntax

The `index.site` file is where a site is described, in code. The syntax is much similar to that of the Assembly programming language, where programs are broken down into individual lines.

```asm
; This is a comment
"This is a string"

; Some commands
md "Example text"
jmp my_routine
```

### Pages

An `index.site` can contain multiple `page` definitions. `page` definitions are used for creating individual pages in a website, and are given a name and path.

```asm
.page "This is the title", "my/page/path":
```

- `.page` tells the compiler a page is being defined
- `"This is the title"` is the title of a page (string)
- `"my/page/path"` the path of the page

*The page path isn't a filename, rather it is the physical URL path of the page. For example, the home page of a site would have a path of just `/`. To access it, the user would navigate to `https://www.mysite.com/`*

### Sections

An `index.site` file can contain multiple `section` definitions, too. `section` definitions are re-usable routines that can be referenced via `jmp` lines.

```asm
.section my_routine:
    md "This is some text."

.page "This is the title", "my/page/path":
    jmp my_routine
```

- `.section` tells the compiler a routine is being defined
- `my_routine` is the name of the routing, a variable

Every time a `jmp <routine name>` line is seen, the compiler briefly jumps to that routine definition, executing it as if it where apart of the line itself.

### Using Markdown

Insite maintains first-class support for markdown with the `md` and `mdf` commands.

```asm
.page "Markdown Page", "/":
    md "# This string can contain any markdown contents!"
    mdf "path/to/a/markdown/file.md"
```

`md` and `mdf` both evaluate during parse time, meaning the compiler only sees the parsed markdown contents. `md` takes a string of markdown, whilst `mdf` takes a path to a markdown file. Insite will throw an error if `mdf` references an invalid markdown file, or the file doesn't exist.

### Layouts

Both sections and pages can contain layout definitions, via the `ld` and `str` keywords.

```asm
.page "Example", "/":
    ld vertical, center
    md "This text is above the last line, and centered horizontally."
    md "This text is below the first line, and centered horizontally."
    str
```

`ld` tells the compiler we are loading a layout definition. `ld` takes two arguments, layout and alignment. Layout can either be `vertical` or `horizontal`. Alignment can either be `left`, `center`, or `right`.

`str` tells the compiler we are storing a layout. Without a `str` line, the layout propagates all downward lines. `str` comes after any lines the layout should effect.

*If a site layout appears incorrect, make sure every `ld` call has a `str` below it's content.*

### Expander

Layouts can also be stretched or "spread out" with `exp`.

```asm
.page "Example", "/":
    ld horizontal, center
    md "This text is all the way to the left."
    exp
    md "This text is all the way to the right."
    str
```

`exp` tells the compiler to expand a layout's area with blank space, stretching across the available width or height.
