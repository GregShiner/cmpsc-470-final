use phf::phf_set;
use sexp;

pub mod parse;

#[derive(Debug)]
struct TokenCount {
    lists: u32,
    atoms: u32,
    builtins: u32,
    identifiers: u32,
    strings: u32,
    integers: u32,
    floats: u32,
}

impl Default for TokenCount {
    fn default() -> Self {
        TokenCount {
            lists: 0,
            atoms: 0,
            builtins: 0,
            identifiers: 0,
            strings: 0,
            integers: 0,
            floats: 0,
        }
    }
}

impl TokenCount {
    fn new(
        lists: u32,
        atoms: u32,
        builtins: u32,
        identifiers: u32,
        strings: u32,
        integers: u32,
        floats: u32,
    ) -> Self {
        TokenCount {
            lists,
            atoms,
            builtins,
            identifiers,
            strings,
            integers,
            floats,
        }
    }
}

impl std::ops::Add for TokenCount {
    type Output = TokenCount;

    fn add(self, rhs: Self) -> Self::Output {
        TokenCount {
            lists: self.lists + rhs.lists,
            atoms: self.atoms + rhs.atoms,
            builtins: self.builtins + rhs.builtins,
            identifiers: self.identifiers + rhs.identifiers,
            strings: self.strings + rhs.strings,
            integers: self.integers + rhs.integers,
            floats: self.floats + rhs.floats,
        }
    }
}

impl std::iter::Sum for TokenCount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(TokenCount::default(), |acc, x| acc + x)
    }
}

const BUILTINS: phf::Set<&'static str> = phf_set! { "let", "lambda", "box", "unbox", "ref", "&" };

fn count_tokens(s_exp: &sexp::Sexp) -> TokenCount {
    match s_exp {
        sexp::Sexp::Atom(atom) => match atom {
            sexp::Atom::S(s) => {
                let s2: &String = &"Hello".to_owned();
                if BUILTINS.contains(&s) {
                    TokenCount {
                        lists: 0,
                        atoms: 1,
                        builtins: 1,
                        identifiers: 0,
                        strings: 0,
                        integers: 0,
                        floats: 0,
                    }
                } else if s.starts_with("\"") && s.ends_with("\"") && s.len() >= 2 {
                    TokenCount {
                        lists: 0,
                        atoms: 1,
                        builtins: 0,
                        identifiers: 0,
                        strings: 1,
                        integers: 0,
                        floats: 0,
                    }
                } else {
                    TokenCount {
                        lists: 0,
                        atoms: 1,
                        builtins: 0,
                        identifiers: 1,
                        strings: 0,
                        integers: 0,
                        floats: 0,
                    }
                }
            }
            sexp::Atom::I(_) => TokenCount {
                lists: 0,
                atoms: 1,
                builtins: 0,
                identifiers: 0,
                strings: 0,
                integers: 1,
                floats: 0,
            },
            sexp::Atom::F(_) => TokenCount {
                lists: 0,
                atoms: 1,
                builtins: 0,
                identifiers: 0,
                strings: 0,
                integers: 0,
                floats: 1,
            },
        },
        sexp::Sexp::List(list) => {
            list.iter().map(count_tokens).sum::<TokenCount>()
                + TokenCount {
                    lists: 1,
                    atoms: 0,
                    builtins: 0,
                    identifiers: 0,
                    strings: 0,
                    integers: 0,
                    floats: 0,
                }
        }
    }
}

fn main() {
    let list = sexp::parse(
        "(let ((x (box 5)))
            (let ((ref (& x)))
                (let ((y x))
                    (unbox x))))",
    )
    .unwrap();
    println!("{:?}", list);
    println!("{:?}", count_tokens(&list));
}
