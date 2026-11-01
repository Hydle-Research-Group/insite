use std::{
    collections::HashMap,
    fs,
    path::{Component, Path, PathBuf},
};

use crate::{
    compiler::{BuildConfig, Page, PageCollection, PageID},
    error::Error,
    parser::{Alignment, Command, Layout},
};

/// Convert a URL-like page path into a safe relative HTML file path.
///
/// - `path`: the page path to convert
///
/// This function will return `None` if the path components are of `..`, `../` or `C:`
fn output_path(path: &str) -> Option<PathBuf> {
    let normalized = path.trim().replace('\\', "/");
    let trimmed = normalized.trim_matches('/');

    if trimmed.is_empty() {
        return Some(PathBuf::from("index.html"));
    }

    let mut relative = PathBuf::new();

    for component in Path::new(trimmed).components() {
        match component {
            Component::Normal(part) => relative.push(part),
            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => return None,
        }
    }

    if relative.as_os_str().is_empty() {
        return Some(PathBuf::from("index.html"));
    }

    let mut result = relative;
    let is_html = result
        .extension()
        .is_some_and(|extension| extension == "html");

    if !is_html {
        result.set_extension("html");
    }

    Some(result)
}

/// An Insite Compiler.
pub struct Compiler {
    symbols: HashMap<String, Vec<Command>>,
    page_collection: PageCollection,
}

impl Compiler {
    /// Constructs a new `Compiler` object.
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            page_collection: PageCollection::new(),
        }
    }

    /// Build a web page from the commands.
    ///
    /// - `config`: the build configuration
    /// - `commands`: the commands to parse and build the site from
    ///
    /// # Errors
    ///
    /// This function will `Err` if the any `Command` inside of `commands` was unable to be read correctly.
    pub fn build(&mut self, config: BuildConfig, commands: Vec<Command>) -> Result<(), Error> {
        if let Err(e) = fs::remove_dir_all(&config.output) {
            println!("insite: {e}");
        }

        if let Err(e) = fs::create_dir(&config.output) {
            println!("insite: {e}");
        }

        if let Some(s) = config.style
            && let Err(_) = fs::copy(&s, &config.output.join("style.css"))
        {
            println!("insite: unable to link stylesheet '{}'", s.display());
        }

        for command in commands {
            self.execute_command(command, &PageID(0))?;
        }

        for page in self.page_collection.pages() {
            let path = config
                .output
                .join(if let Some(o) = output_path(page.path()) {
                    o
                } else {
                    println!("insite: skipping unsafe output path '{}'", page.path());

                    continue;
                });

            if path.ends_with("/") {
                if let Err(e) = fs::create_dir_all(&path) {
                    println!("insite: {e}");
                }
            } else {
                if let Err(e) = fs::write(
                    &path,
                    if config.pretty {
                        page.contents().to_owned()
                    } else {
                        page.contents().replace("\n", "")
                    },
                ) {
                    println!("insite: {e}");
                }
            }
        }

        Ok(())
    }

    /// Execute a command.
    ///
    /// - `command`: the command to execute
    /// - `page`: the `PageID` to write to
    ///
    /// This function will build an HTML file, written onto `page`.
    ///
    /// # Errors
    ///
    /// This function will `Err` if attempts to access undefined symbols occur.
    fn execute_command(&mut self, command: Command, page: &PageID) -> Result<(), Error> {
        match command {
            Command::Page {
                name,
                path,
                children,
                location,
            } => {
                self.page_command(&name, &path, children)?;
            }
            Command::Section {
                name,
                children,
                location,
            } => {
                self.section_command(&name, children);
            }
            Command::Load {
                layout,
                alignment,
                location: _,
            } => {
                if let Some(p) = self.page_collection.get_page_mut(page) {
                    let direction = match layout {
                        Layout::Horizontal => "row",
                        Layout::Vertical => "column",
                    };

                    let align_items = match alignment {
                        Alignment::Left => "flex-start",
                        Alignment::Center => "center",
                        Alignment::Right => "flex-end",
                    };

                    p.write(&format!(
                        concat!(
                            "<div style=\"",
                            "display: flex;",
                            "flex-direction: {};",
                            "align-items: {};",
                            "justify-content: {};",
                            "width: 100%;",
                            "box-sizing: border-box;",
                            "\">"
                        ),
                        direction, align_items, align_items
                    ));
                }
            }
            Command::Store { location } => {
                if let Some(p) = self.page_collection.get_page_mut(page) {
                    p.write("</div>");
                }
            }
            Command::Jump { section, location } => {
                if let Some(c) = self.symbols.clone().get(&section) {
                    for command in c {
                        self.execute_command(command.to_owned(), page)?;
                    }
                } else {
                    return Err(Error::new(
                        location,
                        &format!("no section defined for '{}'", &section),
                    ));
                }
            }
            Command::Markdown { text, location } => {
                if let Some(p) = self.page_collection.get_page_mut(page) {
                    p.write(&markdown::to_html(&text));
                }
            }
            Command::Expand { location } => {
                if let Some(p) = self.page_collection.get_page_mut(page) {
                    p.write("<div style=\"flex-grow: 1;\"></div>");
                }
            }
        }
        Ok(())
    }

    /// Execute a page command (`Command::Page`)
    ///
    /// - `name`: the name of the page
    /// - `path`: the path of the page
    /// - `children`: the children commands of the page
    fn page_command(
        &mut self,
        name: &str,
        path: &str,
        children: Vec<Command>,
    ) -> Result<(), Error> {
        let id = self.page_collection.add_page(Page::new(name, path));

        let p = self.page_collection.get_page_mut(&id).unwrap();

        // html header
        p.write(&format!(
            r#"<!DOCTYPE html><html lang="en"><head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<meta http-equiv="X-UA-Compatible" content="ie=edge">
<title>{}</title>
<link rel="stylesheet" href="style.css">
<link rel="icon" href="./{}.ico" type="image/x-icon">
</head>
<body>
<main>"#,
            &name, &name
        ));

        for child in children {
            self.execute_command(child, &id)?;
        }

        let p = self.page_collection.get_page_mut(&id).unwrap();
        p.write("</main>\n</body>\n</html>");

        Ok(())
    }

    /// Execute a section command (`Command::Section`)
    ///
    /// - `name`: the name of the section
    /// - `children`: the children of the section
    ///
    /// This function will insert `name` into a global symbol table with the corresponding `children`, allowing
    /// `jmp` commands to reference `name` for element reuse.
    fn section_command(&mut self, name: &str, children: Vec<Command>) {
        self.symbols.insert(name.to_string(), children);
    }
}
