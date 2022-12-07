use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::ops::{ControlFlow, Index, IndexMut};
use std::str::FromStr;

use itertools::Itertools;

pub fn star_one(input: &str) -> u64 {
    let file_system: FileSystem = input.parse().expect("Failed to parse file system");

    let sizes = file_system.calculate_sizes();
    sizes.values().filter(|&&s| s <= 100000).sum()
}

pub fn star_two(input: &str) -> u64 {
    let file_system: FileSystem = input.parse().expect("Failed to parse file system");

    let sizes = file_system.calculate_sizes();
    let root_path = file_system.root_path();
    let free_space = 70_000_000 - sizes[&root_path];
    let need_to_free = 30_000_000 - free_space;

    sizes
        .into_iter()
        .filter(|(p, s)| p != &root_path && s >= &need_to_free)
        .map(|(_, s)| s)
        .min()
        .unwrap()
}

#[derive(Debug)]
struct FileSystem {
    root_idx: Idx,
    arena: Arena<Entry>,
}

impl FileSystem {
    fn walk<F, B, C>(&self, mut visit: F) -> ControlFlow<B, C>
    where
        F: FnMut(&Entry, &Path, usize) -> ControlFlow<B, C>,
    {
        let mut stack = VecDeque::new();
        stack.push_back((self.root_idx, Path::of(self.root_idx), 0));
        let mut last_return = None;

        while let Some((idx, path, depth)) = stack.pop_back() {
            let entry = &self.arena[idx];

            // Visit
            let control = visit(&entry, &path, depth);
            if control.is_break() {
                return control;
            }
            last_return = Some(control);

            match entry {
                Entry::Root { children } | Entry::Dir { children, .. } => {
                    for child in children.iter().rev() {
                        stack.push_back((*child, path.extend(*child), depth + 1));
                    }
                }
                _ => {}
            }
        }

        // Unwrap is safe because we always have at least the root
        last_return.unwrap()
    }

    fn root_path(&self) -> Path {
        Path::of(self.root_idx)
    }

    fn calculate_sizes(self: &FileSystem) -> HashMap<Path, u64> {
        // TODO: Should probably just do this when parsing
        let mut sizes: HashMap<Path, u64> = Default::default();
        self.walk::<_, (), ()>(|e, path, _| {
            if e.is_dir() && !sizes.contains_key(&path) {
                sizes.insert(path.clone(), 0);
            }

            match e {
                Entry::File { size, .. } => {
                    for (seen_path, seen_size) in sizes.iter_mut() {
                        if path.is_sub_path(seen_path) {
                            *seen_size += size;
                        }
                    }
                }
                _ => {}
            }

            ControlFlow::Continue(())
        });

        sizes
    }
}

impl FromStr for FileSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut arena: Arena<Entry> = Arena::new();
        let mut lines = s.lines().map(str::trim).filter(|l| !l.is_empty());
        let root_idx = arena.insert(Entry::Root { children: vec![] });
        let mut current_idx = root_idx;

        while let Some(line) = lines.next() {
            if let Some(rest) = line.strip_prefix("$") {
                if rest.trim() == "ls" {
                    let ls_output = lines.take_while_ref(|l| !l.starts_with("$"));

                    let children = parse_ls_output(ls_output, &mut arena, current_idx)?;

                    arena[current_idx].set_children(children);
                    continue;
                }

                if let Some(rest) = rest.trim().strip_prefix("cd").map(str::trim) {
                    if rest == "/" {
                        current_idx = root_idx;
                        continue;
                    }

                    if rest == ".." {
                        current_idx = arena[current_idx].parent();
                        continue;
                    }

                    let arg = rest.trim();
                    let dir_idx = arena[current_idx]
                        .find_child_idx(
                            &arena,
                            |e| matches!(e, Entry::Dir { name, .. } if name == arg),
                        )
                        .ok_or_else(|| format!("Invalid cd to {}, dir missing", arg))?;
                    current_idx = dir_idx;
                }
            }
        }

        Ok(Self { arena, root_idx })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Path {
    segments: Vec<Idx>,
}

