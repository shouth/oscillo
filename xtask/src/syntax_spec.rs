use std::{collections::HashMap, ops::Index};

use ungrammar::{Grammar, Node, Rule, Token};

pub struct SyntaxSpec {
    pub tokens: Vec<TokenSpecData>,
    pub rules: Vec<RuleSpecData>,
}

pub struct RuleSpecData {
    pub name: String,
    pub kind: RuleSpecKind,
}

pub enum RuleSpecKind {
    Aggregate {
        fields: Vec<FieldSpecData>,
    },
    List {
        separator: Option<TokenSpec>,
        element: RuleSpec,
    },
    Variant {
        variants: Vec<FieldSpecData>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleSpec(usize);

pub struct TokenSpecData {
    pub token: String,
    pub kind: SyntaxTokenSpecKind,
}

pub enum SyntaxTokenSpecKind {
    Punct { name: String },
    Keyword,
    Literal,
    Token,
    Trivial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenSpec(usize);

pub struct FieldSpecData {
    pub label: Option<String>,
    pub mandatory: bool,
    pub kind: FieldSpecKind,
}

pub enum FieldSpecKind {
    Rule(RuleSpec),
    Token(TokenSpec),
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
    token_set: &'a TokenSet<'a>,
    rule_to_spec: HashMap<Node, RuleSpec>,
    str_to_spec: HashMap<String, TokenSpec>,
}

impl<'a> SyntaxSpecBuilder<'a> {
    pub fn new(grammar: &'a Grammar, token_set: &'a TokenSet) -> Self {
        let mut rule_to_spec = HashMap::new();
        for node in grammar.iter() {
            rule_to_spec.insert(node, RuleSpec(rule_to_spec.len()));
        }

        let str_to_spec = std::iter::empty()
            .chain(
                token_set
                    .punctuations
                    .iter()
                    .map(|(name, _)| name.to_owned()),
            )
            .chain(token_set.keywords.iter().map(|name| name.to_owned()))
            .chain(token_set.literals.iter().map(|name| name.to_owned()))
            .chain(token_set.tokens.iter().map(|name| name.to_owned()))
            .chain(token_set.trivials.iter().map(|name| name.to_owned()))
            .enumerate()
            .map(|(i, name)| (name.to_owned(), TokenSpec(i)))
            .collect::<HashMap<_, _>>();

        Self {
            grammar,
            token_set,
            rule_to_spec,
            str_to_spec,
        }
    }

    pub fn build(&self) -> Result<SyntaxSpec, Box<dyn std::error::Error>> {
        let tokens = self.generate_tokens();
        let rules = self.generate_rules()?;
        Ok(SyntaxSpec { tokens, rules })
    }

    fn generate_tokens(&self) -> Vec<TokenSpecData> {
        std::iter::empty()
            .chain(self.token_set.punctuations.iter().map(|(token, name)| {
                (
                    String::from(*token),
                    SyntaxTokenSpecKind::Punct {
                        name: String::from(*name),
                    },
                )
            }))
            .chain(
                self.token_set
                    .keywords
                    .iter()
                    .map(|token| (String::from(*token), SyntaxTokenSpecKind::Keyword)),
            )
            .chain(
                self.token_set
                    .literals
                    .iter()
                    .map(|token| (String::from(*token), SyntaxTokenSpecKind::Literal)),
            )
            .chain(
                self.token_set
                    .tokens
                    .iter()
                    .map(|token| (String::from(*token), SyntaxTokenSpecKind::Token)),
            )
            .chain(
                self.token_set
                    .trivials
                    .iter()
                    .map(|token| (String::from(*token), SyntaxTokenSpecKind::Trivial)),
            )
            .map(|(token, kind)| TokenSpecData { token, kind })
            .collect::<Vec<_>>()
    }

    fn generate_rules(&self) -> Result<Vec<RuleSpecData>, Box<dyn std::error::Error>> {
        let mut rules = Vec::new();
        for rule in self.grammar.iter() {
            let rule_spec = match self.grammar[rule].rule {
                _ if self.grammar[rule].name.ends_with("List") => self.check_list(rule),
                Rule::Seq(_) => self.check_aggregate(rule),
                Rule::Alt(_) => self.check_variant(rule),
                _ => Err(format!("Unexpected rule {:?}", rule).into()),
            }?;
            rules.push(rule_spec);
        }
        Ok(rules)
    }

    fn check_token(&self, token: &Token) -> Result<TokenSpec, Box<dyn std::error::Error>> {
        let token = &self.grammar[*token];
        match self.str_to_spec.get(&token.name) {
            Some(spec) => Ok(*spec),
            None => Err(format!("Unknown token {:?}", token.name).into()),
        }
    }

    fn is_mandatory(&self, rule: &Rule) -> bool {
        match rule {
            Rule::Opt(_) => false,
            _ => true,
        }
    }

    fn get_label(&self, rule: &Rule) -> Option<String> {
        match rule {
            Rule::Labeled { label, .. } => Some(label.clone()),
            _ => None,
        }
    }

    fn get_field_kind(&self, rule: &Rule) -> Result<FieldSpecKind, Box<dyn std::error::Error>> {
        match rule {
            Rule::Node(node) => Ok(FieldSpecKind::Rule(self.rule_to_spec[node])),
            Rule::Token(token) => Ok(FieldSpecKind::Token(self.check_token(token)?)),
            Rule::Opt(rule) => self.get_field_kind(&rule),
            Rule::Labeled { rule, .. } => self.get_field_kind(rule),
            _ => Err(format!("Unexpected rule {:?}", rule).into()),
        }
    }

    fn check_aggregate(&self, node: Node) -> Result<RuleSpecData, Box<dyn std::error::Error>> {
        let node = &self.grammar[node];
        let Rule::Seq(rules) = &node.rule else {
            unreachable!("Expected sequence of rules");
        };

        let mut fields = Vec::new();
        for rule in rules {
            let label = self.get_label(rule);
            let mandatory = self.is_mandatory(rule);
            let kind = self.get_field_kind(rule)?;
            fields.push(FieldSpecData {
                label,
                mandatory,
                kind,
            });
        }

        Ok(RuleSpecData {
            name: node.name.clone(),
            kind: RuleSpecKind::Aggregate { fields },
        })
    }

    fn check_list(&self, node: Node) -> Result<RuleSpecData, Box<dyn std::error::Error>> {
        let node = &self.grammar[node];

        if !node.name.ends_with("List") {
            unreachable!("Expected list node");
        }

        todo!("Implement list node")
    }

    fn check_variant(&self, node: Node) -> Result<RuleSpecData, Box<dyn std::error::Error>> {
        let node = &self.grammar[node];
        let Rule::Alt(rules) = &node.rule else {
            unreachable!("Expected alternative of rules");
        };

        let mut variants = Vec::new();
        for rule in rules {
            let label = self.get_label(rule);
            let mandatory = self.is_mandatory(rule);
            let kind = self.get_field_kind(rule)?;
            variants.push(FieldSpecData {
                label,
                mandatory,
                kind,
            });
        }

        Ok(RuleSpecData {
            name: node.name.clone(),
            kind: RuleSpecKind::Variant { variants },
        })
    }
}

impl TryFrom<(&Grammar, &TokenSet<'_>)> for SyntaxSpec {
    type Error = Box<dyn std::error::Error>;

    fn try_from((grammar, token_set): (&Grammar, &TokenSet)) -> Result<Self, Self::Error> {
        SyntaxSpecBuilder::new(grammar, token_set).build()
    }
}
