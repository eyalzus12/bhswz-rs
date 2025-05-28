use swf::{
    Swf, Tag,
    avm2::types::{AbcFile, ConstantPool, MethodBody, Multiname, Op},
};

type AbcReader<'a> = swf::avm2::read::Reader<'a>;

pub fn find_swz_key_in_swf(swf: &swf::Swf) -> Option<u32> {
    read_abc_file(swf).and_then(|abc| find_swz_key(&abc))
}

pub fn find_swz_key(abc_file: &AbcFile) -> Option<u32> {
    // search for a pushuint instruction before a call to ANE_RawData.Init
    // this means that pushuint is in between a getlex "ANE_Rawdata" instruction,
    // and a callpropvoid "Init" instruction

    for mb in &abc_file.method_bodies {
        let code = mb.decode_instructions();
        let getlex_pos = find_rawdata_getlex(&code, &abc_file.constant_pool);
        for (i, &pos) in getlex_pos.iter().enumerate() {
            let end = match i == getlex_pos.len() - 1 {
                true => code.len(),
                false => getlex_pos[i + 1],
            };

            let callpropvoid_pos = find_rawdata_init_call(&code[pos..end], &abc_file.constant_pool);

            if let Some(cpv_pos) = callpropvoid_pos {
                match find_last_pushuint_value(&code[0..cpv_pos], &abc_file.constant_pool) {
                    Some(arg) => return Some(arg),
                    None => {}
                };
            }
        }
    }

    None
}

fn get_muliname_name<'a>(multiname: &'a Multiname, cpool: &'a ConstantPool) -> Option<&'a str> {
    match multiname {
        Multiname::QName { name, .. } => Some(*name),
        Multiname::QNameA { name, .. } => Some(*name),
        Multiname::RTQName { name, .. } => Some(*name),
        Multiname::RTQNameA { name, .. } => Some(*name),
        Multiname::Multiname { name, .. } => Some(*name),
        Multiname::MultinameA { name, .. } => Some(*name),
        _ => None,
    }
    .and_then(|idx| {
        let name = &cpool.strings[idx.0 as usize - 1];
        std::str::from_utf8(name).ok()
    })
}

fn find_rawdata_getlex(code: &[Op], cpool: &ConstantPool) -> Vec<usize> {
    code.iter()
        .enumerate()
        .filter_map(|(code_idx, op)| match op {
            Op::GetLex { index } => {
                let mn = &cpool.multinames[index.0 as usize - 1];
                get_muliname_name(mn, cpool)
                    .filter(|&n| n == "ANE_RawData")
                    .and(Some(code_idx))
            }
            _ => None,
        })
        .collect()
}

fn find_rawdata_init_call(code: &[Op], cpool: &ConstantPool) -> Option<usize> {
    code.iter().position(|op| match op {
        Op::CallPropVoid { index, .. } => {
            let mn = &cpool.multinames[index.0 as usize - 1];
            get_muliname_name(mn, cpool)
                .filter(|&n| n == "Init")
                .is_some()
        }
        _ => false,
    })
}

fn find_last_pushuint_value(code: &[Op], cpool: &ConstantPool) -> Option<u32> {
    code.iter().rev().find_map(|ins| match ins {
        Op::PushUint { value } => {
            let arg = cpool.uints[value.0 as usize - 1];
            Some(arg)
        }
        _ => None,
    })
}

trait MethodBodyExt {
    fn decode_instructions(&self) -> Vec<Op>;
}

impl MethodBodyExt for MethodBody {
    fn decode_instructions(&self) -> Vec<Op> {
        let mut instructions: Vec<Op> = Vec::new();
        let mut reader = AbcReader::new(&self.code);
        while let Ok(ins) = reader.read_op() {
            instructions.push(ins);
        }

        return instructions;
    }
}

fn read_abc_file(swf: &Swf) -> Option<AbcFile> {
    swf.tags.iter().find_map(|t| match t {
        Tag::DoAbc(data) => {
            let mut reader = AbcReader::new(data);
            let abc_file = reader.read().ok();
            abc_file
        }
        _ => None,
    })
}
