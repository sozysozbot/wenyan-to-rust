use crate::parse;
use bimap::BiMap;
use std::collections::HashMap;
use std::collections::HashSet;

type Table = HashMap<String, String>;

fn to_pinyin(ident: parse::Identifier, conversion_table: &Table) -> String {
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

    pub fn new(parsed: &[parse::Statement], conversion_table: &Table) -> Self {
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

    fn insert_ident(&mut self, ident: &parse::Identifier, conversion_table: &Table) {
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

    fn insert_stmts(&mut self, statements: &[parse::Statement], conversion_table: &Table) {
        for s in statements {
            self.insert_stmt(&s, &conversion_table)
        }
    }

    fn insert_dat(&mut self, dat: &parse::Data, conversion_table: &Table) {
        if let parse::Data::Identifier(id) = dat {
            self.insert_ident(&id, &conversion_table)
        }
    }

    fn insert_data_or_qi2(&mut self, dat: &parse::DataOrQi2, conversion_table: &Table) {
        if let parse::DataOrQi2::Data(d1) = dat {
            self.insert_dat(d1, &conversion_table);
        }
    }

    fn insert_rvaluenoqi2(&mut self, val: &parse::RvalueNoQi2, conversion_table: &Table) {
        match val {
            parse::RvalueNoQi2::Index(data, _)
            | parse::RvalueNoQi2::Simple(data)
            | parse::RvalueNoQi2::Length(data) => self.insert_dat(data, &conversion_table),
            parse::RvalueNoQi2::IndexByIdent(data, ident) => {
                self.insert_dat(data, &conversion_table);
                self.insert_ident(ident, &conversion_table)
            }
        }
    }

    fn insert_unaryifexpr(&mut self, unary: &parse::UnaryIfExpr, conversion_table: &Table) {
        match unary {
            parse::UnaryIfExpr::Simple(data) => self.insert_data_or_qi2(data, &conversion_table),
            parse::UnaryIfExpr::Complex(val) => self.insert_rvaluenoqi2(val, &conversion_table),
        }
    }

    fn insert_ifexpr(&mut self, ifexpr: &parse::IfCond, conversion_table: &Table) {
        match ifexpr {
            parse::IfCond::Binary(data1, _, data2) => {
                self.insert_unaryifexpr(data1, &conversion_table);
                self.insert_unaryifexpr(data2, &conversion_table);
            }
            parse::IfCond::Unary(data) => {
                self.insert_unaryifexpr(data, &conversion_table);
            }
            parse::IfCond::NotQi2 => {}
        }
    }

    fn insert_math(&mut self, math: &parse::MathKind, conversion_table: &Table) {
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
    fn insert_rvalue(&mut self, rv: &parse::Rvalue, conversion_table: &Table) {
        match rv {
            parse::Rvalue::Index(data, _)
            | parse::Rvalue::Length(data)
            | parse::Rvalue::Simple(data) => self.insert_data_or_qi2(data, &conversion_table),
        }
    }
    fn insert_idents(&mut self, idents: &[parse::Identifier], conversion_table: &Table) {
        for ident in idents {
            self.insert_ident(&ident, &conversion_table)
        }
    }
    fn insert_dats(&mut self, data_arr: &[parse::Data], conversion_table: &Table) {
        for dat in data_arr {
            self.insert_dat(dat, &conversion_table);
        }
    }
    fn insert_stmt(&mut self, st: &parse::Statement, conversion_table: &Table) {
        use parse::Statement::*;
        match st {
            ReferenceWhatIsLeft { data } => {
                self.insert_dat(&data, &conversion_table);
            }
            ForArr { list, elem, stmts } => {
                self.insert_ident(&list, &conversion_table);
                self.insert_ident(&elem, &conversion_table);
                self.insert_stmts(&stmts, &conversion_table)
            }
            ArrayCat { append_to, elems } => {
                self.insert_data_or_qi2(&parse::DataOrQi2::from(append_to), &conversion_table);
                self.insert_idents(&elems, &conversion_table)
            }
            ArrayFill {
                what_to_fill,
                elems,
            } => {
                self.insert_data_or_qi2(&parse::DataOrQi2::from(what_to_fill), &conversion_table);
                if let parse::IdentOrQi2::Ident(ident) = what_to_fill {
                    self.mutable_idents.insert(ident.clone());
                }
                self.insert_dats(&elems, &conversion_table);
            }
            If {
                ifcase: (ifexpr, ifcase),
                elseifcases,
                elsecase,
            } => {
                self.insert_ifexpr(ifexpr, &conversion_table);
                self.insert_stmts(&ifcase, &conversion_table);
                for (elseifexpr, elseifcase) in elseifcases {
                    self.insert_ifexpr(elseifexpr, &conversion_table);
                    self.insert_stmts(&elseifcase, &conversion_table)
                }
                self.insert_stmts(&elsecase, &conversion_table)
            }
            Reference { rvalue } => self.insert_rvaluenoqi2(rvalue, &conversion_table),
            NameMulti { idents } => self.insert_idents(&idents, &conversion_table),
            Math { math } => self.insert_math(math, &conversion_table),
            Assignment {
                lvalue: parse::Lvalue::Simple(ident),
                rvalue,
            }
            | Assignment {
                lvalue: parse::Lvalue::Index(ident, _),
                rvalue,
            } => {
                self.insert_ident(&ident, &conversion_table);
                self.mutable_idents.insert(ident.clone());
                self.insert_rvalue(rvalue, &conversion_table)
            }
            Assignment {
                lvalue: parse::Lvalue::IndexByIdent(ident, index),
                rvalue,
            } => {
                self.insert_ident(&ident, &conversion_table);
                self.mutable_idents.insert(ident.clone());
                self.insert_ident(&index, &conversion_table);
                self.insert_rvalue(rvalue, &conversion_table)
            }
            Print | Flush | Break | Continue => {}
            ForEnum { statements, num: _ } | Loop { statements } => {
                self.insert_stmts(&statements, &conversion_table)
            }
            Declare(parse::DeclareStatement {
                how_many_variables: _,
                type_: _,
                data_arr,
            }) => self.insert_dats(data_arr, &conversion_table),
            InitDefine {
                name,
                type_: _,
                data: dat,
            } => {
                self.insert_dat(dat, &conversion_table);
                self.insert_ident(&name, &conversion_table)
            }
            ForEnumIdent { ident, statements } => {
                if let parse::IdentOrQi2::Ident(i) = ident {
                    self.insert_ident(&i, &conversion_table);
                }
                self.insert_stmts(&statements, &conversion_table)
            }
            Define {
                idents,
                decl:
                    parse::DeclareStatement {
                        how_many_variables: _,
                        type_: _,
                        data_arr,
                    },
            } => {
                self.insert_dats(data_arr, &conversion_table);
                self.insert_idents(&idents, &conversion_table)
            }
        }
    }
}
