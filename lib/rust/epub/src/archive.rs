use std::borrow::Cow;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::read_to_string;
use std::path::Path;
use std::path::PathBuf;
use std::ptr;

use html5ever::Attribute;
use html5ever::QualName;
use html5ever::interface::ElementFlags;
use html5ever::interface::NodeOrText;
use html5ever::interface::QuirksMode;
use html5ever::interface::TreeSink;
use html5ever::local_name;
use html5ever::tendril::StrTendril;
use html5ever::tendril::TendrilSink;
use itertools::Itertools;
use railgun::error::ResultExt;
use unicode_normalization::UnicodeNormalization;
use zip::ZipArchive;

use crate::epub::Container;
use crate::epub::FromParameterizedZip;
use crate::epub::FromZip;
use crate::epub::Package;
use crate::epub::v2::ncx::Ncx;
use crate::error::IoErrorContext;
use crate::error::OtherContext;
use crate::error::Result;

#[derive(Debug)]
pub enum EpubNode {
    Chapter(String),
    Header(String),
    Paragraph(String),
}

#[derive(Debug)]
pub struct EpubArchive {
    // path: PathBuf,
    // zip: RefCell<ZipArchive<File>>,
    // container: Container,
    pub package: Package,
    pub rendered: String,
    pub chapters: Vec<(String, Vec<EpubNode>)>,
}

impl EpubArchive {
    pub fn opennew<P>(path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let file = File::open(path).context(IoErrorContext {})?;

        let mut zip = ZipArchive::new(BufReader::new(file))
            .boxed_local()
            .context(OtherContext {})?;

        let container = Self::parse::<Container>(&mut zip)
            .boxed_local()
            .context(OtherContext {})?;

        println!("{:#?}", container.package(&mut zip));

        Ok(())
    }

    pub fn open<P>(path: P) -> Result<EpubArchive>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let file = File::open(path).context(IoErrorContext {})?;

        let mut zip = ZipArchive::new(BufReader::new(file))
            .boxed_local()
            .context(OtherContext {})?;

        let container = Self::parse::<Container>(&mut zip)?;

        let x = container.package(&mut zip)?;

        let package =
            Self::parse_with::<Package>(&mut zip, &container.rootfiles.rootfile[0].full_path)?;

        // println!("{:#?}", package);

        /*
        let nav = package
            .manifest
            .items
            .iter()
            .find(|it| it.properties == Some("nav".to_owned()));

        if nav.is_none() {
            println!("{:#?}", package);
            panic!("No nav for epub {:?}", path);
        }
        */

        let manifest_html_files: HashMap<String, (String, String)> = package
            .manifest
            .items
            .iter()
            .filter(|&item| (item.media_type == "application/xhtml+xml"))
            .map(|item| (item.id.clone(), item.href.clone()))
            .map(|(id, path)| {
                let fullpath = &container.rootfiles.rootfile[0].full_path;
                let base = PathBuf::from(fullpath);
                let base = base.parent().unwrap();

                let cloned = path.clone();
                read_to_string(&mut zip.by_name(&base.join(&path).to_str().unwrap())?)
                    .map(|content| (id, (path, content)))
                    .with_context(|_| {
                        // println!("{:#?} from {:?}", base.join(&cloned), superpath);
                        IoErrorContext {}
                    })
            })
            .collect::<Result<HashMap<_, _>>>()?;

        let spine = package
            .spine
            .itemrefs
            .iter()
            .map(|item| manifest_html_files.get(&item.idref).unwrap())
            .collect::<Vec<_>>();

        let mut spine_walker = &*spine;

        let ncx = Self::parse_with::<Ncx>(&mut zip, "toc.ncx");

