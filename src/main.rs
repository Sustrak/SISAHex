extern crate regex;

mod instruction;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;
use std::collections::HashMap;


use instruction::{InsDic, Instruction, init_ins_dic};

fn create_hex(pos: &Instruction, s: &mut String, h: &HashMap<&str, i16>) {
    //opcode
    s.insert_str(pos.op, &format!("{:01$b}", h.get("op").unwrap(), 4));
    if pos.rb != 64 {
        s.insert_str(pos.rd, &format!("{:01$b}", h.get("rd").unwrap(), 3));
        s.insert_str(pos.ra, &format!("{:01$b}", h.get("ra").unwrap(), 3));
        s.insert_str(pos.f, &format!("{:01$b}", h.get("f").unwrap(), 3));
        s.insert_str(pos.rb, &format!("{:01$b}", h.get("rb").unwrap(), 3));
    }
    else if pos.ra == 64 {
        s.insert_str(pos.rd, &format!("{:01$b}", h.get("rd").unwrap(), 3));
        s.insert_str(pos.f, &format!("{:01$b}", h.get("f").unwrap(), 1));
        let aux = format!("{:01$b}", h.get("n").unwrap(), 8);
        s.insert_str(pos.n, &aux[aux.len()-8..aux.len()]);
    }
    else {
        s.insert_str(pos.rd, &format!("{:01$b}", h.get("rd").unwrap(), 3));
        s.insert_str(pos.ra, &format!("{:01$b}", h.get("ra").unwrap(), 3));
        let aux = format!("{:01$b}", h.get("n").unwrap(), 6);
        s.insert_str(pos.n, &aux[aux.len()-6..aux.len()]);
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let mut f = File::open(file_name).expect("File not found");
    let mut out_f = File::create(file_name.replace(".s", ".hex")).unwrap();

    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Couldn't read form the file");

    let dict: InsDic = init_ins_dic();

    let r_ins_name = Regex::new(r"[a-zA-Z]{2,}").expect("Couldn't create the r_ins_name re expresion");

    for l in contents.lines() {
        //Allow comments
        if l.chars().next().unwrap() == ';' {
            continue;
        }
        let mut ins0 = r_ins_name.captures(&l).unwrap();
        let ins_name = &ins0[0];
        
        let mut r_ins_parse: Regex;
        
        match dict.parse_ins.get(ins_name) {
            Some(re) => r_ins_parse = Regex::new(&re).expect("Couldn't create the r_ins_parse re expresion"),
            None => panic!("{} is not a instruction.", ins_name)
        }

        let mut ins_b: HashMap<&str, i16> = HashMap::new();
        
        ins_b.insert("op", dict.op_codes.get(ins_name).unwrap().opcode);
        ins_b.insert("f", dict.op_codes.get(ins_name).unwrap().f);
        let c = r_ins_parse.captures(&l).unwrap();
        
        for item in dict.uses.get(ins_name).unwrap() {
            if item == "n" && (ins_name == "st" || ins_name == "ld") {
                if l.contains("0x") {
                    println!("{}, {}", l, i16::from_str_radix(&c[item.as_str()].to_string(), 16).unwrap());
                    ins_b.insert(item, i16::from_str_radix(&c[item.as_str()].to_string(), 16).unwrap()/2);
                }
                else {
                    ins_b.insert(item, c[item.as_str()].to_string().parse::<i16>().unwrap()/2);  
                }
            }
            else if item == "n" && l.contains("0x") {
                println!("{}, {}", l, i16::from_str_radix(&c[item.as_str()].to_string(), 16).unwrap());
                ins_b.insert(item, i16::from_str_radix(&c[item.as_str()].to_string(), 16).unwrap());
            }
            else {
                ins_b.insert(item, c[item.as_str()].to_string().parse::<i16>().unwrap());
            }

        }
        
        if ins_name == "not" {
            ins_b.insert("rb", 0);
        }
        let mut out_s: String = String::with_capacity(16);

        let ins_positions = dict.m_code_loc.get(ins_name).unwrap();

        // println!("{:?}", ins_b.get("n"));

        create_hex(&ins_positions, &mut out_s, &ins_b);
        
        writeln!(out_f, "{:01$x}", i32::from_str_radix(&out_s, 2).unwrap(), 4);
    }
    write!(out_f, "FFFF");
}
