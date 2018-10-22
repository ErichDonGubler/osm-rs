extern crate osmpbfreader;
extern crate roxmltree;
#[macro_use]
extern crate structopt;
extern crate version_compare;

pub use osmpbfreader::objects::{
    Node,
    NodeId,
    Ref,
    Relation,
    RelationId,
    Tags,
    Way,
    WayId,
    OsmId,
    OsmObj,
    TagsImpl,
};
use {
    roxmltree::{
        Document,
    },
    std::{
        fmt::{
            Debug,
            Formatter,
            Result as FmtResult,
        },
        fs::read_to_string,
        path::PathBuf,
    },
    structopt::StructOpt,
    version_compare::{
        CompOp,
        Version,
    },
};

struct OsmXml<T>(T);

impl<T> Debug for OsmXml<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_tuple("OsmXml")
            .field(&self.0)
            .finish()
    }
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
}

pub fn main() {
    let Cli {
        input_file,
    } = Cli::from_args();

    let input = read_to_string(input_file).unwrap();
    let document = Document::parse(&input).unwrap();

    let root = {
        let mut root_children = document.root().children();
        match (root_children.next(), root_children.next()) {
            (Some(node), None) => {
                node
            }
            (Some(node1), Some(node2)) => panic!("Crap, more than one root element: {:?}, {:?}", node1, node2),
            (None, _) => panic!("Crap, no root elements found!)"),
        }
    };
    assert!(root.has_tag_name("osm"));

    let osm_version = Version::from(root.attribute("version").unwrap()).unwrap();
    assert!(Version::from("0.6").unwrap().compare_to(&osm_version, &CompOp::Eq));

    println!("root: {:#?}", root);

    for child in root.children() {
        println!("child: {:#?}", child);
    }
}
