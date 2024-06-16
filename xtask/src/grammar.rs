use std::collections::HashMap;
use std::error::Error;
use std::ops::Index;

use ungrammar::Grammar as Ungrammar;
use ungrammar::Node as UngrammarNode;
use ungrammar::Rule as UngrammarRule;
use ungrammar::Token as UngrammarToken;

pub type GrammarResult<T> = Result<T, Box<dyn Error>>;

pub trait Terminal {
    fn terminal(&self) -> &str;
}

pub struct Grammar<Token> {
    rules: Vec<Rule>,
    tokens: Vec<Token>,
}

impl<Token: Terminal> Grammar<Token> {
    pub fn try_new(ungrammar: &Ungrammar, tokens: Vec<Token>) -> GrammarResult<Self> {
        GrammarBuilder::new().build(ungrammar, tokens)
    }
}

impl<Token> Grammar<Token> {
    pub fn rules(&self) -> impl Iterator<Item = RuleIndex> {
        (0..self.rules.len()).map(|i| RuleIndex(i as u32))
    }

    pub fn tokens(&self) -> impl Iterator<Item = TokenIndex> {
        (0..self.tokens.len()).map(|i| TokenIndex(i as u32))
    }
}

impl<Token> Index<RuleIndex> for Grammar<Token> {
    type Output = Rule;

    fn index(&self, index: RuleIndex) -> &Self::Output {
        &self.rules[index.0 as usize]
    }
}

impl<Token> Index<TokenIndex> for Grammar<Token> {
    type Output = Token;

