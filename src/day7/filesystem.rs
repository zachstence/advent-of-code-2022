use std::fmt::Display;
use itertools::Itertools;


enum Node {
    Directory(Directory),
    File(File),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Directory(dir) => write!(f, "{}", dir),
            Node::File(file) => write!(f, "{}", file),
        }
    }
}

pub struct Directory {
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} (dir)", self.name)
    }
}

pub struct File {
    name: String,
    size: u32,
    parent: usize,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} (file, size={})", self.name, self.size)
    }
}

pub struct Filesystem {
    nodes: Vec<Node>,
    curr_index: usize,
}

impl Filesystem {
    pub fn new() -> Self {
        let root_dir = Directory {
            name: "/".to_string(),
            parent: None,
            children: vec![]
        };
        let root_node = Node::Directory(root_dir);

        Self {
            nodes: vec![root_node],
            curr_index: 0,
        }
    }

    pub fn root_dir(&self) -> &Directory {
        match &self.nodes[0] {
            Node::Directory(dir) => dir,
            _ => panic!("root_dir isn't a directory"),
        }
    }

    pub fn curr_dir(&self) -> &Directory {
        match &self.nodes[self.curr_index] {
            Node::Directory(dir) => dir,
            _ => panic!("curr_dir isn't a directory"),
        }
    }

    pub fn curr_dir_mut(&mut self) -> &mut Directory {
        match &mut self.nodes[self.curr_index] {
            Node::Directory(dir) => dir,
            _ => panic!("curr_dir isn't a directory"),
        }
    }

    pub fn cd(&mut self, index: usize) {
        let node = &self.nodes[index];
        match node {
            Node::Directory(_) =>  {
                self.curr_index = index;
            },
            _ => panic!("Cannot `cd` into non-directory"),
        }
    }

    pub fn add_directory(&mut self, name: String) -> usize {       
        let parent_index = self.curr_index;
        let child_index = self.nodes.len();

        let dir = Directory {
            name,
            parent: Some(parent_index),
            children: vec![],
        };
        let node = Node::Directory(dir);
        self.nodes.push(node);

        self.curr_dir_mut().children.push(child_index);

        child_index
    }

    pub fn add_file(&mut self, name: String, size: u32) -> usize {
        let parent_index = self.curr_index;
        let child_index = self.nodes.len();

        let file = File {
            name,
            size,
            parent: parent_index,
        };
        let node = Node::File(file);
        self.nodes.push(node);

        self.curr_dir_mut().children.push(child_index);

        child_index
    }

    #[allow(clippy::only_used_in_recursion)]
    pub fn dir_to_string(&self, dir: &Directory, level: usize) -> String {
        let prefix = (0..level).map(|_| "  ").collect::<String>();

        let top_str = format!("{prefix}{dir}");

        let children_str = dir
            .children
            .iter()
            .map(|i| {
                match &self.nodes[*i] {
                    Node::Directory(d) => self.dir_to_string(d, level + 1),
                    Node::File(f) => format!("  {prefix}{f}"),
                }
            })
            .join("\n");

        if !children_str.is_empty() {
            top_str + "\n" + &children_str
        } else {
            top_str
        }

        
    }
}

impl Display for Filesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.dir_to_string(self.root_dir(), 0);
        write!(f, "{s}")
    }
}
