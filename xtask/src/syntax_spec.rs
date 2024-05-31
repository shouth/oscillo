use std::{collections::HashMap, ops::Index};

use ungrammar::{Grammar, Node, NodeData, Rule, TokenData};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxSpec {
    pub tokens: Vec<TokenSpecData>,
    pub rules: Vec<RuleSpecData>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleSpecData {
    pub name: String,
    pub kind: RuleSpecKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleSpec(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenSpecData {
    pub token: String,
    pub kind: TokenSpecKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenSpecKind {
    Punct { name: String },
    Keyword,
    Literal,
    Token,
    Trivial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenSpec(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuleSpecKind {
    Aggregate(Vec<AggregateItem>),
    List {
        element: RuleSpec,
    },
    SeparatedList {
        separator: TokenSpec,
        element: RuleSpec,
    },
    TokenVariant(Vec<TokenVariantItem>),
    RuleVariant(Vec<RuleVariantItem>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AggregateItem {
    pub label: Option<String>,
    pub mandatory: bool,
    pub inner: RuleOrToken,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuleOrToken {
    Rule(RuleSpec),
    Token(TokenSpec),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenVariantItem {
    pub label: Option<String>,
    pub token: TokenSpec,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleVariantItem {
    pub label: Option<String>,
    pub rule: RuleSpec,
}

impl Index<RuleSpec> for SyntaxSpec {
    type Output = RuleSpecData;

    fn index(&self, index: RuleSpec) -> &Self::Output {
        &self.rules[index.0]
    }
}

impl Index<TokenSpec> for SyntaxSpec {
    type Output = TokenSpecData;

    fn index(&self, index: TokenSpec) -> &Self::Output {
        &self.tokens[index.0]
    }
}

pub struct TokenSet<'a> {
    pub punctuations: &'a [(&'a str, &'a str)],
    pub keywords: &'a [&'a str],
    pub literals: &'a [&'a str],
    pub tokens: &'a [&'a str],
    pub trivials: &'a [&'a str],
}

struct SyntaxSpecBuilder<'a> {
    grammar: &'a Grammar,
    tokens: &'a TokenSet<'a>,
    rule_to_spec: HashMap<Node, RuleSpec>,
    str_to_spec: HashMap<String, TokenSpec>,
}

impl<'a> SyntaxSpecBuilder<'a> {
    pub fn new(grammar: &'a Grammar, token_set: &'a TokenSet) -> Self {
        Self {
            grammar,
            tokens: token_set,
            rule_to_spec: Default::default(),
            str_to_spec: Default::default(),
        }
    }

    pub fn build(&mut self) -> Result<SyntaxSpec, Box<dyn std::error::Error>> {
        let tokens = self.generate_tokens()?;
        let rules = self.generate_rules()?;
        Ok(SyntaxSpec { tokens, rules })
    }

    fn generate_tokens(&mut self) -> Result<Vec<TokenSpecData>, Box<dyn std::error::Error>> {
        self.str_to_spec.extend(
            std::iter::empty()
                .chain(self.tokens.punctuations.iter().map(|(token, _)| token))
                .chain(self.tokens.keywords.iter())
                .chain(self.tokens.literals.iter())
                .chain(self.tokens.tokens.iter())
                .chain(self.tokens.trivials.iter())
                .enumerate()
                .map(|(i, name)| ((*name).to_owned(), TokenSpec(i))),
        );

        for token in self.grammar.tokens() {
            let TokenData { name, .. } = &self.grammar[token];
            self.str_to_spec
                .get(name)
                .ok_or(format!("Unknown token {:?}", name))?;
        }

        Ok(std::iter::empty()
            .chain(self.tokens.punctuations.iter().map(|(token, name)| {
                (
                    token,
                    TokenSpecKind::Punct {
                        name: String::from(*name),
                    },
                )
            }))
            .chain(
                self.tokens
                    .keywords
                    .iter()
                    .map(|token| (token, TokenSpecKind::Keyword)),
            )
            .chain(
                self.tokens
                    .literals
                    .iter()
                    .map(|token| (token, TokenSpecKind::Literal)),
            )
            .chain(
                self.tokens
                    .tokens
                    .iter()
                    .map(|token| (token, TokenSpecKind::Token)),
            )
            .chain(
                self.tokens
                    .trivials
                    .iter()
                    .map(|token| (token, TokenSpecKind::Trivial)),
            )
            .map(|(token, kind)| TokenSpecData {
                token: (*token).to_owned(),
                kind,
            })
            .collect::<Vec<_>>())
    }

    fn generate_rules(&mut self) -> Result<Vec<RuleSpecData>, Box<dyn std::error::Error>> {
        for (i, node) in self.grammar.iter().enumerate() {
            self.rule_to_spec.insert(node, RuleSpec(i));
        }

        let mut rules = Vec::new();
        for rule in self.grammar.iter() {
            let NodeData { name, rule } = &self.grammar[rule];
            let name = name.clone();
            let kind = match rule {
                Rule::Seq(rules) => self
                    .try_generate_separated_list_rule(rules.as_slice())
                    .or_else(|_| self.try_generate_list_rule(rules.as_slice()))?,
                Rule::Alt(rules) => self
                    .try_generate_token_variant(rules.as_slice())
                    .or_else(|_| self.try_generate_rule_variant(rules.as_slice()))?,
                Rule::Labeled { .. } | Rule::Node(_) | Rule::Token(_) | Rule::Opt(_) => {
                    RuleSpecKind::Aggregate(vec![self.generate_field(&rule)?])
                }
                Rule::Rep(rule) => match rule.as_ref() {
                    Rule::Node(rule) => RuleSpecKind::List {
                        element: self.rule_to_spec[&rule],
                    },
                    _ => return Err(format!("Expected element rule").into()),
                },
            };
            rules.push(RuleSpecData { name, kind });
        }
        Ok(rules)
    }

    fn try_generate_separated_list_rule(
        &self,
        rules: &[Rule],
    ) -> Result<RuleSpecKind, Box<dyn std::error::Error>> {
        let element = match rules.get(0) {
            Some(Rule::Node(rule)) => rule,
            _ => return Err(format!("Expected element rule").into()),
        };

        let separator = match rules.get(1) {
            Some(Rule::Rep(rule)) => match rule.as_ref() {
                Rule::Seq(rules) => match rules.as_slice() {
                    [Rule::Token(token), Rule::Node(rule)] if rule == element => token,
                    _ => return Err(format!("Expected separator rule").into()),
                },
                _ => return Err(format!("Expected separator rule").into()),
            },
            _ => return Err(format!("Expected separator rule").into()),
        };

        match rules.get(2) {
            Some(Rule::Opt(rule)) => match rule.as_ref() {
                Rule::Token(token) if token != separator => {
                    return Err(format!("Expected end token").into())
                }
                _ => {}
            },
            Some(_) => return Err(format!("Expected end token").into()),
            None => {}
        }

        Ok(RuleSpecKind::SeparatedList {
            separator: self.str_to_spec[&self.grammar[*separator].name],
            element: self.rule_to_spec[element],
        })
    }

    fn try_generate_list_rule(
        &self,
        rules: &[Rule],
    ) -> Result<RuleSpecKind, Box<dyn std::error::Error>> {
        let mut fields = Vec::new();
        for rule in rules.iter() {
            fields.push(self.generate_field(&rule)?);
        }
        Ok(RuleSpecKind::Aggregate(fields))
    }

    fn try_generate_token_variant(
        &self,
        tokens: &[Rule],
    ) -> Result<RuleSpecKind, Box<dyn std::error::Error>> {
        let mut variants = Vec::new();
        for token in tokens.iter() {
            match token {
                Rule::Token(token) => variants.push(TokenVariantItem {
                    label: None,
                    token: self.str_to_spec[&self.grammar[*token].name],
                }),
                Rule::Labeled { rule, label } => match rule.as_ref() {
                    Rule::Token(token) => variants.push(TokenVariantItem {
                        label: Some(label.clone()),
                        token: self.str_to_spec[&self.grammar[*token].name],
                    }),
                    _ => return Err(format!("Expected token rule").into()),
                },
                _ => return Err(format!("Expected token rule").into()),
            }
        }
        Ok(RuleSpecKind::TokenVariant(variants))
    }

    fn try_generate_rule_variant(
        &self,
        rules: &[Rule],
    ) -> Result<RuleSpecKind, Box<dyn std::error::Error>> {
        let mut variants = Vec::new();
        for rule in rules.iter() {
            match rule {
                Rule::Node(rule) => variants.push(RuleVariantItem {
                    label: None,
                    rule: self.rule_to_spec[rule],
                }),
                Rule::Labeled { rule, label } => match rule.as_ref() {
                    Rule::Node(rule) => variants.push(RuleVariantItem {
                        label: Some(label.clone()),
                        rule: self.rule_to_spec[rule],
                    }),
                    _ => return Err(format!("Expected node rule").into()),
                },
                _ => return Err(format!("Expected node rule").into()),
            }
        }
        Ok(RuleSpecKind::RuleVariant(variants))
    }

    fn generate_field(&self, rule: &Rule) -> Result<AggregateItem, Box<dyn std::error::Error>> {
        match rule {
            Rule::Node(node) => Ok(AggregateItem {
                label: None,
                mandatory: true,
                inner: RuleOrToken::Rule(self.rule_to_spec[node]),
            }),
            Rule::Token(token) => Ok(AggregateItem {
                label: None,
                mandatory: true,
                inner: RuleOrToken::Token(self.str_to_spec[&self.grammar[*token].name]),
            }),
            Rule::Opt(rule) => Ok(AggregateItem {
                mandatory: false,
                ..self.generate_field(&rule)?
            }),
            Rule::Labeled { rule, label } => Ok(AggregateItem {
                label: Some(label.clone()),
                ..self.generate_field(&rule)?
            }),
            _ => Err(format!("Unexpected rule {:?}", rule).into()),
        }
    }
}

impl TryFrom<(&Grammar, &TokenSet<'_>)> for SyntaxSpec {
    type Error = Box<dyn std::error::Error>;

    fn try_from((grammar, token_set): (&Grammar, &TokenSet)) -> Result<Self, Self::Error> {
        SyntaxSpecBuilder::new(grammar, token_set).build()
    }
}
