/// A `Page` ID.
pub struct PageID(pub usize);

/// A collection of `Page` objects.
pub struct PageCollection {
    pages: Vec<Page>,
}

impl PageCollection {
    /// Constructs a new `PageCollection` object.
    pub fn new() -> Self {
        Self { pages: Vec::new() }
    }

    /// Add a `Page` to the collection.
    ///
    /// - `page`: the page to add
    ///
    /// This function returns the corresponding `PageID` for the appended `page`.
    pub fn add_page(&mut self, page: Page) -> PageID {
        self.pages.push(page);

        PageID(self.pages.len() - 1)
    }

    /// Retrieve a `Page` in the collection.
    ///
    /// - `id`: the `PageID` of the page
    ///
    /// This function will return `None` if the `id` isn't inside the collection.
    pub fn get_page(&self, id: &PageID) -> Option<&Page> {
        self.pages.get(id.0)
    }

    /// Retrieve a mutable `Page` in the collection.
    ///
    /// - `id`: the `PageID` of the page
    ///
    /// This function will return `None` if the `id` isn't inside the collection.
    pub fn get_page_mut(&mut self, id: &PageID) -> Option<&mut Page> {
        self.pages.get_mut(id.0)
    }

    /// Returns the `Page` objects contained inside the collection.
    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}

/// An HTML page.
pub struct Page {
    name: String,
    path: String,
    contents: String,
}

impl Page {
    /// Constructs a new `Page` object.
    ///
    /// - `name`: the name of the page
    /// - `path`: the path of the page
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_owned(),
            path: path.to_owned(),
            contents: String::new(),
        }
    }

    /// Write an HTML string onto the page.
    ///
    /// - `html`: the HTML to write onto the page
    pub fn write(&mut self, html: &str) {
        self.contents.push_str(html);
    }

    /// Returns the name of the page.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the path of the page.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the HTML contents of the page.
    pub fn contents(&self) -> &str {
        &self.contents
    }
}
