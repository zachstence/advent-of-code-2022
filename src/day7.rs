#[aoc(day7, part1)]
pub fn part1(input: &str) -> u32 {
    let mut filesystem = Filesystem::new();

    filesystem.cd("/");

    filesystem.ls("dir a
14848514 b.txt
8504156 c.dat
dir d");


    0
}

enum FilesystemNode<'a> {
    File(File),
    Dir(&'a mut Dir),
}

trait IsDir {
    fn is_dir(&self) -> bool;
}

impl IsDir for FilesystemNode<'_> {
    fn is_dir(&self) -> bool {
        match self {
            FilesystemNode::Dir(_) => true,
            FilesystemNode::File(_) => false,
        }
    }
}

struct Filesystem<'a> {
    current_index: usize,
    nodes: Vec<&'a mut FilesystemNode<'a>>,
}

impl<'a> Filesystem<'a> {
    fn new() -> &'a Self {
        let root = FilesystemNode::Dir(
            &mut Dir {
                index: 0,
                name: String::from("/"),
                size: 0,
                parent_index: None,
                children_indices: vec![],
            }
        );

        &Self {
            current_index: 0,
            nodes: vec![&root],
        }
    }

    fn current_dir(&'a self) -> &'a mut Dir {
        match self.nodes.get(self.current_index).expect("Current node should exist") {
            FilesystemNode::Dir(dir) => dir,
            FilesystemNode::File(file) => panic!("Invalid state, current directory is a file {}", file.name),
        }
    }

    // Change to a known child directory or root "/"
    fn cd(&mut self, name: &str) -> &Self {
        if name == "/" {
            self.current_index = 0;
            return self;
        }


        let children = self.current_dir()
            .children_indices
            .iter()
            .map(|i| self.nodes.get(*i).unwrap())
            .filter(|node| node.is_dir());
        println!("Children {:?}", children);

        // if let Some(dir) = current_dir {
        //     if let Some(child_dir) = dir.find_child_dir(name) {
        //         self.current_index = child_dir.index;
        //         return self;
        //     }
        // }

        self
    }

    /** Create and add dirs/files based on the output of "ls" */
    fn ls(&mut self, ls_output: &str) -> &Self {
        ls_output.lines().for_each(|line| {
            let (dir_or_size, name) = line.split_once(' ').unwrap();

            if dir_or_size == "dir" {
                println!("Found subdir {}", name);

                let &mut current_dir = self.current_dir();
        
                current_dir.add_child_dir(self.current_index, String::from(name));
            } else {
                println!("Found file {} with size {}", name, dir_or_size)
            }
        });

        self
    }
}

struct Dir {
    index: usize,
    name: String,
    size: u32,
    parent_index: Option<usize>,
    children_indices: Vec<usize>
}

struct File {
    name: String,
    size: u32,
    parent_index: usize,
}

impl Dir {
    fn add_child_dir(mut self, index: usize, name: String) -> Dir {
        let new_dir = Self {
            index,
            name,
            size: 0,
            parent_index: Some(self.index),
            children_indices: vec![],
        };
        self.children_indices.push(index);

        new_dir
    }

    fn add_child_file(mut self, index: usize, name: String, size: u32) -> File {
        let new_file = File {
            name,
            size,
            parent_index: self.index,
        };
        self.children_indices.push(index);

        new_file
    }
}
