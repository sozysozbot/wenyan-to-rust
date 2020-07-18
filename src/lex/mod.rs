#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Lex {
    /// 吾有
    Wu2You3,
    /// 今
    Jin1,
    /// 今有
    Jin1You3,
    Type(Type),
    /// 曰
    Yue1,

    /// 書之
    Shu1Zhi1,

    /// 吾嘗觀
    Wu2Chang2Guan1,
    StringLiteral(String),
    BoolValue(BoolValue),
    Identifier(String),
    IntNum(IntNum),
    FloatNumKeywords(FloatNumKeywords),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IntNum(Vec<IntNumKeywords>);

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum FloatNumKeywords {
    /// 分
    Fen1,
    /// 釐
    Li2,
    /// 毫
    Hao2,
    /// 絲
    Si1,
    /// 忽
    Hu1,
    /// 微
    Wei1,
    /// 纖
    Xian1,
    /// 沙
    Sha1,
    /// 塵
    Chen2,
    /// 埃
    Ai1,
    /// 渺
    Miao3,
    /// 漠
    Mo4,
}
impl FloatNumKeywords {
    fn from_char(c: char) -> Option<FloatNumKeywords> {
        match c {
            '分' => Some(FloatNumKeywords::Fen1),
            '釐' => Some(FloatNumKeywords::Li2),
            '毫' => Some(FloatNumKeywords::Hao2),
            '絲' => Some(FloatNumKeywords::Si1),
            '忽' => Some(FloatNumKeywords::Hu1),
            '微' => Some(FloatNumKeywords::Wei1),
            '纖' => Some(FloatNumKeywords::Xian1),
            '沙' => Some(FloatNumKeywords::Sha1),
            '塵' => Some(FloatNumKeywords::Chen2),
            '埃' => Some(FloatNumKeywords::Ai1),
            '渺' => Some(FloatNumKeywords::Miao3),
            '漠' => Some(FloatNumKeywords::Mo4),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum IntNumKeywords {
    /// 零
    Ling2,
    /// 一
    Yi1,
    /// 二
    Er4,
    /// 三
    San1,
    /// 四
    Si4,
    /// 五
    Wu3,
    /// 六
    Liu4,
    /// 七
    Qi1,
    /// 八
    Ba1,
    /// 九
    Jiu3,
    /// 十
    Shi2,
    /// 百
    Bai3,
    /// 千
    Qian1,
    /// 萬
    Wan4,
    /// 億
    Yi4,
    /// 兆
    Zhao4,
    /// 京
    Jing1,
    /// 垓
    Gai1,
    /// 秭
    Zi3,
    /// 穣
    Rang2,
    /// 溝
    Gou1,
    /// 澗
    Jian4,
    /// 正
    Zheng4,
    /// 載
    Zai4,
    /// 極
    Ji2,
}

impl IntNumKeywords {
    fn from_char(c: char) -> Option<IntNumKeywords> {
        match c {
            '零' => Some(IntNumKeywords::Ling2),
            '一' => Some(IntNumKeywords::Yi1),
            '二' => Some(IntNumKeywords::Er4),
            '三' => Some(IntNumKeywords::San1),
            '四' => Some(IntNumKeywords::Si4),
            '五' => Some(IntNumKeywords::Wu3),
            '六' => Some(IntNumKeywords::Liu4),
            '七' => Some(IntNumKeywords::Qi1),
            '八' => Some(IntNumKeywords::Ba1),
            '九' => Some(IntNumKeywords::Jiu3),
            '十' => Some(IntNumKeywords::Shi2),
            '百' => Some(IntNumKeywords::Bai3),
            '千' => Some(IntNumKeywords::Qian1),
            '萬' => Some(IntNumKeywords::Wan4),
            '億' => Some(IntNumKeywords::Yi4),
            '兆' => Some(IntNumKeywords::Zhao4),
            '京' => Some(IntNumKeywords::Jing1),
            '垓' => Some(IntNumKeywords::Gai1),
            '秭' => Some(IntNumKeywords::Zi3),
            '穣' => Some(IntNumKeywords::Rang2),
            '溝' => Some(IntNumKeywords::Gou1),
            '澗' => Some(IntNumKeywords::Jian4),
            '正' => Some(IntNumKeywords::Zheng4),
            '載' => Some(IntNumKeywords::Zai4),
            '極' => Some(IntNumKeywords::Ji2),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum BoolValue {
    /// 陰
    Yin1,
    /// 陽
    Yang2,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Type {
    /// 數
    Shu4,
    /// 列
    Lie4,
    /// 言
    Yan2,
    /// 爻
    Yao2,
}

use peek_nth::IteratorExt;

#[derive(Debug)]
pub enum LexError {
    UnexpectedCharAfter(char, char),
    UnexpectedEOFAfter(char),
    NonterminatedIdentifier,
    EmptyIdentifier,
    NonterminatedStringLiteral,
}

pub fn lex(input: &str) -> Result<Vec<Lex>, LexError> {
    let mut ans = vec![];
    let mut iter = input.chars().peekable_nth();
    loop {
        let c = match iter.next() {
            None => break,
            Some(d) => d,
        };

        if c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '。' || c == '、' {
            continue;
        }

        match c {
            '數' => ans.push(Lex::Type(Type::Shu4)),
            '列' => ans.push(Lex::Type(Type::Lie4)),
            '言' => ans.push(Lex::Type(Type::Yan2)),
            '爻' => ans.push(Lex::Type(Type::Yao2)),
            '曰' => ans.push(Lex::Yue1),
            '陰' => ans.push(Lex::BoolValue(BoolValue::Yin1)),
            '陽' => ans.push(Lex::BoolValue(BoolValue::Yang2)),
            '「' => {
                let peek = iter.peek();
                match peek {
                    None => return Err(LexError::NonterminatedIdentifier),
                    Some('「') => {
                        iter.next(); /* parse string literal */
                        let mut strlit = String::new();
                        loop {
                            let next = iter.next();
                            match next {
                                None => return Err(LexError::NonterminatedStringLiteral),
                                Some('」') => match iter.next() {
                                    None => return Err(LexError::NonterminatedStringLiteral),
                                    Some('」') => break,
                                    Some(a) => return Err(LexError::UnexpectedCharAfter('」', a)),
                                },
                                Some(a) => strlit.push(a),
                            }
                        }
                        ans.push(Lex::StringLiteral(strlit));
                    }
                    Some(_) => {
                        /* parse identifier */
                        let mut ident = String::new();
                        loop {
                            match iter.next() {
                                None => return Err(LexError::NonterminatedIdentifier),
                                Some('」') => break,
                                Some(a) => ident.push(a),
                            }
                        }

                        if ident.is_empty() {
                            return Err(LexError::EmptyIdentifier);
                        }

                        ans.push(Lex::Identifier(ident));
                    }
                }
            }
            '吾' => {
                let next = iter.next();
                match next {
                    None => return Err(LexError::UnexpectedEOFAfter('吾')),
                    Some('有') => ans.push(Lex::Wu2You3),
                    Some('嘗') => match iter.next() {
                        Some('觀') => ans.push(Lex::Wu2Chang2Guan1),
                        None => return Err(LexError::UnexpectedEOFAfter('嘗')),
                        Some(a) => return Err(LexError::UnexpectedCharAfter('嘗', a)),
                    },
                    Some(a) => return Err(LexError::UnexpectedCharAfter('吾', a)),
                }
            }
            '書' => {
                let next = iter.next();
                match next {
                    None => return Err(LexError::UnexpectedEOFAfter('書')),
                    Some('之') => ans.push(Lex::Shu1Zhi1),
                    Some(a) => return Err(LexError::UnexpectedCharAfter('書', a)),
                }
            }
            '今' => {
                let peek = iter.peek();
                match peek {
                    Some('有') => {
                        iter.next();
                        ans.push(Lex::Jin1You3)
                    }
                    _ => ans.push(Lex::Jin1),
                }
            }
            '零' | '一' | '二' | '三' | '四' | '五' | '六' | '七' | '八' | '九' | '十' | '百'
            | '千' | '萬' | '億' | '兆' | '京' | '垓' | '秭' | '穣' | '溝' | '澗' | '正' | '載'
            | '極' => {
                let mut vec = vec![];
                vec.push(IntNumKeywords::from_char(c).expect("Cannot happen"));
                loop {
                    let k = iter.peek();
                    let c2 = match k {
                        None => break,
                        Some(a) => *a,
                    };
                    let word = match IntNumKeywords::from_char(c2) {
                        None => break,
                        Some(key) => key,
                    };
                    vec.push(word);
                    iter.next();
                }
                ans.push(Lex::IntNum(IntNum(vec)));
            }
            '分' | '釐' | '毫' | '絲' | '忽' | '微' | '纖' | '沙' | '塵' | '埃' | '渺' | '漠' => {
                ans.push(Lex::FloatNumKeywords(
                    FloatNumKeywords::from_char(c).expect("Cannot happen"),
                ))
            }
            _ => unimplemented!(),
        }
    }
    Ok(ans)
}
