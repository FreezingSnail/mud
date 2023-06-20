use std::collections::BTreeMap;


#[derive(Debug)]
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

#[derive(Debug)]
struct PrefixTrie {
    root: PrefixTrieNode
}


impl PrefixTrie {
    fn new() -> Self {
        PrefixTrie{root: PrefixTrieNode { children: BTreeMap::new()}}
    }

    pub fn insert(&mut self, word: &str) {
        self.root.insert(word);
    }

    pub fn search(&self, prefix: String) -> bool {
        let mut cur = &self.root;
        for c in prefix.chars() {
            if !cur.children.contains_key(&c) {
                return false;
            }
            cur = cur.children.get(&c).unwrap()
        }
        return true
    }
}
    
     



#[cfg(test)]
mod tests {
    use super::*;
    const COMMANDS: [&str;3] = ["enter", "look", "pickup"];

    #[test]
    fn build() {
        let mut trie = PrefixTrie::new();
        for c in COMMANDS {
            trie.insert(c);
            assert!(trie.search(c.to_string()))
        }
        println!("{:?}", trie)

    }


}
