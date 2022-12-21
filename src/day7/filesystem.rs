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
    total_size: u64,
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} (dir, total_size={})", self.name, self.total_size)
    }
}

pub struct File {
    name: String,
    size: u64,
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
            children: vec![],
            total_size: 0,
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

    pub fn get_node_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }

    pub fn cd(&mut self, dir_name: &String) {
        if dir_name == ".." {
            let index = self.curr_dir().parent.unwrap_or(0);
            return self.cd_index(index);
        }

        // TODO only need to search current node's children
        // Each node needs to know its index in order for that to work ^
        let index: usize = self.nodes
            .iter()
            .find_position(|node| match node {
                Node::Directory(dir) => dir.name == *dir_name,
                Node::File(_) => false,
            })
            .unwrap()
            .0;
        
        self.cd_index(index)
    }

    fn cd_index(&mut self, index: usize) {
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
            total_size: 0,
        };
        let node = Node::Directory(dir);
        self.nodes.push(node);

        self.curr_dir_mut().children.push(child_index);

        child_index
    }

    pub fn add_file(&mut self, name: String, size: u64) -> usize {
        let parent_index = self.curr_index;
        let child_index = self.nodes.len();

        let file = File {
            name,
            size,
            parent: parent_index,
        };
        let node = Node::File(file);
        self.nodes.push(node);

        // Add index to children
        self.curr_dir_mut().children.push(child_index);

        // Update dir sizes
        let mut index = Some(self.curr_index);
        while let Some(i) = index {
            if let Some(Node::Directory(dir)) = self.get_node_mut(i) {
                // TODO if this is over 100,000 we don't need to add
                // TODO if this addition goes over 100,000 we can stop iterating
                dir.total_size += size;
                
                index = dir.parent;
            }
        }

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