        let chapters = ncx
            .map(|mut ncx| {
                let ncx_toc =
                    ncx.navmap.nav_points.iter().enumerate().find(|(i, it)| {
                        it.nav_label.text == "目次" || it.content.src.contains("toc")
                    });
                if let Some((i, toc)) = ncx_toc {
                    ncx.navmap.nav_points = ncx
                        .navmap
                        .nav_points
                        .into_iter()
                        .skip(i + 1)
                        .collect::<Vec<_>>();
                }

                ncx.navmap
                    .nav_points
                    .into_iter()
                    .map(|nav_point| {
                        let src = nav_point.content.src.split('#').nth(0).unwrap();

                        (nav_point.nav_label.text, src.to_owned())
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .circular_tuple_windows()
                    .map(|(a, b)| {
                        let spine_pos_one = spine_walker.iter().position(|it| a.1 == it.0);
                        let spine_pos_two = spine_walker.iter().position(|it| b.1 == it.0);

                        (
                            a.0,
                            match (spine_pos_one, spine_pos_two) {
                                (Some(one), Some(two)) => {
                                    let items = &spine_walker[one..two];
                                    spine_walker = &spine_walker[two..];
                                    items
                                }
                                (Some(one), None) => {
                                    let items = &spine_walker[one..];
                                    spine_walker = &[];
                                    items
                                }
                                _ => {
                                    panic!("Invalid spine");
                                }
                            },
                        )
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|(title, pages)| {
                        let mut text = vec![];

                        for (_, html) in pages {
                            let arena = typed_arena::Arena::new();
                            let sink = Sink {
                                arena: &arena,
                                document: arena.alloc(Node::new(NodeData::Document)),
                                quirks_mode: Cell::new(QuirksMode::NoQuirks),
                            };

                            let html = html5ever::parse_document(sink, Default::default())
                                .from_utf8()
                                .one(html.as_bytes());

                            Self::html_to_node(&mut text, html, false, false);
                        }

                        (title, text)
                    })
                    .collect::<Vec<_>>()
            })
            .ok();

        let start_ch = package
            .guide
            .as_ref()
            .unwrap()
            .references
            .iter()
            .find(|reference| reference.r#type == "text")
            .map(|reference| {
                println!("5");
                let url = &reference.href;
                url.split('#').nth(0).unwrap().to_owned()
            })
            .unwrap_or_else(|| {
                let result = spine.iter().enumerate().find(|it| it.1.0.contains("toc"));

                result
                    .map(|(a, _)| spine.get(a + 1).unwrap())
                    .map(|(a, b)| a.clone())
                    .unwrap_or_else(|| {
                        let url = &package
                            .guide
                            .as_ref()
                            .unwrap()
                            .references
                            .first()
                            .unwrap()
                            .href;
                        url.split('#').nth(0).unwrap().to_owned()
                    })
            });

        let start = spine
            .iter()
            .position(|item| {
                println!("{:#?}", item.0);

                item.0 == start_ch
            })
            .unwrap();
        let spine = &spine[start..];
        let ordered = spine.iter().map(|item| item.1.clone()).collect::<Vec<_>>();

        let rendered = ordered
            .iter()
            .map(|html| {
                let arena = typed_arena::Arena::new();
                let sink = Sink {
                    arena: &arena,
                    document: arena.alloc(Node::new(NodeData::Document)),
                    quirks_mode: Cell::new(QuirksMode::NoQuirks),
                };

                let html = html5ever::parse_document(sink, Default::default())
                    .from_utf8()
                    .one(html.as_bytes());

                let mut text = String::new();
                Self::html_to_text(&mut text, html, false, false);

                text
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let epub = Self {
            package,
            rendered,
            chapters: chapters.unwrap_or(vec![]),
        };

        /*
        let mut epub = Self {
            path,
            zip: RefCell::new(zip),
            package,
            container,
        };

        epub.check_support()?;

        Ok(epub)
         */

        Ok(epub)
    }

    fn html_to_node(text: &mut Vec<EpubNode>, node: &Node, text_mode: bool, inline_text: bool) {
        match &node.data {
            NodeData::Document => {
                for document_node in node {
                    if let NodeData::Element {
                        name:
                            QualName {
                                local: local_name!("html"),
                                ..
                            },
                        ..
                    } = document_node.data
                    {
                        Self::html_to_node(text, document_node, false, false);
                    }
                }
            }
            NodeData::Element {
                name,
                attrs,
                template_contents,
                mathml_annotation_xml_integration_point,
            } => {
                for element_node in node {
                    if let NodeData::Element { name, .. } = &element_node.data {
                        match name.local {
                            local_name!("h1") => {
                                let mut title = String::new();
                                Self::html_to_text(&mut title, element_node, true, false);

                                text.push(EpubNode::Chapter(title));
                            }
                            local_name!("h2") | local_name!("h3") => {
                                let mut title = String::new();
                                Self::html_to_text(&mut title, element_node, true, false);

                                text.push(EpubNode::Header(title));
                            }
                            local_name!("p") => {
                                let mut paragraph = String::new();
                                Self::html_to_text(&mut paragraph, element_node, true, false);
                                text.push(EpubNode::Paragraph(paragraph));
                            }
                            local_name!("span") => {
                                Self::html_to_node(text, element_node, true, true);
                            }
                            local_name!("ruby") => {
                                // We only care about the outer part of the ruby node. The inner
                                // part is the furigana
                                if let Some(inner) = element_node.first_child.get() {
                                    Self::html_to_node(text, inner, true, true);
                                }
                            }
                            local_name!("body") | local_name!("div") | local_name!("a") => {
                                Self::html_to_node(text, element_node, text_mode, inline_text);
                            }
                            local_name!("head")
                            | local_name!("br")
                            | local_name!("svg")
                            | local_name!("hr")
                            | local_name!("table")
                            | local_name!("img") => {}
                            _ => {
                                // Self::html_to_node(text, element_node, text_mode, inline_text);
                                todo!("Element not implemented {:#?}", name);
                            }
                        }
                    } else {
                        Self::html_to_node(text, element_node, text_mode, inline_text);
                    }
                }
            }
            NodeData::Text { contents } => {
                if text_mode {
                    let mut result = String::new();

                    result.push_str(&contents.borrow().to_string());

                    if let Some(NodeData::Element { name, .. }) =
                        node.next_sibling.get().map(|it| &it.data)
                    {
                        if matches!(name.local, local_name!("span") | local_name!("ruby")) {
                            return;
                        }
                    }

                    if !inline_text {
                        // result.push_str("\n\n");
                    }

                    text.push(EpubNode::Paragraph(result));
                }
            }

            NodeData::Doctype {
                name,
                public_id,
                system_id,
            } => todo!("a"),
            NodeData::Comment { contents } => todo!("b"),
            NodeData::ProcessingInstruction { target, contents } => todo!("d"),
        }
    }

    fn html_to_text(text: &mut String, node: &Node, text_mode: bool, inline_text: bool) {
        match &node.data {
            NodeData::Document => {
                for document_node in node {
                    if let NodeData::Element {
                        name:
                            QualName {
                                local: local_name!("html"),
                                ..
                            },
                        ..
                    } = document_node.data
                    {
                        Self::html_to_text(text, document_node, false, false);
                    }
                }
            }
            NodeData::Text { contents } => {
                if text_mode {
                    let normalized = &contents.borrow().to_string().nfkc().collect::<String>();

                    text.push_str(normalized);

                    if let Some(NodeData::Element { name, .. }) =
                        node.next_sibling.get().map(|it| &it.data)
                    {
                        if matches!(name.local, local_name!("span") | local_name!("ruby")) {
                            return;
                        }
                    }

                    if !inline_text {
                        // text.push_str("\n\n");
                    }
                }
            }
            NodeData::Element {
                name,
                attrs,
                template_contents,
                mathml_annotation_xml_integration_point,
            } => {
                for element_node in node {
                    if let NodeData::Element { name, .. } = &element_node.data {
                        match name.local {
                            local_name!("h1") => {
                                Self::html_to_text(text, element_node, true, false);
                            }
                            local_name!("p") => {
                                Self::html_to_text(text, element_node, true, false);
                            }
                            local_name!("span") => {
                                Self::html_to_text(text, element_node, true, true);
                            }
                            local_name!("ruby") => {
                                // We only care about the outer part of the ruby node. The inner
                                // part is the furigana
                                if let Some(inner) = element_node.first_child.get() {
                                    Self::html_to_text(text, inner, true, true);
                                }
                            }
                            _ => {
                                Self::html_to_text(text, element_node, text_mode, inline_text);
                                // todo!("{:#?}", name);
                            }
                        }
                    } else {
                        Self::html_to_text(text, element_node, text_mode, inline_text);
                    }
                }
            }
            NodeData::Doctype {
                name,
                public_id,
                system_id,
            } => todo!("a"),
            NodeData::Comment { contents } => {
                // noop
            }
            NodeData::ProcessingInstruction { target, contents } => todo!("d"),
        }
    }

    pub(crate) fn parse<'a, TContainer>(
        zip: &'a mut ZipArchive<impl Read + Seek>,
    ) -> Result<TContainer::Type>
    where
        TContainer: FromZip<'a>,
    {
        TContainer::parse(TContainer::read(zip)?)
    }

    pub(crate) fn parse_with<'a, TContainer>(
        zip: &'a mut ZipArchive<impl Read + Seek>,
        data: TContainer::Params,
    ) -> Result<TContainer::Type>
    where
        TContainer: FromParameterizedZip<'a>,
    {
        TContainer::parse(zip, data)
    }

    /*
    fn get_rootfile(&self) -> Result<RootFile, EpubError> {
        let rootfile = self.container.rootfiles.rootfile.first();

        if let Some(file) = rootfile {
            if file.media_type != "application/oebps-package+xml" {
                return Err(EpubError::Other(eyre!("Invalid rootfile media-type")));
            }

            Ok(file.clone())
        } else {
            Err(EpubError::Other(eyre!("No rootfile")))
        }
    }

    fn check_support(&mut self) -> Result<(), EpubError> {
        if self
            .zip
            .get_mut()
            .by_name("META-INF/encryption.xml")
            .is_ok()
        {
            return Err(EpubError::Unsupported(UnsupportedReason::Encrypted));
        }

        Ok(())
    }

    pub fn content(&mut self) -> Content<'_> {
        Content::new(self)
    }
    */
}

/*
#[derive(Debug)]
pub struct Content<'archive> {
    archive: &'archive EpubArchive,
    head: String,
    spine_entry: usize,
}

impl<'archive> Content<'archive> {
    fn new(archive: &'archive EpubArchive) -> Self {
        // TODO: fix expect
        let entry = if let Some(guide) = &archive.package.guide {
            guide
                .references
                .iter()
                .find(|item| item.r#type == "text")
                .expect("No text entrypoint")
        } else {
            // TODO: Fix
            panic!("Missing guide")
        };

        let url = Url::parse(&entry.href);
        let url = if let Err(ParseError::RelativeUrlWithoutBase) = url {
            Url::parse(&format!("http://example.com/{}", &entry.href)).unwrap()
        } else {
            url.unwrap()
        };
        let url = url.path();

        let manifest_entry = archive
            .package
            .manifest
            .items
            .iter()
            .enumerate()
            .find(|(_, item)| item.href == url || item.href == url.strip_prefix("/").unwrap())
            .expect("Missing entry")
            .1
            .id
            .clone();

        let spine_entry = archive
            .package
            .spine
            .itemrefs
            .iter()
            .enumerate()
            .find(|(_, item)| item.idref == manifest_entry);

        let index = if let Some((index, _)) = spine_entry {
            index
        } else {
            // TODO: Fix
            panic!("Missing entry")
        };

        Self {
            archive,
            head: url.to_owned(),
            spine_entry: index,
        }
    }

    pub fn read(&mut self) -> Option<String> {
        let spine_entry = self.spine_entry;
        self.spine_entry += 1;

        let reference = self.archive.package.spine.itemrefs.get(spine_entry)?;
        let href = self
            .archive
            .package
            .manifest
            .items
            .iter()
            .find(|item| item.id == reference.idref)?
            .href
            .clone();

        let mut zip = self.archive.zip.borrow_mut();
        let mut content = zip.by_name(&href).expect("Error");

        let mut data = String::new();
        content.read_to_string(&mut data).unwrap();

        let arena = typed_arena::Arena::new();

        let sink = Sink {
            arena: &arena,
            document: arena.alloc(Node::new(NodeData::Document)),
            quirks_mode: QuirksMode::NoQuirks,
        };

        let html = html5ever::parse_document(sink, Default::default())
            .from_utf8()
            .one(data.as_bytes());

        let mut text = "".to_owned();
        let result = html_to_text(&mut text, html, false);

        // TODO: Read into html

        Some(text)
    }
}

fn html_to_text(text: &mut String, node: &Node, text_mode: bool) {
    match &node.data {
        NodeData::Document => {
            for document_node in node {
                if let NodeData::Element {
                    name:
                        QualName {
                            local: local_name!("html"),
                            ..
                        },
                    ..
                } = document_node.data
                {
                    html_to_text(text, document_node, false);
                }
            }
        },
        NodeData::Doctype {
            name,
            public_id,
            system_id,
        } => todo!(),
        NodeData::Text { contents } => {
            if text_mode {
                text.push_str(&contents.borrow().to_string())
            }
        },
        NodeData::Comment { contents } => todo!(),
        NodeData::Element {
            name,
            attrs,
            template_contents,
            mathml_annotation_xml_integration_point,
        } => {
            for element_node in node {
                if let NodeData::Element { name, .. } = &element_node.data {
                    match name.local {
                        local_name!("p") => {
                            html_to_text(text, element_node, true);
                            // text.push_str("p");
                        },
                        local_name!("ruby") => {
                            // We only care about the outer part of the ruby node. The inner is the
                            // furigana.
                            if let Some(inner) = element_node.first_child.get() {
                                html_to_text(text, inner, true);
                            }
                        },
                        _ => html_to_text(text, element_node, false),
                    }
                } else {
                    html_to_text(text, element_node, text_mode);
                }
            }
        },
        NodeData::ProcessingInstruction { target, contents } => todo!(),
    };
}
*/
type Arena<'arena> = &'arena typed_arena::Arena<Node<'arena>>;
type Ref<'arena> = &'arena Node<'arena>;
type Link<'arena> = Cell<Option<Ref<'arena>>>;

/// Sink struct is responsible for handling how the data that comes out of the
/// HTML parsing unit (TreeBuilder in our case) is handled.
struct Sink<'arena> {
    arena: Arena<'arena>,
    document: Ref<'arena>,
    quirks_mode: Cell<QuirksMode>,
}

/// DOM node which contains links to other nodes in the tree.
#[derive(Debug)]
pub struct Node<'arena> {
    parent: Link<'arena>,
    next_sibling: Link<'arena>,
    previous_sibling: Link<'arena>,
    first_child: Link<'arena>,
    last_child: Link<'arena>,
    data: NodeData<'arena>,
    iter_count: Cell<usize>,
}

impl<'arena> Iterator for &Node<'arena> {
    type Item = Ref<'arena>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_count.get() == 0 {
            self.iter_count.set(self.iter_count.take() + 1);

