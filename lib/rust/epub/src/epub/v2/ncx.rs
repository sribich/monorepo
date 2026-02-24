use std::str::FromStr;

use quick_xml::de::from_str;

use crate::Error;
use crate::archive::EpubArchive;

pub struct Ncx {
    spec: spec::Ncx,
}

impl Ncx {
    pub fn parse(epub: &mut EpubArchive, path: &str) -> Result<Self, Error> {
        let data = epub.read(path)?;

        Ncx::from_str(&data)
    }

    pub fn to_entries(self) -> Vec<(String, String)> {
        self.spec
            .navmap
            .nav_points
            .into_iter()
            .flat_map(spec::NavPoint::to_entries)
            .collect::<Vec<_>>()
    }
}

impl FromStr for Ncx {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ncx { spec: from_str(s)? })
    }
}

#[expect(dead_code, reason = "adhering to spec")]
mod spec {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Ncx {
        #[serde(rename = "navMap")]
        pub navmap: NavMap,
    }

    #[derive(Deserialize)]
    pub struct NavMap {
        #[serde(rename = "navPoint")]
        pub nav_points: Vec<NavPoint>,
    }

    #[derive(Deserialize)]
    pub struct NavPoint {
        #[serde(rename = "navLabel")]
        pub nav_label: NavLabel,
        pub content: Content,
        #[serde(rename = "navPoint")]
        pub children: Option<Vec<NavPoint>>,
    }

    #[derive(Deserialize)]
    pub struct NavLabel {
        pub text: String,
    }

    #[derive(Deserialize)]
    pub struct Content {
        #[serde(rename = "@src")]
        pub src: String,
    }

    impl NavPoint {
        pub fn to_entries(self) -> Vec<(String, String)> {
            let src = self.content.src;

            let items = vec![(self.nav_label.text, src)];

            if let Some(children) = self.children {
                [
                    items,
                    children
                        .into_iter()
                        .flat_map(NavPoint::to_entries)
                        .collect::<Vec<_>>(),
                ]
                .concat()
            } else {
                items
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use indoc::indoc;
    use insta::assert_debug_snapshot;

    use crate::epub::v2::ncx::Ncx;

    #[test]
    pub fn supports_nested_toc() {
        let toc = indoc! {r#"
            <?xml version="1.0"?>
            <ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1">
               <head>
                   <meta name="dtb:uid" content="..."/>
                   <meta name="dtb:depth" content="3"/>
                   <meta name="dtb:totalPageCount" content="0"/>
                   <meta name="dtb:maxPageNumber" content="0"/>
               </head>
               <docTitle>
                   <text>...</text>
               </docTitle>
               <navMap>
                   <navPoint id="navPoint-1" playOrder="1">
                       <navLabel>
                           <text>Section with no subsection</text>
                       </navLabel>
                       <content src="text/content001.xhtml"/>
                   </navPoint>
                   <navPoint id="navPoint-2" playOrder="2">
                       <navLabel>
                           <text>TOC entry name Section title</text>
                       </navLabel>
                       <content src="text/content001.xhtml#heading_id_3"/>
                       <navPoint id="navPoint-3" playOrder="3">
                           <navLabel>
                               <text>Section entry name.</text>
                           </navLabel>
                           <content src="text/content002.xhtml"/>
                       </navPoint>
                       <navPoint id="navPoint-4" playOrder="4">
                           <navLabel>
                               <text>Introduction.</text>
                           </navLabel>
                           <content src="text/content003.xhtml"/>
                           <navPoint id="navPoint-5" playOrder="5">
                               <navLabel>
                                   <text>Preserving the Text.</text>
                               </navLabel>
                               <content src="text/content003.xhtml#heading_id_217"/>
                           </navPoint>
                           <navPoint id="navPoint-6" playOrder="6">
                               <navLabel>
                                   <text>Lower level chapter title</text>
                               </navLabel>
                               <content src="text/content003.xhtml#heading_id_218"/>
                           </navPoint>
                           <navPoint id="navPoint-7" playOrder="7">
                               <navLabel>
                                   <text>Another lower level title.</text>
                               </navLabel>
                               <content src="text/content003.xhtml#heading_id_219"/>
                           </navPoint>
                       </navPoint>
                   </navPoint>
               </navMap>
            </ncx>
        "#};

        let data = Ncx::from_str(toc).unwrap();
        let data = data.to_entries();

        assert_debug_snapshot!(data, @r#"
        [
            (
                "Section with no subsection",
                "text/content001.xhtml",
            ),
            (
                "TOC entry name Section title",
                "text/content001.xhtml#heading_id_3",
            ),
            (
                "Section entry name.",
                "text/content002.xhtml",
            ),
            (
                "Introduction.",
                "text/content003.xhtml",
            ),
            (
                "Preserving the Text.",
                "text/content003.xhtml#heading_id_217",
            ),
            (
                "Lower level chapter title",
                "text/content003.xhtml#heading_id_218",
            ),
            (
                "Another lower level title.",
                "text/content003.xhtml#heading_id_219",
            ),
        ]
        "#);
    }
}
