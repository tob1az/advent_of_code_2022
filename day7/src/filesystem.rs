use std::cell::RefCell;
use std::rc::Rc;

type MutableNode = RefCell<Node>;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    size: usize,
    subnodes: Vec<Rc<MutableNode>>,
}

impl Node {
    pub fn new_file(name: &str, size: usize) -> Self {
        Self {
            name: name.to_owned(),
            size,
            subnodes: vec![],
        }
    }

    pub fn new_directory(name: &str, file_sizes: &[(String, usize)]) -> Self {
        Self {
            name: name.to_owned(),
            size: 0,
            subnodes: file_sizes
                .into_iter()
                .map(|(n, s)| Rc::new(RefCell::new(Self::new_file(n.as_str(), *s))))
                .collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
            + self
                .subnodes
                .iter()
                .map(|n| n.borrow().size())
                .sum::<usize>()
    }

    pub fn is_file(&self) -> bool {
        self.subnodes.is_empty()
    }
}

#[derive(Debug)]
pub struct TreeWalker {
    root: Rc<MutableNode>,
    directory_stack: Vec<Rc<MutableNode>>,
}

impl Default for TreeWalker {
    fn default() -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new_directory("/", &vec![]))),
            directory_stack: vec![],
        }
    }
}

impl TreeWalker {
    pub fn cd(&mut self, directory: &str) {
        match directory {
            "/" => self.directory_stack = vec![self.root.clone()],
            ".." => {
                self.directory_stack.pop();
            }
            _ => {
                let subdirectory = self
                    .current_directory()
                    .borrow()
                    .subnodes
                    .iter()
                    .filter(|n| n.borrow().name == directory)
                    .next()
                    .expect("Directory is known")
                    .clone();
                self.directory_stack.push(subdirectory);
            }
        }
    }

    pub fn populate_current_directory(&mut self, file_sizes: &[(String, usize)]) {
        self.current_directory()
            .replace_with(|node| Node::new_directory(node.name.as_str(), file_sizes));
    }

    pub fn traverse<F>(&self, f: &mut F)
    where
        F: FnMut(&Node),
    {
        Self::traverse_subtree(&self.root.borrow(), f);
    }

    fn traverse_subtree<F>(node: &Node, f: &mut F)
    where
        F: FnMut(&Node),
    {
        f(node);
        for node in node.subnodes.iter() {
            Self::traverse_subtree(&*node.borrow(), f);
        }
    }

    fn current_directory(&self) -> Rc<MutableNode> {
        self.directory_stack
            .last()
            .expect("Directory is selected")
            .clone()
    }
}