            return self.first_child.get();
        }

        self.iter_count.set(self.iter_count.take() + 1);
        let mut child = self.first_child.get();

        for _ in 1..self.iter_count.get() {
            if let Some(inner) = child {
                child = inner.next_sibling.get();
            }
        }

        child
    }
}

/// HTML node data which can be an element, a comment, a string, a DOCTYPE,
/// etc...
#[derive(Debug)]
pub enum NodeData<'arena> {
    Document,
    Doctype {
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    },
    Text {
        contents: RefCell<StrTendril>,
    },
    Comment {
        contents: StrTendril,
    },
    Element {
        name: QualName,
        attrs: RefCell<Vec<Attribute>>,
        template_contents: Option<Ref<'arena>>,
        mathml_annotation_xml_integration_point: bool,
    },
    ProcessingInstruction {
        target: StrTendril,
        contents: StrTendril,
    },
}

impl<'arena> Node<'arena> {
    fn new(data: NodeData<'arena>) -> Self {
        Node {
            parent: Cell::new(None),
            previous_sibling: Cell::new(None),
            next_sibling: Cell::new(None),
            first_child: Cell::new(None),
            last_child: Cell::new(None),
            data,
            iter_count: Cell::new(0),
        }
    }

    fn detach(&self) {
        let parent = self.parent.take();
        let previous_sibling = self.previous_sibling.take();
        let next_sibling = self.next_sibling.take();

        if let Some(next_sibling) = next_sibling {
            next_sibling.previous_sibling.set(previous_sibling);
        } else if let Some(parent) = parent {
            parent.last_child.set(previous_sibling);
        }

        if let Some(previous_sibling) = previous_sibling {
            previous_sibling.next_sibling.set(next_sibling);
        } else if let Some(parent) = parent {
            parent.first_child.set(next_sibling);
        }
    }

