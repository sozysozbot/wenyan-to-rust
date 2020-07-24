use crate::parse;
use bimap::BiMap;
use std::collections::HashMap;
use std::collections::HashSet;

fn to_pinyin(ident: parse::Identifier, conversion_table: &HashMap<String, String>) -> String {
    let parse::Identifier(i) = ident;
    let vec = i
        .chars()
        .map(|c| match conversion_table.get(&format!("{:X}", c as u32)) {
            None => "_".to_string(),
            Some(a) => a.split(' ').collect::<Vec<_>>()[0].to_string(),
        })
        .collect::<Vec<_>>();
    vec.join("")
}

type Hanzi = parse::Identifier;
type Ascii = String;

#[derive(Clone)]
pub struct IdentBiMap {
    bimap: BiMap<Hanzi, Ascii>,
    mutable_idents: HashSet<Hanzi>,
}

impl IdentBiMap {
    pub fn translate_from_hanzi(&self, id: &parse::Identifier) -> Ascii {
        self.bimap.get_by_left(id).unwrap().to_string()
    }

    pub fn is_mutable(&self, id: &parse::Identifier) -> bool {
        self.mutable_idents.contains(id)
    }

    pub fn new(parsed: &[parse::Statement], conversion_table: &HashMap<String, String>) -> Self {
        let mut ans = IdentBiMap {
            bimap: BiMap::new(),
            mutable_idents: HashSet::new(),
        };
        for st in parsed {
            ans.insert_stmt(&st, &conversion_table);
        }

        eprintln!("bimap: {:?}", ans.bimap);
        eprintln!("mutable_idents: {:?}", ans.mutable_idents);
        ans
    }

    fn insert_ident(
        &mut self,
        ident: &parse::Identifier,
        conversion_table: &HashMap<String, String>,
    ) {
        // if already known, no need to do anything
        if self.bimap.get_by_left(&ident).is_some() {
            return;
        }

        // otherwise, ident is unknown, and hence must be added.

        let mut candidate: Ascii = to_pinyin(ident.clone(), &conversion_table);

        loop {
            if self.bimap.get_by_right(&candidate).is_some() {
                candidate.push('_');
            } else {
                self.bimap.insert(ident.clone(), candidate);
                break;
            }
        }
    }

    fn insert_stmts(
        &mut self,
        statements: &[parse::Statement],
        conversion_table: &HashMap<String, String>,
    ) {
        for s in statements {
            self.insert_stmt(&s, &conversion_table)
        }
    }

    fn insert_dat(&mut self, dat: &parse::Data, conversion_table: &HashMap<String, String>) {
        if let parse::Data::Identifier(id) = dat {
            self.insert_ident(&id, &conversion_table)
        }
    }

    fn insert_data_or_qi2(
        &mut self,
        dat: &parse::DataOrQi2,
        conversion_table: &HashMap<String, String>,
    ) {
        if let parse::DataOrQi2::Data(d1) = dat {
            self.insert_dat(d1, &conversion_table);
        }
    }

    fn insert_ifexpr(
        &mut self,
        ifexpr: &parse::IfCond,
        conversion_table: &HashMap<String, String>,
    ) {
        match ifexpr {
            parse::IfCond::Binary(data1, _, data2) => {
                self.insert_data_or_qi2(data1, &conversion_table);
                self.insert_data_or_qi2(data2, &conversion_table);
            }
            parse::IfCond::Unary(data) => {
                self.insert_data_or_qi2(data, &conversion_table);
            }
            parse::IfCond::NotQi2 => {}
        }
    }

