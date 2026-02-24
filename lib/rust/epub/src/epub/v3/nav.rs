use std::str::FromStr;

use railgun::error::ResultExt;
use serde_xml_rs::from_str;

// use quick_xml::de::from_str;
use crate::Error;
use crate::archive::EpubArchive;
use crate::error::OtherContext;

pub struct Nav {
    spec: spec::Html,
}

impl Nav {
    pub fn parse(epub: &mut EpubArchive, path: &str) -> Result<Self, Error> {
        let data = epub.read(path)?;

        Nav::from_str(&data)
    }

    pub fn to_entries(self) -> Vec<(String, String)> {
        let toc = self
            .spec
            .body
            .nav
            .into_iter()
            .find(|it| it.epub_type == "toc")
            .unwrap();

        toc.ol
            .li
            .into_iter()
            .map(|it| match (it.a, it.span) {
                (None, None) => unreachable!(),
                (None, Some(span)) => (span.a.content, span.a.href),
                (Some(a), None) => (a.content, a.href),
                (Some(_), Some(_)) => unreachable!(),
            })
            .collect::<Vec<_>>()
    }
}

impl FromStr for Nav {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let html = from_str::<spec::Html>(s)
            .boxed_local()
            .context(OtherContext {})?;

        /*
        if html.is_err() {
            // println!("{}", s);
            html?;
        } else {
            // println!("{:#?}", html);
        }
         */

        Ok(Nav { spec: html })
    }
}

#[expect(dead_code, reason = "adhering to spec")]
mod spec {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Html {
        pub body: Body,
    }

    #[derive(Debug, Deserialize)]
    pub struct Body {
        pub nav: Vec<Nav>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Nav {
        #[serde(rename = "@epub:type")]
        pub epub_type: String,
        pub ol: Ol,
    }

    #[derive(Debug, Deserialize)]
    pub struct Ol {
        #[serde(default)]
        pub li: Vec<Li>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Li {
        pub a: Option<A>,
        pub span: Option<Span>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Span {
        pub a: A,
    }

    #[derive(Debug, Deserialize)]
    pub struct A {
        #[serde(rename = "@href")]
        pub href: String,
        #[serde(rename = "#text")]
        pub content: String,
    }
}

/*
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops">
<head>
<meta charset="UTF-8"/>
<title>Navigation</title>
</head>
<body>

<nav epub:type="toc" id="toc">
<h1>Navigation</h1>

<ol>
<li><a href="Text/p-toc-001.xhtml">目次</a></li>
<li><a href="Text/p-002.xhtml#toc-001">プロローグ　借金が増加する</a></li>
<li><a href="Text/p-003.xhtml#toc-002">第一章　依頼受理から買物する</a></li>
<li><a href="Text/p-008.xhtml#toc-003">第二章　増額の後から判明する</a></li>
<li><a href="Text/p-015.xhtml#toc-004">第三章　再進行から探索する</a></li>
<li><a href="Text/p-018.xhtml#toc-005">第四章　探索から追跡する</a></li>
<li><a href="Text/p-022.xhtml#toc-006">第五章　進撃から気絶する</a></li>
<li><a href="Text/p-025.xhtml#toc-007">エピローグ　目覚めから休息する</a></li>
<li><a href="Text/p-028.xhtml#toc-008">幕間　とある神官の手記より</a></li>
<li><a href="Text/p-colophon.xhtml">奥付</a></li>
</ol>

</nav>

<nav epub:type="landmarks" id="guide">
<h1>Guide</h1>
<ol>
    <li><a epub:type="cover" href="Text/p-cover.xhtml">Cover</a></li>
    <li><a epub:type="toc" href="Text/p-toc-001.xhtml">目次</a></li>
</ol>
</nav>
</body>
</html>

*/