    fn append(&'arena self, new_child: &'arena Self) {
        new_child.detach();
        new_child.parent.set(Some(self));
        if let Some(last_child) = self.last_child.take() {
            new_child.previous_sibling.set(Some(last_child));
            debug_assert!(last_child.next_sibling.get().is_none());
            last_child.next_sibling.set(Some(new_child));
        } else {
            debug_assert!(self.first_child.get().is_none());
            self.first_child.set(Some(new_child));
        }
        self.last_child.set(Some(new_child));
    }

    fn insert_before(&'arena self, new_sibling: &'arena Self) {
        new_sibling.detach();
        new_sibling.parent.set(self.parent.get());
        new_sibling.next_sibling.set(Some(self));
        if let Some(previous_sibling) = self.previous_sibling.take() {
            new_sibling.previous_sibling.set(Some(previous_sibling));
            debug_assert!(ptr::eq::<Node>(
                previous_sibling.next_sibling.get().unwrap(),
                self
            ));
            previous_sibling.next_sibling.set(Some(new_sibling));
        } else if let Some(parent) = self.parent.get() {
            debug_assert!(ptr::eq::<Node>(parent.first_child.get().unwrap(), self));
            parent.first_child.set(Some(new_sibling));
        }
        self.previous_sibling.set(Some(new_sibling));
    }
}