    fn insert_math(&mut self, math: &parse::MathKind, conversion_table: &HashMap<String, String>) {
        match math {
            parse::MathKind::ArithUnaryMath(data) => {
                self.insert_data_or_qi2(data, &conversion_table)
            }

            parse::MathKind::ArithBinaryMath(_, data1, _, data2)
            | parse::MathKind::ModMath(_, data1, _, data2) => {
                self.insert_data_or_qi2(data1, &conversion_table);
                self.insert_data_or_qi2(data2, &conversion_table);
            }
            parse::MathKind::BooleanAlgebra(ident1, ident2, _) => {
                self.insert_ident(&ident1, &conversion_table);
                self.insert_ident(&ident2, &conversion_table);
            }
        }
    }
    fn insert_stmt(&mut self, st: &parse::Statement, conversion_table: &HashMap<String, String>) {
        match st {
            parse::Statement::ForArr { list, elem, stmts } => {
                self.insert_ident(&list, &conversion_table);
                self.insert_ident(&elem, &conversion_table);
                self.insert_stmts(&stmts, &conversion_table)
            }
            parse::Statement::ArrayCat { append_to, elems } => {
                self.insert_data_or_qi2(&parse::DataOrQi2::from(append_to), &conversion_table);
                for ident in elems {
                    self.insert_ident(&ident, &conversion_table)
                }
            }
            parse::Statement::ArrayFill {
                what_to_fill,
                elems,
            } => {
                self.insert_data_or_qi2(&parse::DataOrQi2::from(what_to_fill), &conversion_table);
                if let parse::IdentOrQi2::Ident(ident) = what_to_fill {
                    self.mutable_idents.insert(ident.clone());
                }
                for e in elems {
                    self.insert_dat(&e, &conversion_table);
                }
            }
            parse::Statement::If {
                ifcase: (ifexpr, ifcase),
                elseifcases,
                elsecase,
            } => {
                self.insert_ifexpr(ifexpr, &conversion_table);
                for s in ifcase {
                    self.insert_stmt(&s, &conversion_table)
                }
                for (elseifexpr, elseifcase) in elseifcases {
                    self.insert_ifexpr(elseifexpr, &conversion_table);
                    for s in elseifcase {
                        self.insert_stmt(&s, &conversion_table)
                    }
                }
                for s in elsecase {
                    self.insert_stmt(&s, &conversion_table)
                }
            }
            parse::Statement::Reference { data } => {
                self.insert_dat(data, &conversion_table);
            }
            parse::Statement::ReferenceInd { data, index: _ } => {
                self.insert_dat(data, &conversion_table);
            }
            parse::Statement::NameMulti { idents } => {
                for id in idents {
                    self.insert_ident(&id, &conversion_table);
                }
            }
            parse::Statement::Math { math } => self.insert_math(math, &conversion_table),
            parse::Statement::Assign { ident, data } => {
                self.insert_ident(&ident, &conversion_table);
                self.mutable_idents.insert(ident.clone());
                self.insert_data_or_qi2(data, &conversion_table);
            }
            parse::Statement::AssignInd { ident, index, data } => {
                self.insert_ident(&ident, &conversion_table)
            },
            parse::Statement::Print
            | parse::Statement::Flush
            | parse::Statement::Break
            | parse::Statement::Continue => {}
            parse::Statement::ForEnum { statements, num: _ }
            | parse::Statement::Loop { statements } => {
                self.insert_stmts(&statements, &conversion_table)
            }
            parse::Statement::Declare(parse::DeclareStatement {
                how_many_variables: _,
                type_: _,
                data_arr,
            }) => {
                for dat in data_arr {
                    self.insert_dat(dat, &conversion_table);
                }
            }
            parse::Statement::InitDefine {
                name,
                type_: _,
                data: dat,
            } => {
                self.insert_dat(dat, &conversion_table);
                self.insert_ident(&name, &conversion_table)
            }
            parse::Statement::ForEnumIdent { ident, statements } => {
                self.insert_ident(&ident, &conversion_table);
                self.insert_stmts(&statements, &conversion_table)
            }
            parse::Statement::Define {
                idents,
                decl:
                    parse::DeclareStatement {
                        how_many_variables: _,
                        type_: _,
                        data_arr,
                    },
            } => {
                for dat in data_arr {
                    self.insert_dat(dat, &conversion_table);
                }
                for ident in idents {
                    self.insert_ident(&ident, &conversion_table)
                }
            }
        }
    }
}