impl Path {
    fn of(segment: Idx) -> Self {
        Self {
            segments: vec![segment],
        }
    }

    fn extend(&self, segment: Idx) -> Self {
        let mut segments = Vec::with_capacity(self.segments.len() + 1);
        segments.extend_from_slice(&self.segments);
        segments.push(segment);

        Self { segments }
    }

    fn is_sub_path(&self, of: &Path) -> bool {
        self.segments.starts_with(&of.segments)
    }
}

fn parse_ls_output<'a>(
    lines: impl IntoIterator<Item = &'a str>,
    arena: &mut Arena<Entry>,
    parent_idx: Idx,
) -> Result<Vec<Idx>, String> {
    lines
        .into_iter()
        .map(|l| Entry::parse(l, parent_idx))
        .map(|r| r.map(|e| arena.insert(e)))
        .collect()
}

#[derive(Debug)]
enum Entry {
    Root {
        children: Vec<Idx>,
    },
    Dir {
        name: String,
        children: Vec<Idx>,
        parent: Idx,
    },
    File {
        size: u64,
        name: String,
    },
}

impl Entry {
    fn set_children(&mut self, new_children: Vec<Idx>) {
        assert!(matches!(self, Entry::Root { .. } | Entry::Dir { .. }));
        match self {
            Entry::Root { children } => *children = new_children,
            Entry::Dir { children, .. } => *children = new_children,
            Entry::File { .. } => unreachable!("Can't set children for file entry"),
        }
    }

    fn parent(&self) -> Idx {
        match self {
            Entry::Dir { parent, .. } => *parent,
            _ => unreachable!("Attempted to find the parent of {:?}", self),
        }
    }

    fn find_child_idx<F>(&self, arena: &Arena<Entry>, mut predicate: F) -> Option<Idx>
    where
        F: FnMut(&Entry) -> bool,
    {
        match self {
            Entry::Root { children } | Entry::Dir { children, .. } => children
                .iter()
                .find(|&&idx| predicate(&arena[idx]))
                .copied(),
            Entry::File { .. } => unreachable!("Attempted to get children of file"),
        }
    }

    fn parse(s: &str, parent: Idx) -> Result<Self, String> {
        // Dir
        if let Some(name) = s.strip_prefix("dir") {
            return Ok(Self::Dir {
                name: name.trim().to_owned(),
                children: vec![],
                parent,
            });
        }

        // Must be a file then
        let (size, name) = s
            .split_once(char::is_whitespace)
            .ok_or_else(|| format!("Invalid ls output {}", s))?;
        let size = size
            .parse()
            .map_err(|e| format!("Failed to parse file size in {}, {}", s, e))?;
        let name = name.trim().to_owned();

        Ok(Self::File { size, name })
    }

    fn is_dir(&self) -> bool {
        matches!(self, Entry::Dir { .. } | Entry::Root { .. })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Idx(usize);

#[derive(Default, Debug)]
struct Arena<T> {
    storage: Vec<T>,
}

impl<T> Arena<T> {
    fn new() -> Self {
        Self { storage: vec![] }
    }

    fn insert(&mut self, value: T) -> Idx {
        let idx = Idx(self.storage.len());

        self.storage.push(value);

        idx
    }
}

impl<T> Index<Idx> for Arena<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.storage[index.0]
    }
}

impl<T> IndexMut<Idx> for Arena<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.storage[index.0]
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Root { .. } => write!(f, "/ (dir)"),
            Entry::Dir { name, .. } => write!(f, "/ {} (dir)", name),
            Entry::File { size, name } => write!(f, "{} (file, size={})", name, size),
        }
    }
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let ControlFlow::Break(e) =
            self.walk(
                |e, _, depth| match write!(f, "{:>depth$}{}\n", "", e, depth = depth * 2) {
                    Err(e) => ControlFlow::Break(e),
                    Ok(_) => ControlFlow::Continue(()),
                },
            )
        {
            return Err(e);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two, FileSystem};

    const TEST_INPUT: &'static str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(TEST_INPUT), 95437);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(TEST_INPUT), 24933642);
    }
}
