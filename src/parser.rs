use std::str::FromStr;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    assembler::AssemblerError,
    ast::{ASTArg, ASTNode},
    register::Register,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ASTParser;

impl ASTParser {
    pub fn parse_file(input: &str) -> Result<Vec<ASTNode>, AssemblerError> {
        let unparsed_file = std::fs::read_to_string(input)?;

        let mut parser = Self::parse(Rule::file, &unparsed_file)?;

        let file = parser.next().unwrap();

        Self::parse_inner(file)
    }

    fn parse_inner(rule: Pair<Rule>) -> Result<Vec<ASTNode>, AssemblerError> {
        let mut ast = Vec::new();

        for node in rule.into_inner() {
            match node.as_rule() {
                Rule::binaryins => {
                    let mut inner = node.into_inner();
                    let op = inner.next().unwrap().as_str().to_lowercase();
                    let left = Self::parse_value(inner.next().unwrap())?;
                    let right = Self::parse_value(inner.next().unwrap())?;
                    match op.as_str() {
                        "mov" => ast.push(ASTNode::Mov(left, right)),
                        "add" => ast.push(ASTNode::Add(left, right)),
                        "sub" => ast.push(ASTNode::Sub(left, right)),
                        "mul" => ast.push(ASTNode::Mul(left, right)),
                        "shl" => ast.push(ASTNode::Shl(left, right)),
                        "shr" => ast.push(ASTNode::Shr(left, right)),
                        "and" => ast.push(ASTNode::And(left, right)),
                        "or" => ast.push(ASTNode::Or(left, right)),
                        "xor" => ast.push(ASTNode::Xor(left, right)),
                        "jne" => ast.push(ASTNode::Jne(left, right)),
                        "jeq" => ast.push(ASTNode::Jeq(left, right)),
                        "jlt" => ast.push(ASTNode::Jlt(left, right)),
                        "jgt" => ast.push(ASTNode::Jgt(left, right)),
                        "jle" => ast.push(ASTNode::Jle(left, right)),
                        "jge" => ast.push(ASTNode::Jge(left, right)),
                        _ => {
                            return Err(AssemblerError::Parser(format!(
                                "Unknown binary instruction: {}",
                                op
                            )))
                        }
                    }
                }
                Rule::unaryins => {
                    let mut inner = node.into_inner();
                    let op = inner.next().unwrap().as_str().to_lowercase();
                    let val = Self::parse_value(inner.next().unwrap())?;
                    match op.as_str() {
                        "not" => ast.push(ASTNode::Not(val)),
                        "jmp" => ast.push(ASTNode::Jmp(val)),
                        "psh" => ast.push(ASTNode::Psh(val)),
                        "pop" => ast.push(ASTNode::Pop(val)),
                        "cal" => ast.push(ASTNode::Cal(val)),
                        "inc" => ast.push(ASTNode::Inc(val)),
                        "dec" => ast.push(ASTNode::Dec(val)),
                        "sys" => ast.push(ASTNode::Sys(val)),
                        _ => {
                            return Err(AssemblerError::Parser(format!(
                                "Unknown unary instruction: {}",
                                op
                            )))
                        }
                    }
                }
                Rule::nullaryins => {
                    let mut inner = node.into_inner();
                    let op = inner.next().unwrap().as_str().to_lowercase();
                    match op.as_str() {
                        "ret" => ast.push(ASTNode::Ret),
                        "hlt" => ast.push(ASTNode::Hlt),
                        "nop" => ast.push(ASTNode::Nop),
                        _ => {
                            return Err(AssemblerError::Parser(
                                "Unknown nullary instruction".to_string(),
                            ))
                        }
                    }
                }
                Rule::label => {
                    let label = node.as_span().as_str();
                    let no_colon = label.trim_end_matches(':');
                    ast.push(ASTNode::Label(no_colon.to_string()))
                }
                Rule::EOI => break,
                _ => {
                    return Err(AssemblerError::Parser(format!(
                        "Unexpected rule: {:?}",
                        node.as_rule()
                    )))
                }
            }
        }
        Ok(ast)
    }

    fn parse_value(rule: Pair<Rule>) -> Result<ASTArg, AssemblerError> {
        match rule.as_rule() {
            Rule::decnumber => {
                let value = rule.as_str().parse::<u16>()?;
                Ok(ASTArg::Lit(value))
            }
            Rule::hexnumber => {
                let trimmed = rule.as_str().trim_start_matches("0x");
                let value = u16::from_str_radix(trimmed, 16)?;
                Ok(ASTArg::Lit(value))
            }
            Rule::octnumber => {
                let trimmed = rule.as_str().trim_start_matches("0o");
                let value = u16::from_str_radix(trimmed, 8)?;
                Ok(ASTArg::Lit(value))
            }
            Rule::binnumber => {
                let trimmed = rule.as_str().trim_start_matches("0b");
                let value = u16::from_str_radix(trimmed, 2)?;
                Ok(ASTArg::Lit(value))
            }
            Rule::reg => {
                let reg = Register::from_str(rule.as_str())
                    .map_err(|e| AssemblerError::Parser(e.to_string()))?;
                Ok(ASTArg::Reg(reg))
            }
            Rule::word => {
                let value = rule.as_str();
                Ok(ASTArg::Label(value.to_string()))
            }
            Rule::memloc => {
                let mut inner = rule.into_inner();
                let memloc_rule = inner.next().unwrap();
                Ok(ASTArg::Mem(Box::new(Self::parse_value(memloc_rule)?)))
            }
            Rule::memoff => {
                let mut inner = rule.into_inner();
                let num = inner.next().unwrap();
                let memoff_rule = inner.next().unwrap();
                Ok(ASTArg::Offset(
                    Box::new(Self::parse_value(num)?),
                    Box::new(Self::parse_value(memoff_rule)?),
                ))
            }
            _ => Err(AssemblerError::Parser(format!(
                "Unknown rule: {:?}",
                rule.as_rule()
            ))),
        }
    }
}
