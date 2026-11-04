use std::{
    collections::HashMap,
    fs,
    path::{Component, Path, PathBuf},
};

use crate::{
    compiler::{Page, PageCollection, PageID},
    config::BuildConfig,
    error::{Error, LanguageError},
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

    // Explicit HTML filenames remain files.
    if relative.extension().is_some_and(|ext| ext == "html") {
        return Some(relative);
    }

    // All other paths are routes, so produce an index.html.
    relative.push("index.html");

    Some(relative)
}

/// An Insite Compiler.
pub struct Compiler {
    symbols: HashMap<String, Vec<Command>>,
    page_collection: PageCollection,
    config: BuildConfig,
    dev_mode: bool,
}

impl Compiler {
    /// Constructs a new `Compiler` object.
    ///
    /// - `config`: the build configuration
    /// - `dev_mode`: enable dev mode
    ///
    /// If `dev_mode` is `true`, the compiler will inject reloading scripts into the output HTML,
    /// otherwise the output is generated as normal.
    pub fn new(config: &BuildConfig, dev_mode: bool) -> Self {
        Self {
            symbols: HashMap::new(),
            page_collection: PageCollection::new(),
            config: config.to_owned(),
            dev_mode,
        }
    }

    /// Build a web page from the commands.
    ///
    /// - `commands`: the commands to parse and build the site from
    ///
    /// # Errors
    ///
    /// This function will `Err` if the any `Command` inside of `commands` was unable to be read correctly.
    pub fn build(&mut self, commands: Vec<Command>) -> Result<(), Error> {
        if self.config.purge && self.config.output.exists() {
            // clear the output directory
            fs::remove_dir_all(&self.config.output)?;
        }

        // create the output directory
        fs::create_dir(&self.config.output)?;

        if let Some(s) = &self.config.stylesheet
            && let Err(_) = fs::copy(&s, &self.config.output.join("style.css"))
        {
            println!("insite: unable to bundle stylesheet '{}'", s.display());
        }

        if let Some(f) = &self.config.favicon
            && let Err(_) = fs::copy(&f, &self.config.output.join(f.file_name().unwrap()))
        {
            println!("insite: unable to bundle favicon '{}'", f.display());
        }

        for f in &self.config.assets {
            if let Err(_) = fs::copy(&f, &self.config.output.join(&f.file_name().unwrap())) {
                println!("insite: unable to bundle asset '{}'", f.display());
            }
        }

        for command in commands {
            self.execute_command(command, &PageID(0))?;
        }

        for page in self.page_collection.pages() {
            let Some(relative_path) = output_path(page.path()) else {
                println!("insite: skipping unsafe output path '{}'", page.path());
                continue;
            };

            let path = self.config.output.join(relative_path);

            if let Some(parent) = path.parent()
                && let Err(_) = fs::create_dir_all(parent)
            {
                println!("insite: unable to create directory '{}'", parent.display());
                continue;
            }

            if let Err(_) = fs::write(&path, page.contents()) {
                println!("insite: unable to write '{}'", path.display());
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
                    return Err(Error::Language(LanguageError::new(
                        location,
                        &format!("no section defined for '{}'", &section),
                    )));
                }
            }
            Command::Markdown { html, location } => {
                if let Some(p) = self.page_collection.get_page_mut(page) {
                    // markdown is treated as a vertically stack section of text
                    // always put inside a vertical layout unless specified otherwise
                    p.write(&if self.config.allow_markdown_layout {
                        html
                    } else {
                        format!(
                            "<div style=\"display: flex; flex-direction: column;\">{}</div>",
                            html
                        )
                    });
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

        let site_name = if self.config.allow_path_appending && !self.dev_mode {
            format!("/{}", self.config.name)
        } else {
            String::new()
        };

        // html header
        let favicon = self
            .config
            .favicon
            .as_ref()
            .map_or_else(String::new, |icon| {
                format!(
                    r#"<link rel="icon" type="image/x-icon" href="{site_name}/{}">"#,
                    icon.file_name().unwrap().display()
                )
            });
        let stylesheet = format!(r#"<link rel="stylesheet" href="{site_name}/style.css">"#);
        let dev_script = if self.dev_mode {
            r#"<script>
                (() => {
                    const socket = new WebSocket(`ws://${location.host}/__insite_ws`);

                    socket.addEventListener("message", (event) => {
                        if (event.data === "reload") {
                            location.reload();
                        }
                    });
                })();
            </script>"#
        } else {
            ""
        };

        p.write(&format!(
            r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta http-equiv="X-UA-Compatible" content="ie=edge"><title>{name}</title>{stylesheet}{favicon}</head>{dev_script}<body><main>"#
        ));

        for child in children {
            self.execute_command(child, &id)?;
        }

        let p = self.page_collection.get_page_mut(&id).unwrap();
        p.write("</main></body></html>");

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