impl<'arena> Sink<'arena> {
    fn new_node(&self, data: NodeData<'arena>) -> Ref<'arena> {
        self.arena.alloc(Node::new(data))
    }

    fn append_common<P, A>(&self, child: NodeOrText<Ref<'arena>>, previous: P, append: A)
    where
        P: FnOnce() -> Option<Ref<'arena>>,
        A: FnOnce(Ref<'arena>),
    {
        let new_node = match child {
            NodeOrText::AppendText(text) => {
                // Append to an existing Text node if we have one.
                if let Some(&Node {
                    data: NodeData::Text { ref contents },
                    ..
                }) = previous()
                {
                    contents.borrow_mut().push_tendril(&text);
                    return;
                }
                self.new_node(NodeData::Text {
                    contents: RefCell::new(text),
                })
            }
            NodeOrText::AppendNode(node) => node,
        };

        append(new_node);
    }
}

/// By implementing the TreeSink trait we determine how the data from the tree
/// building step is processed. In our case, our data is allocated in the arena
/// and added to the Node data structure.
///
/// For deeper understating of each function go to the TreeSink declaration.
impl<'arena> TreeSink for Sink<'arena> {
    type ElemName<'a>
        = &'a QualName
    where
        Self: 'a;
    type Handle = Ref<'arena>;
    type Output = Ref<'arena>;

    fn finish(self) -> Ref<'arena> {
        self.document
    }

    fn parse_error(&self, _: Cow<'static, str>) {}

    fn get_document(&self) -> Ref<'arena> {
        self.document
    }

    fn set_quirks_mode(&self, mode: QuirksMode) {
        self.quirks_mode.set(mode);
    }

    fn same_node(&self, x: &Ref<'arena>, y: &Ref<'arena>) -> bool {
        ptr::eq::<Node>(*x, *y)
    }

    fn elem_name<'a>(&self, target: &'a Ref<'arena>) -> Self::ElemName<'a> {
        match target.data {
            NodeData::Element { ref name, .. } => name,
            _ => panic!("not an element!"),
        }
    }

    fn get_template_contents(&self, target: &Ref<'arena>) -> Ref<'arena> {
        if let NodeData::Element {
            template_contents: Some(contents),
            ..
        } = target.data
        {
            contents
        } else {
            panic!("not a template element!")
        }
    }

    fn is_mathml_annotation_xml_integration_point(&self, target: &Ref<'arena>) -> bool {
        if let NodeData::Element {
            mathml_annotation_xml_integration_point,
            ..
        } = target.data
        {
            mathml_annotation_xml_integration_point
        } else {
            panic!("not an element!")
        }
    }

    fn create_element(
        &self,
        name: QualName,
        attrs: Vec<Attribute>,
        flags: ElementFlags,
    ) -> Ref<'arena> {
        self.new_node(NodeData::Element {
            name,
            attrs: RefCell::new(attrs),
            template_contents: flags.template.then(|| self.new_node(NodeData::Document)),
            mathml_annotation_xml_integration_point: flags.mathml_annotation_xml_integration_point,
        })
    }

    fn create_comment(&self, text: StrTendril) -> Ref<'arena> {
        self.new_node(NodeData::Comment { contents: text })
    }

    fn create_pi(&self, target: StrTendril, data: StrTendril) -> Ref<'arena> {
        self.new_node(NodeData::ProcessingInstruction {
            target,
            contents: data,
        })
    }

