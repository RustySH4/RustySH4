use genco::prelude::*;
use scraper::{Html, Selector};

use crate::helper::extract_function_name;

use super::helper::get_nth_cell_of_table;

pub fn generate_impl_empty(html_content: &str) -> rust::Tokens {
    // Parse table from HTML
    let document = Html::parse_document(html_content);

    let row_selector = Selector::parse(r#"div.col_cont"#).unwrap();
    let mut function_names: Vec<String> = vec![];
    let mut function_abstracts: Vec<String> = vec![];

    for row in document.select(&row_selector) {
        let arch = get_nth_cell_of_table(&row, 1);

        if !arch.contains("SH4") {
            continue;
        }

        let col_cont_selector = Selector::parse(r#"div.col_cont_note"#).unwrap();
        let col_cont = row.select(&col_cont_selector).next().unwrap();

        let precode_selector = Selector::parse(r#"pre"#).unwrap();
        let precode = col_cont
            .select(&precode_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();

        let function_abstract = get_nth_cell_of_table(&row, 3);
        function_abstracts.push(function_abstract);
        let function_name = extract_function_name(&precode, &function_names);
        function_names.push(function_name);
    }

    quote! {
        $(format!("// THIS CODE WAS GENERATED BY SH4GENERATOR v{} BY FRANCISZEK ŁOPUSZAŃSKI", env!("CARGO_PKG_VERSION")))
        #![allow(unused)]
        use crate::{
            cpu::CPU,
            opcodes::{OpCode, OpCodeArgs},
        };

        #[allow(non_snake_case)]
        impl CPU {
            fn sign_extend(x: u16) -> u32 {
                if (x & 0x80) == 0 {
                    x as u32 & 0x000_000FF
                } else {
                    x as u32 | 0xFFFF_FF00
                }
            }

            pub fn execute(&mut self, opcode: OpCode) {
                (opcode.callee)(self, opcode.args)
            }

            $(for (name, abst) in function_names.iter().zip(function_abstracts.iter()) => $(quote!(
                pub fn $name(&mut self, args: OpCodeArgs){
                    $(format!("/* {} */", abst))
                    todo!()
                }$['\n']
            )))
        }
    }
}
