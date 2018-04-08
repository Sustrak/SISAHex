use std::collections::HashMap;

#[derive(Debug,PartialEq, Eq, Hash, Clone, Copy)]
pub struct Instruction {
    pub op: usize,
    pub f : usize,
    pub rd: usize,
    pub ra: usize,
    pub rb: usize,
    pub n : usize
}

#[derive(Debug,PartialEq, Eq, Hash, Clone, Copy)]
pub struct Codes {
    pub opcode: i16,
    pub f: i16
}

pub struct InsDic {
    pub m_code_loc: HashMap<String, Instruction>,
    pub parse_ins: HashMap<String, String>,
    pub op_codes: HashMap<String, Codes>,
    pub uses: HashMap<String, Vec<String>>

}


pub fn init_ins_dic() -> InsDic {
    let mut ins_dic = InsDic {
        m_code_loc: HashMap::new(),
        parse_ins: HashMap::new(),
        op_codes: HashMap::new(),
        uses: HashMap::new()
    };
    //Instruction format of Logic Arithmetic & Comp
    let mut ins = Instruction{
        op: 0,
        f: 10,
        rd: 4,
        ra: 7,
        rb: 13,
        n: 64
    };

    let s: String = String::from(r"r(?P<rd>\d),\s?r(?P<ra>\d),\s?r(?P<rb>\d)");
    let mut c = Codes {
        opcode: 0,
        f: 0
    };
    //Logic & Arithmetic
    ins_dic.m_code_loc.insert(String::from("and"), ins);
    ins_dic.m_code_loc.insert(String::from("or"), ins);
    ins_dic.m_code_loc.insert(String::from("xor"), ins);
    ins_dic.m_code_loc.insert(String::from("not"), ins);
    ins_dic.m_code_loc.insert(String::from("add"), ins);
    ins_dic.m_code_loc.insert(String::from("sub"), ins);
    ins_dic.m_code_loc.insert(String::from("sha"), ins);
    ins_dic.m_code_loc.insert(String::from("shl"), ins);

    ins_dic.parse_ins.insert(String::from("and"), s.clone());
    ins_dic.parse_ins.insert(String::from("or"), s.clone());
    ins_dic.parse_ins.insert(String::from("xor"), s.clone());
    let ss: String = String::from(r"r(?P<rd>\d),\s?r(?P<ra>\d)");
    ins_dic.parse_ins.insert(String::from("not"), ss);
    ins_dic.parse_ins.insert(String::from("add"), s.clone());
    ins_dic.parse_ins.insert(String::from("sub"), s.clone());
    ins_dic.parse_ins.insert(String::from("sha"), s.clone());
    ins_dic.parse_ins.insert(String::from("shl"), s.clone());

    ins_dic.op_codes.insert(String::from("and"), c);
    c.f = 1;
    ins_dic.op_codes.insert(String::from("or"), c);
    c.f = 2;
    ins_dic.op_codes.insert(String::from("xor"), c);
    c.f = 3;
    ins_dic.op_codes.insert(String::from("not"), c);
    c.f = 4;
    ins_dic.op_codes.insert(String::from("add"), c);
    c.f = 5;
    ins_dic.op_codes.insert(String::from("sub"), c);
    c.f = 6;
    ins_dic.op_codes.insert(String::from("sha"), c);
    c.f = 7;
    ins_dic.op_codes.insert(String::from("shl"), c);


    ins_dic.uses.insert(String::from("and"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("or"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("xor"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("not"), vec![String::from("rd"), String::from("ra")]);
    ins_dic.uses.insert(String::from("add"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("sub"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("sha"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("shl"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);

    //Comparation
    ins_dic.m_code_loc.insert(String::from("cmplt"), ins);
    ins_dic.m_code_loc.insert(String::from("cmple"), ins);
    ins_dic.m_code_loc.insert(String::from("cmpeq"), ins);
    ins_dic.m_code_loc.insert(String::from("cmpltu"), ins);
    ins_dic.m_code_loc.insert(String::from("cmpleu"), ins);

    ins_dic.parse_ins.insert(String::from("cmplt"), s.clone());
    ins_dic.parse_ins.insert(String::from("cmple"), s.clone());
    ins_dic.parse_ins.insert(String::from("cmpeq"), s.clone());
    ins_dic.parse_ins.insert(String::from("cmpltu"), s.clone());
    ins_dic.parse_ins.insert(String::from("cmpleu"), s.clone());

    c.opcode = 1;
    c.f = 0;
    ins_dic.op_codes.insert(String::from("cmplt"), c);
    c.f = 1;
    ins_dic.op_codes.insert(String::from("cmple"), c);
    c.f = 3;
    ins_dic.op_codes.insert(String::from("cmpeq"), c);
    c.f = 4;
    ins_dic.op_codes.insert(String::from("cmpltu"), c);
    c.f = 5;
    ins_dic.op_codes.insert(String::from("cmpleu"), c);

    ins_dic.uses.insert(String::from("cmplt"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("cmple"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("cmpeq"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("cmpltu"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("cmpleu"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);

    //ARITHMETIC EXTENSION
    ins_dic.m_code_loc.insert(String::from("mul"), ins);
    ins_dic.m_code_loc.insert(String::from("mulh"), ins);
    ins_dic.m_code_loc.insert(String::from("mulhu"), ins);
    ins_dic.m_code_loc.insert(String::from("div"), ins);
    ins_dic.m_code_loc.insert(String::from("divu"), ins);

    ins_dic.parse_ins.insert(String::from("mul"), s.clone());
    ins_dic.parse_ins.insert(String::from("mulh"), s.clone());
    ins_dic.parse_ins.insert(String::from("mulhu"), s.clone());
    ins_dic.parse_ins.insert(String::from("div"), s.clone());
    ins_dic.parse_ins.insert(String::from("divu"), s.clone());

    c.opcode = 8;
    c.f = 0;
    ins_dic.op_codes.insert(String::from("mul"), c);
    c.f = 1;
    ins_dic.op_codes.insert(String::from("mulh"), c);
    c.f = 2;
    ins_dic.op_codes.insert(String::from("mulhu"), c);
    c.f = 4;
    ins_dic.op_codes.insert(String::from("div"), c);
    c.f = 5;
    ins_dic.op_codes.insert(String::from("divu"), c);

    ins_dic.uses.insert(String::from("mul"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("mulh"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("mulhu"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("div"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    ins_dic.uses.insert(String::from("divu"), vec![String::from("rd"), String::from("ra"), String::from("rb")]);
    
    //ADDI
    ins.f = 64; ins.rb = 64;
    ins.n = 10;
    c.opcode = 2; c.f = 0;
    ins_dic.m_code_loc.insert(String::from("addi"), ins);
    ins_dic.parse_ins.insert(String::from("addi"), String::from(r"r(?P<rd>\d),\s?r(?P<ra>\d),\s?0?x?(?P<n>-?[0-9a-zA-Z]{1,})"));
    ins_dic.op_codes.insert(String::from("addi"), c);
    ins_dic.uses.insert(String::from("addi"), vec![String::from("rd"), String::from("ra"), String::from("n")]);

    //LOAD
    c.opcode = 3; c.f = 0;
    ins_dic.m_code_loc.insert(String::from("ld"), ins);
    ins_dic.parse_ins.insert(String::from("ld"), String::from(r"r(?P<rd>\d),\s?(?P<n>-?[0-9a-zA-Z]{1,})\(r(?P<ra>\d)\)"));
    ins_dic.op_codes.insert(String::from("ld"), c);
    ins_dic.uses.insert(String::from("ld"), vec![String::from("rd"), String::from("ra"), String::from("n")]);

    //STORE
    c.opcode = 4; c.f = 0;
    ins_dic.m_code_loc.insert(String::from("st"), ins);
    ins_dic.parse_ins.insert(String::from("st"), String::from(r"0?x?(?P<n>-?[0-9a-zA-Z]{1,})\(r(?P<ra>\d)\),\s?r(?P<rd>\d)"));
    ins_dic.op_codes.insert(String::from("st"), c);
    ins_dic.uses.insert(String::from("st"), vec![String::from("rd"), String::from("ra"), String::from("n")]);

    //LDB
    c.opcode = 13;
    ins_dic.m_code_loc.insert(String::from("ldb"), ins);
    ins_dic.parse_ins.insert(String::from("ldb"), String::from(r"r(?P<rd>\d),\s?0?x?(?P<n>-?[0-9a-zA-Z]{1,})\(r(?P<ra>\d)\)"));
    ins_dic.op_codes.insert(String::from("ldb"), c);
    ins_dic.uses.insert(String::from("ldb"), vec![String::from("rd"), String::from("ra"), String::from("n")]);

    //STB
    c.opcode = 14;
    ins_dic.m_code_loc.insert(String::from("stb"), ins);
    ins_dic.parse_ins.insert(String::from("stb"), String::from(r"(?P<n>-?[0-9a-zA-Z]{1,})\(r(?P<ra>\d)\),\s?r(?P<rd>\d)"));
    ins_dic.op_codes.insert(String::from("stb"), c);
    ins_dic.uses.insert(String::from("stb"), vec![String::from("rd"), String::from("ra"), String::from("n")]);

    //MOV
    c.opcode = 5; c.f = 0;
    ins.f = 7;
    ins.ra = 64;
    ins.n = 8;
    ins_dic.m_code_loc.insert(String::from("movi"), ins);
    ins_dic.m_code_loc.insert(String::from("movhi"), ins);

    ins_dic.parse_ins.insert(String::from("movi"), String::from(r"r(?P<rd>\d),\s?0?x?(?P<n>-?[0-9a-zA-Z]{1,})"));
    ins_dic.parse_ins.insert(String::from("movhi"), String::from(r"r(?P<rd>\d),\s?0?x?(?P<n>-?[0-9a-zA-Z]{1,})"));

    ins_dic.op_codes.insert(String::from("movi"), c);
    c.f = 1;
    ins_dic.op_codes.insert(String::from("movhi"), c);

    ins_dic.uses.insert(String::from("movi"), vec![String::from("rd"), String::from("n")]);
    ins_dic.uses.insert(String::from("movhi"), vec![String::from("rd"), String::from("n")]);

    ins_dic
}