    fn index(&self, index: TokenIndex) -> &Self::Output {
        &self.tokens[index.0 as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleIndex(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenIndex(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeIndex {
    Rule(RuleIndex),
    Token(TokenIndex),
}

pub struct Rule {
    pub name: String,
    pub kind: RuleKind,
}

pub enum RuleKind {
    Sequence(Vec<SequenceItem>),
    Choice(Vec<NodeIndex>),
    Repeat(NodeIndex, Option<NodeIndex>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequenceItem {
    pub label: Option<String>,
    pub node: NodeIndex,
    pub mandatory: bool,
}

struct GrammarBuilder {
    rules: HashMap<UngrammarNode, RuleIndex>,
    tokens: HashMap<UngrammarToken, TokenIndex>,
}

impl GrammarBuilder {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
            tokens: HashMap::new(),
        }
    }

    fn build<Token: Terminal>(
        mut self,
        ungrammar: &Ungrammar,
        tokens: Vec<Token>,
    ) -> GrammarResult<Grammar<Token>> {
        let tokens = self.build_tokens(ungrammar, tokens)?;
        let rules = self.build_rules(ungrammar)?;
        Ok(Grammar { rules, tokens })
    }

    fn build_tokens<Token: Terminal>(
        &mut self,
        ungrammar: &Ungrammar,
        tokens: Vec<Token>,
    ) -> GrammarResult<Vec<Token>> {
        let mut index = HashMap::new();
        for (i, token) in tokens.iter().enumerate() {
            if let Some(_) = index.insert(token.terminal(), TokenIndex(i as u32)) {
                return Err(format!("duplicate token `{:?}`", token.terminal()).into());
            }
        }

        self.tokens.extend(
            ungrammar
                .tokens()
                .map(|token| {
                    let terminal = ungrammar[token].name.as_str();
                    index
                        .get(terminal)
                        .map(|&index| (token, index))
                        .ok_or_else(|| format!("token `{:?}` not found", terminal))
                })
                .collect::<Result<Vec<_>, _>>()?,
        );

        Ok(tokens)
    }

    fn build_rules(&mut self, ungrammar: &Ungrammar) -> GrammarResult<Vec<Rule>> {
        self.rules.extend(
            ungrammar
                .iter()
                .enumerate()
                .map(|(i, node)| (node, RuleIndex(i as u32))),
        );

        ungrammar
            .iter()
            .map(|node| {
                let node = &ungrammar[node];
                let name = node.name.clone();
                let kind = match &node.rule {
                    rule @ (UngrammarRule::Token(_) | UngrammarRule::Node(_)) => {
                        RuleKind::Choice(vec![self.build_node(rule)?])
                    }
                    rule @ UngrammarRule::Seq(rules) => match self.build_delimited(rule) {
                        Ok(result) => result,
                        Err(_) => {
                            let items = rules
                                .iter()
                                .map(|rule| self.build_sequence_item(rule))
                                .collect::<Result<_, _>>()?;
                            RuleKind::Sequence(items)
                        }
                    },
                    UngrammarRule::Alt(rules) => {
                        let items = rules
                            .iter()
                            .map(|rule| self.build_node(rule))
                            .collect::<Result<_, _>>()?;
                        RuleKind::Choice(items)
                    }
                    UngrammarRule::Rep(rule) => RuleKind::Repeat(self.build_node(rule)?, None),
                    rule => RuleKind::Sequence(vec![self.build_sequence_item(rule)?]),
                };
                Ok(Rule { name, kind })
            })
            .collect::<Result<_, _>>()
    }

    fn build_delimited(&mut self, rule: &UngrammarRule) -> GrammarResult<RuleKind> {
        let mut rules = match rule {
            UngrammarRule::Seq(rules) => rules.iter(),
            _ => return Err(format!("rule `{:?}` cannot be a delimited", rule).into()),
        };

        let element = match rules.next() {
            Some(element) => self.build_node(element)?,
            _ => return Err(format!("rule `{:?}` cannot be a delimited", rule).into()),
        };

        let separator = match rules.next() {
            Some(rule) => match &rule {
                UngrammarRule::Rep(repeat) => match repeat.as_ref() {
                    UngrammarRule::Seq(rules) => {
                        let mut rules = rules.iter();

                        let separator = rules.next()
                            .ok_or_else(|| format!("rule `{:?}` cannot be a delimited", rule).into())
                            .and_then(|rule| self.build_node(rule))?;

                        let second_element = rules.next()
                            .ok_or_else(|| format!("rule `{:?}` cannot be a delimited", rule).into())
                            .and_then(|rule| self.build_node(rule))?;

                        if !rules.next().is_none() {
                            return Err(format!("rule `{:?}` cannot be a delimited", rule).into());
                        }

                        if second_element != element {
                            return Err(format!("rule `{:?}` cannot be a delimited", rule).into());
                        }

                        separator
                    }
                    _ => return Err(format!("rule `{:?}` cannot be a delimited", rule).into()),
                }
                _ => return Err(format!("rule `{:?}` cannot be a delimited", rule).into()),
            }
            _ => return Err(format!("rule `{:?}` cannot be a delimited", rule).into()),
        };

        match rules.next() {
            Some(rule) => match rule {
                UngrammarRule::Opt(rule) => {
                    if self.build_node(rule)? != separator {
                        return Err(format!("rule `{:?}` cannot be a delimited", rule).into());
                    }
                }
                _ => return Err(format!("rule `{:?}` cannot be a delimited", rule).into()),
            }
            _ => (),
        }

        if !rules.next().is_none() {
            return Err(format!("rule `{:?}` cannot be a delimited", rule).into());
        }

        Ok(RuleKind::Repeat(element, Some(separator)))
    }

    fn build_node(&mut self, rule: &UngrammarRule) -> GrammarResult<NodeIndex> {
        match rule {
            UngrammarRule::Node(node) => self
                .rules
                .get(node)
                .map(|&index| NodeIndex::Rule(index))
                .ok_or_else(|| format!("node `{:?}` not found", node).into()),
            UngrammarRule::Token(token) => self
                .tokens
                .get(token)
                .map(|&index| NodeIndex::Token(index))
                .ok_or_else(|| format!("token `{:?}` not found", token).into()),
            _ => Err(format!("rule `{:?}` cannot be a `RuleIndex`", rule).into()),
        }
    }

    fn build_sequence_item(&mut self, rule: &UngrammarRule) -> GrammarResult<SequenceItem> {
        match rule {
            UngrammarRule::Node(_) | UngrammarRule::Token(_) => Ok(SequenceItem {
                label: None,
                node: self.build_node(rule)?,
                mandatory: true,
            }),
            UngrammarRule::Labeled { label, rule } => Ok(SequenceItem {
                label: Some(label.clone()),
                ..self.build_sequence_item(rule)?
            }),
            UngrammarRule::Opt(rule) => Ok(SequenceItem {
                mandatory: false,
                ..self.build_sequence_item(rule)?
            }),
            _ => Err(format!("rule `{:?}` cannot be a `SequenceItem`", rule).into()),
        }
    }
}
