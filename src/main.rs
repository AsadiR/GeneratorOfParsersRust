mod ast;

use std::vec::Vec;

/*
((grammar = rule+)
 (rule = <@NONTERM "=" multi_addendum> )
 (multi_addendum = addendum <"||" addendum>* )
 (addendum = factor+)
 (factor = "(" multi_addendum ")" op || ident op || )
 (op = "*" || "+" || "?" || )
 (ident  = @TERM || @NONTERM )))

*/




macro_rules! gen_ast {
    // grammar = rule+ .
    // rule = "[" @NONTERM "=" (multi_addendum | factor) "]" .
    ( $([$lp:ident => $rp:tt])* ) => {{
        let mut grammar = ast::AST {
            marker: "Grammar".to_string(),
            value: None,
            children: Some(Vec::new())
        };

        $(
            let mut rule = ast::AST {
                marker: "Rule".to_string(),
                value: None,
                children: Some(Vec::new())
            };
            let lp = ast::AST {
                marker: "@Term".to_string(),
                value: Some($lp),
                children: None
            };
            let mut ma = ast::AST {
                marker: "MultiAddendum".to_string(),
                value: None,
                children: Some(Vec::new())
            };
            rule.children.as_mut().unwrap().push(lp);
            gen_ast!($rp @ma);
            rule.children.as_mut().unwrap().push(ma);
            grammar.children.as_mut().unwrap().push(rule);

        )*;
        grammar
    }};

    // multi_addendum = "(" addendum ("|" addendum)* ")" .
    ( ($($a:tt)|+) @$ma:ident ) => {
        $(
            let a = ast::AST {
                marker: "Addendum".to_string(),
                value: None,
                children: Some(Vec::new())
            };
            gen_ast!($a);
            $ma.children.as_mut().unwrap().push(a);
        )*;
    };

    //factor = "(" multi_addendum ")"
    ( ($($f:tt)*)) => {
        println!("factor_in_br");
    };

    //factor = ident
    ( $($f:ident)* ) => {
        println!("factor ");
    };
}



fn main() {
    let term = "term".to_string();
    let number = "number".to_string();
    //let it ="it".to_string();
    let (at,bt,ct) = (1,2,3);
    let grammar = gen_ast!{
        [term => ( (at bt) | (bt|ct) | ct)]
        //[it => ( (3 5)+ | 4+)]
        //[number => 1]
    };
    println!("{}", grammar);
}

//(Grammar -> (Rule -> (@Term -> value: term) ) (Rule -> (@Term -> value: number) ) )
//cargo rustc -- -Z unstable-options --pretty=expanded