mod ast;

use std::vec::Vec;

macro_rules! gen_ast {
    // grammar = rule+ .
    // rule = "[" "@nonterm" "=" multi_addendum "]" .
    ( $([@$lp:ident => $rp:tt])* ) => {{
        let mut grammar = ast::AST {
            marker: "Grammar".to_string(),
            value: None,
            children: Some(Vec::new())
        };

        $(
            let mut rule = ast::AST {
                marker: "#Rule".to_string(),
                value: None,
                children: Some(Vec::new())
            };
            let lp = ast::AST {
                marker: "Term".to_string(),
                value: Some($lp),
                children: None
            };
            let mut ma = ast::AST {
                marker: "MultiAddendum".to_string(),
                value: None,
                children: Some(Vec::new())
            };
            rule.children.as_mut().unwrap().push(lp);
            gen_ast!($rp %ma ma);
            rule.children.as_mut().unwrap().push(ma);
            grammar.children.as_mut().unwrap().push(rule);

        )*;
        grammar
    }};

    // multi_addendum = addendum ("|" addendum)* .
    ( $($a:tt)|+ %ma $ma:ident ) => {
        $(
            let mut a = ast::AST {
                marker: "Addendum".to_string(),
                value: None,
                children: Some(Vec::new())
            };
            gen_ast!($a %a a);
            $ma.children.as_mut().unwrap().push(a);
        )*;
    };

    // addendum = "(" factor+ ")" .
    ( ($($f:tt)+) %a $a:ident) => {
        $(
            let mut f = ast::AST {
                marker: "Factor".to_string(),
                value: None,
                children: Some(Vec::new())
            };
            gen_ast!($f %f f);
            $a.children.as_mut().unwrap().push(f);
        )*;
    };


    // factor = ident .
    // ident  = "term" .
    ( $t:ident %f $f:ident) => {
        let t = ast::AST {
            marker: "Term".to_string(),
            value: Some($t),
            children: None
        };
        $f.children.as_mut().unwrap().push(t);
    }

/*
    // factor = ident "*" .
    // ident  = "term" .
    ( * %f $f:ident ) => {
        let op = ast::AST {
            marker: "Term".to_string(),
            value: Some("*".to_string()),
            children: None
        };
        $f.children.as_mut().unwrap().push(op);
    }

    // factor = ident "+" .
    // ident  = "term" .
    ( + %f $f:ident) => {
        let op = ast::AST {
            marker: "Term".to_string(),
            value: Some("+".to_string()),
            children: None
        };
        $f.children.as_mut().unwrap().push(op);
    }

    // factor = ident "?" .
    // ident  = "term" .
    ( ? %f $f:ident) => {
        let op = ast::AST {
            marker: "Term".to_string(),
            value: Some("?".to_string()),
            children: None
        };
        $f.children.as_mut().unwrap().push(op);
    }
*/
}

macro_rules! test{
    ( + ) => { println!("hi");};
}

/*
grammar = rule+ .
rule = "[" "@nonterm" "=" multi_addendum "]" .
multi_addendum = addendum ("|" addendum)* .
addendum = "(" factor+ ")" .
factor = ident .
factor = ident "*" .
factor = ident "+" .
factor = ident "?" .
factor = multi_addendum .
factor = multi_addendum "*" .
factor = multi_addendum "+" .
factor = multi_addendum "?" .
ident  = "term" .
ident = "@nonterm" .
*/

fn main() {
    let (term, number) = ("term".to_string(), "number".to_string());
    let (at,bt,ct) = ("1".to_string(),"2".to_string(),"3".to_string());
    let grammar = gen_ast! {
        [@term => (at)]
        [@number => (ct)]
    };
    println!("{}", grammar);
}

//cargo rustc -- -Z unstable-options --pretty=expanded
//cargo rustc -- -Z trace-macros