    fn append(&self, parent: &Ref<'arena>, child: NodeOrText<Ref<'arena>>) {
        self.append_common(
            child,
            || parent.last_child.get(),
            |new_node| parent.append(new_node),
        );
    }

    fn append_before_sibling(&self, sibling: &Ref<'arena>, child: NodeOrText<Ref<'arena>>) {
        self.append_common(
            child,
            || sibling.previous_sibling.get(),
            |new_node| sibling.insert_before(new_node),
        );
    }

    fn append_based_on_parent_node(
        &self,
        element: &Ref<'arena>,
        prev_element: &Ref<'arena>,
        child: NodeOrText<Ref<'arena>>,
    ) {
        if element.parent.get().is_some() {
            self.append_before_sibling(element, child);
        } else {
            self.append(prev_element, child);
        }
    }

    fn append_doctype_to_document(
        &self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        self.document.append(self.new_node(NodeData::Doctype {
            name,
            public_id,
            system_id,
        }));
    }

    fn add_attrs_if_missing(&self, target: &Ref<'arena>, attrs: Vec<Attribute>) {
        let mut existing = if let NodeData::Element { ref attrs, .. } = target.data {
            attrs.borrow_mut()
        } else {
            panic!("not an element")
        };

        let existing_names = existing
            .iter()
            .map(|e| e.name.clone())
            .collect::<HashSet<_>>();
        existing.extend(
            attrs
                .into_iter()
                .filter(|attr| !existing_names.contains(&attr.name)),
        );
    }

    fn remove_from_parent(&self, target: &Ref<'arena>) {
        target.detach();
    }

    fn reparent_children(&self, node: &Ref<'arena>, new_parent: &Ref<'arena>) {
        let mut next_child = node.first_child.get();
        while let Some(child) = next_child {
            debug_assert!(ptr::eq::<Node>(child.parent.get().unwrap(), *node));
            next_child = child.next_sibling.get();
            new_parent.append(child);
        }
    }
}
