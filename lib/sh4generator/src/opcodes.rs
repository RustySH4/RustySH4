use genco::prelude::*;
use itertools::Itertools;
use scraper::{Html, Selector};
use std::collections::HashMap;

use crate::helper::extract_function_name;

use super::helper::get_nth_cell_of_table;

pub fn generate_mask(inp: &str, char: char) -> u32 {
    let string_mask = inp
        .chars()
        .map(|x| if x == char { '1' } else { '0' })
        .collect::<String>();

    isize::from_str_radix(&string_mask, 2).unwrap() as u32
}

pub fn generate_args(inp: &str) -> HashMap<String, u32> {
    let mut args: HashMap<String, u32> = HashMap::new();

    let chars = inp.chars().unique();

    for char in chars {
        if char == '0' || char == '1' {
            continue;
        }

        let mask = generate_mask(inp, char);
        args.insert(char.to_string(), mask);
    }

    args
}

pub fn generate_opcodes(html_content: &str) -> rust::Tokens {
    // Parse table from HTML
    let document = Html::parse_document(html_content);

    let row_selector = Selector::parse(r#"div.col_cont"#).unwrap();
    let mut op_code_tokens: Vec<rust::Tokens> = vec![];

    let mut function_names: Vec<String> = vec![];

    for row in document.select(&row_selector) {
        let arch = get_nth_cell_of_table(&row, 1);

        if !arch.contains("SH4") {
            continue;
        }

        let name = get_nth_cell_of_table(&row, 2);
        let args = get_nth_cell_of_table(&row, 4);

        let mask = generate_mask(&args, '0') | generate_mask(&args, '1');
        let code = generate_mask(&args, '1');

        let args_map = generate_args(&args);

        let col_cont_selector = Selector::parse(r#"div.col_cont_note"#).unwrap();
        let col_cont = row.select(&col_cont_selector).next().unwrap();

        let precode_selector = Selector::parse(r#"pre"#).unwrap();
        let precode = col_cont
            .select(&precode_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();

        let function_name = extract_function_name(&precode, &function_names);
        function_names.push(function_name.clone());

        op_code_tokens.push(quote! {
            OpCode {
                display: $("\"")$(name.clone())$("\""),
                mask: $mask,
                code: $code,
                args: OpCodeArgs {
                    n: $(*args_map.get("n").unwrap_or(&0)),
                    m: $(*args_map.get("m").unwrap_or(&0)),
                    d: $(*args_map.get("d").unwrap_or(&0)),
                    i: $(*args_map.get("i").unwrap_or(&0))
                },
                callee: CPU::$function_name
            }
        });
    }

    // TODO: Generate abstract table
    quote! {
        $(format!("// THIS CODE WAS GENERATED BY SH4GENERATOR v{} BY FRANCISZEK ŁOPUSZAŃSKI", env!("CARGO_PKG_VERSION")))
        #![allow(unused)]
        use std::fmt;
        use crate::cpu::CPU;

        #[derive(Debug, Clone)]
        pub struct OpCode<'a> {
            pub display: &'a str,
            pub mask: u16,
            pub code: u16,
            pub args: OpCodeArgs,
            pub callee: fn(&mut CPU, args: OpCodeArgs) -> ()
        }

        impl<'a> fmt::Display for OpCode<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut display = self
                    .display
                    .replace(",", ", ")
                    .replace("Rn", &format!("R{}", self.args.n))
                    .replace("Rm", &format!("R{}", self.args.m))
                    .replace("disp", &format!("R{}", self.args.d))
                    .replace("#imm", &format!("0x{:04X}", self.args.i))
                    .trim()
                    .to_string();

                write!(f, "{}", display)
            }
        }

        #[derive(Debug, Default, Clone, Copy)]
        pub struct OpCodeArgs {
            pub n: u16,
            pub m: u16,
            pub i: u16,
            pub d: u16,
        }

        impl<'a> OpCode<'a> {
            pub fn decode_instruction(inp: u16) -> Option<OpCode<'a>> {
                for opcode in OPCODES_TABLE.iter() {
                    if (inp & opcode.mask) != opcode.code {
                        continue;
                    }

                    let mut res = opcode.clone();

                    fn set_arg(inp: u16, mask: u16) -> u16 {
                        if mask == 0 {
                            return 0;
                        }

                        (inp & mask) >> mask.trailing_zeros()
                    }

                    res.args.n = set_arg(inp, opcode.args.n);
                    res.args.m = set_arg(inp, opcode.args.m);
                    res.args.d = set_arg(inp, opcode.args.d);
                    res.args.i = set_arg(inp, opcode.args.i);

                    return Some(res);
                }

                None
            }
        }

        pub static OPCODES_TABLE: [OpCode; $(op_code_tokens.len())] = [
            $(for op_code_token in op_code_tokens => $op_code_token $(",")$['\n'])
        ];
    }
}
