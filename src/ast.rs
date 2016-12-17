use std::fmt;

pub struct AST {
    pub marker: String,
    pub value : Option<String>,
    pub children : Option<Vec<AST>>
}


// реализация trait-а Display, для визуализации AST
impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str =  "".to_string();
        if self.children.is_some() {
            for node in self.children.as_ref().unwrap().iter() {
                str.push_str(&node.to_string());
                str.push_str(" ");
            }
        } else {
            str = format!("value: {}", self.value.as_ref().unwrap()).to_string();
        };
        write!(f, "({} -> {})", self.marker, str)
    }
}

/*
 в Rust нельзя реализовать простейшую рекурсивную структуру из-за его семантики
 поэтому от хранения parent-ов пришлось отказаться
*/