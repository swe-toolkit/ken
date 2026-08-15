use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
    time::Instant,
};

use ken_kernel::{GlobalId, Term};
use num_bigint::BigInt;

use crate::prover::Z3ProcessConfig;

pub(crate) fn candidate_assignment(
    goal: &Term,
    int_id: Option<GlobalId>,
    config: &Z3ProcessConfig,
) -> Option<Vec<BigInt>> {
    let (query, binders) = emit_query(goal, int_id?)?;
    let output = run_process(config, &query)?;
    parse_assignment(&output, binders)
}

fn emit_query(goal: &Term, int_id: GlobalId) -> Option<(String, usize)> {
    let mut binders = 0;
    let mut body = goal;
    while let Term::Pi(domain, codomain) = body {
        if !matches!(domain.as_ref(), Term::Const { id, .. } if *id == int_id) {
            return None;
        }
        binders += 1;
        body = codomain;
    }
    if binders == 0 {
        return None;
    }
    let Term::Eq(ty, lhs, rhs) = body else {
        return None;
    };
    if !matches!(ty.as_ref(), Term::Const { id, .. } if *id == int_id) {
        return None;
    }
    let lhs = emit_int_expr(lhs, binders)?;
    let rhs = emit_int_expr(rhs, binders)?;

    let mut query = String::from("(set-option :produce-models true)\n(set-logic QF_LIA)\n");
    for index in 0..binders {
        query.push_str(&format!("(declare-const k{index} Int)\n"));
    }
    query.push_str(&format!("(assert (not (= {lhs} {rhs})))\n(check-sat)\n"));
    query.push_str("(get-value (");
    for index in 0..binders {
        if index > 0 {
            query.push(' ');
        }
        query.push_str(&format!("k{index}"));
    }
    query.push_str("))\n");
    Some((query, binders))
}

fn emit_int_expr(term: &Term, binders: usize) -> Option<String> {
    match term {
        Term::Var(index) if *index < binders => Some(format!("k{}", binders - 1 - index)),
        Term::IntLit(value) => Some(value.to_string()),
        _ => None,
    }
}

fn run_process(config: &Z3ProcessConfig, query: &str) -> Option<String> {
    let mut child = Command::new(&config.program)
        .args(["-in", "-smt2"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    let mut stdin = child.stdin.take()?;
    if stdin.write_all(query.as_bytes()).is_err() {
        let _ = child.kill();
        let _ = child.wait();
        return None;
    }
    drop(stdin);

    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => {
                let output = child.wait_with_output().ok()?;
                return String::from_utf8(output.stdout).ok();
            }
            Ok(Some(_)) | Err(_) => return None,
            Ok(None) if started.elapsed() < config.timeout => {
                thread::sleep(std::time::Duration::from_millis(5));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    Open,
    Close,
    Atom(&'a str),
}

fn parse_assignment(output: &str, binders: usize) -> Option<Vec<BigInt>> {
    let tokens = tokenize(output);
    let mut cursor = 0;
    atom(&tokens, &mut cursor, "sat")?;
    open(&tokens, &mut cursor)?;
    let mut assignment = Vec::with_capacity(binders);
    for index in 0..binders {
        open(&tokens, &mut cursor)?;
        atom(&tokens, &mut cursor, &format!("k{index}"))?;
        assignment.push(integer(&tokens, &mut cursor)?);
        close(&tokens, &mut cursor)?;
    }
    close(&tokens, &mut cursor)?;
    (cursor == tokens.len()).then_some(assignment)
}

fn tokenize(input: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut start = None;
    for (index, ch) in input.char_indices() {
        if ch.is_whitespace() || matches!(ch, '(' | ')') {
            if let Some(atom_start) = start.take() {
                tokens.push(Token::Atom(&input[atom_start..index]));
            }
            match ch {
                '(' => tokens.push(Token::Open),
                ')' => tokens.push(Token::Close),
                _ => {}
            }
        } else if start.is_none() {
            start = Some(index);
        }
    }
    if let Some(atom_start) = start {
        tokens.push(Token::Atom(&input[atom_start..]));
    }
    tokens
}

fn open(tokens: &[Token<'_>], cursor: &mut usize) -> Option<()> {
    matches!(tokens.get(*cursor), Some(Token::Open)).then(|| *cursor += 1)
}

fn close(tokens: &[Token<'_>], cursor: &mut usize) -> Option<()> {
    matches!(tokens.get(*cursor), Some(Token::Close)).then(|| *cursor += 1)
}

fn atom(tokens: &[Token<'_>], cursor: &mut usize, expected: &str) -> Option<()> {
    matches!(tokens.get(*cursor), Some(Token::Atom(actual)) if *actual == expected)
        .then(|| *cursor += 1)
}

fn integer(tokens: &[Token<'_>], cursor: &mut usize) -> Option<BigInt> {
    if let Some(Token::Atom(value)) = tokens.get(*cursor) {
        let parsed = value.parse().ok()?;
        *cursor += 1;
        return Some(parsed);
    }
    open(tokens, cursor)?;
    atom(tokens, cursor, "-")?;
    let Token::Atom(magnitude) = tokens.get(*cursor)? else {
        return None;
    };
    let parsed = -magnitude.parse::<BigInt>().ok()?;
    *cursor += 1;
    close(tokens, cursor)?;
    Some(parsed)
}
