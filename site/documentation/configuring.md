## Configuring Sites

The way a site is built depends on the configuration defined in `insite.toml`. `insite.toml` can specify build information, asset bundling, and compiler flags.

## `build`

`build` configures a site's build settings.

```toml
[build]
# build attributes here...
```

### Attributes

#### `name` *string, default: ""*

The name of the site.

```toml
[build]
name = "my-site"
```

#### `output` *string, default: "output"*

The output directory of the site.

```toml
[build]
output = "docs"
```

#### `create-gitignore` *bool, default: true*

Create a `.gitignore` file and add `build.output` to it.

```toml
[build]
create-gitignore = false
```

## `resources`

`resources` configures a site's resources.

```toml
[resources]
# resource attributes here...
```

### Attributes

#### `favicon` *string, default: ""*

Path to the site's favicon file. *The favicon is bundled with the site by the compiler.* 

```toml
[resources]
favicon = "path/to/my/favicon.png"
```

#### `stylesheet` *string, default: ""*

Path to the site's CSS stylesheet file. *The stylesheet is bundled with the site by the compiler.* 

```toml
[resources]
stylesheet = "path/to/my/stylesheet.css"
```

#### `assets` *array, default: []*

An array of files to bundle with the site. *The files are placed in the root directory (`/`) of the site.* 

```toml
[resources]
assets = ["path/to/an/icon.png", "this/is/an/image.jpeg"]
```

## `compiler`

`compiler` configures the Insite compiler.

```toml
[compiler]
# compiler attributes here...
```

### Attributes

#### `allow-markdown-html` *bool, default: false*

Allow HTML inside of markdown content.

```toml
[compiler]
allow-markdown-html = true
```

#### `allow-markdown-layout` *bool, default: false*

Allow markdown content to follow the layout of parent elements, rather than creating it's own vertical layout (expected behaviour).

```toml
[compiler]
allow-markdown-html = true
```

#### `allow-path-appending` *bool, default: false*

Append the `build.name` attribute to assets like favicons and stylesheets. *For better GitHub pages compatibility, set this to `true`*

```toml
[compiler]
allow-path-appending = true
```

#### `allow-gfm` *bool, default: true*

Allow the use of [GitHub Flavored Markdown (GFM)](https://github.github.com/gfm/).

```toml
[compiler]
allow-gfm = false
```
