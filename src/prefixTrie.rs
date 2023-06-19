use std::collections::BTreeMap;

struct  PrefixTrieNode {
    children: BTreeMap<char, PrefixTrieNode>
}

impl PrefixTrieNode {
    fn insert(&mut self, word: &str) {
        if word.is_empty() {
            return
        }
        let prefix = word.as_bytes()[0] as char;
        let p = &prefix;
        if !self.children.contains_key(p){
            self.children.insert(prefix,  PrefixTrieNode { children: BTreeMap::new()});
        }
        let rest = &word[1..];
        self.children.get_mut(p).unwrap().insert(rest);
    }
    
}


enum PrefixTrie {
    Empty,
    Root(PrefixTrieNode)
}


impl PrefixTrie {
    fn new() -> Self {
        PrefixTrie::Root(PrefixTrieNode { children: BTreeMap::new()})
    }

    pub fn insert(&mut self, word: &str) {
        match self {
            PrefixTrie::Empty => {
                return
            },
            PrefixTrie::Root(ref mut root) => {
                root.insert(word);
            }
        }
    }

    pub fn search(self, prefix: String) -> bool {
        match self {
            PrefixTrie::Empty => false,
            PrefixTrie::Root(root) => {
                let mut cur = &root;
                for c in prefix.chars() {
                    if !cur.children.contains_key(&c) {
                        return false;
                    }
                    cur = cur.children.get(&c).unwrap()
                }
                return true
            }
        }
    }
    
     

}


#[cfg(test)]
mod tests {
    use super::*;
    const COMMANDS: [&str; 2] = ["enter", "look"];

    #[test]
    fn build() {
        let mut trie = PrefixTrie::Empty;
        for c in COMMANDS {
            trie.insert(c);
            assert!(trie.search(c.to_string()));
        }

    }


}
