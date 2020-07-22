use crate::parse;
use bimap::BiMap;
use std::collections::HashMap;
use std::collections::HashSet;

fn to_pinyin(ident: parse::Identifier, conversion_table: &HashMap<String, String>) -> String {
    let parse::Identifier(i) = ident;
    let vec = i
        .chars()
        .map(
            |c| match conversion_table.get(&format!("{:X}", c as u32).to_string()) {
                None => "_".to_string(),
                Some(a) => a.split(" ").collect::<Vec<_>>()[0].to_string(),
            },
        )
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

    pub fn new(parsed: &Vec<parse::Statement>, conversion_table: &HashMap<String, String>) -> Self {
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
        ident: parse::Identifier,
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
                self.bimap.insert(ident, candidate);
                break;
            }
        }
    }

    fn insert_dat(&mut self, dat: &parse::Data, conversion_table: &HashMap<String, String>) {
        if let parse::Data::Identifier(id) = dat {
            self.insert_ident(id.clone(), &conversion_table)
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

    fn insert_stmt(&mut self, st: &parse::Statement, conversion_table: &HashMap<String, String>) {
        match st {
            parse::Statement::If(
                (parse::IfExpression::Binary(data1, _, data2), ifcase),
                elsecase,
            ) => {
                self.insert_data_or_qi2(data1, &conversion_table);
                self.insert_data_or_qi2(data2, &conversion_table);
                for s in ifcase {
                    self.insert_stmt(&s, &conversion_table)
                }
                for s in elsecase {
                    self.insert_stmt(&s, &conversion_table)
                }
            }
            parse::Statement::If((parse::IfExpression::Unary(data), ifcase), elsecase) => {
                self.insert_data_or_qi2(data, &conversion_table);
                for s in ifcase {
                    self.insert_stmt(&s, &conversion_table)
                }
                for s in elsecase {
                    self.insert_stmt(&s, &conversion_table)
                }
            }
            parse::Statement::Reference { data, ident } => {
                self.insert_dat(data, &conversion_table);
                if let Some(i) = ident {
                    self.insert_ident(i.clone(), &conversion_table)
                }
            }
            parse::Statement::NameMulti { idents } => {
                for id in idents {
                    self.insert_ident(id.clone(), &conversion_table);
                }
            }
            parse::Statement::Math {
                math: parse::MathKind::ArithBinaryMath(_, data1, _, data2),
            }
            | parse::Statement::Math {
                math: parse::MathKind::ModMath(_, data1, _, data2),
            } => {
                self.insert_data_or_qi2(data1, &conversion_table);
                self.insert_data_or_qi2(data2, &conversion_table);
            }
            parse::Statement::Math {
                math: parse::MathKind::BooleanAlgebra(ident1, ident2, _),
            } => {
                self.insert_ident(ident1.clone(), &conversion_table);
                self.insert_ident(ident2.clone(), &conversion_table);
            }
            parse::Statement::Assign { ident, data } => {
                self.insert_ident(ident.clone(), &conversion_table);
                self.mutable_idents.insert(ident.clone());
                self.insert_data_or_qi2(data, &conversion_table);
            }
            parse::Statement::Print | parse::Statement::Flush => {}
            parse::Statement::ForEnum { statements, num: _ } => {
                for s in statements {
                    self.insert_stmt(&s, &conversion_table)
                }
            }
            parse::Statement::Declare {
                0:
                    parse::DeclareStatement {
                        how_many_variables: _,
                        type_: _,
                        data_arr,
                    },
            } => {
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
                self.insert_ident(name.clone(), &conversion_table)
            }
            parse::Statement::ForEnumIdent { ident, statements } => {
                self.insert_ident(ident.clone(), &conversion_table);
                for s in statements {
                    self.insert_stmt(&s, &conversion_table)
                }
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
                    self.insert_ident(ident.clone(), &conversion_table)
                }
            }
        }
    }
}
