pub fn rot(s: &str) -> String {
    // your code
    let mut bytes = s.as_bytes().to_vec();
    bytes.reverse();

    String::from_utf8(bytes).unwrap()
}

pub fn selfie_and_rot(s: &str) -> String {
    let lines: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line: &str| {
            let mut bytes = line.as_bytes().to_vec();
            let mut dots = vec![b'.'; bytes.len()];
            bytes.append(&mut dots);
            String::from_utf8(bytes).unwrap()
        }).collect();

    let left = lines.join("\n");
    let right = rot(left.as_str());

    vec![left, right].join("\n")
}

// first parameter: dots have to be replaced by function of one variable
pub fn oper(opfn: fn(&str) -> String, s: &str) -> String {
    opfn(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing1(s: &str, exp: &str) -> () {
        assert_eq!(oper(rot, s), exp.to_string())
    }
    fn testing2(s: &str, exp: &str) -> () {
        assert_eq!(oper(selfie_and_rot, s), exp.to_string())
    }

    #[test]
    fn basics_oper() {
        testing1(
            "fijuoo\nCqYVct\nDrPmMJ\nerfpBA\nkWjFUG\nCVUfyL",
            "LyfUVC\nGUFjWk\nABpfre\nJMmPrD\ntcVYqC\nooujif",
        );
        testing1("rkKv\ncofM\nzXkh\nflCB", "BClf\nhkXz\nMfoc\nvKkr");

        testing2(
            "xZBV\njsbS\nJcpN\nfVnP",
            "xZBV....\njsbS....\nJcpN....\nfVnP....\n....PnVf\n....NpcJ\n....Sbsj\n....VBZx",
        );
        testing2(
            "uLcq\nJkuL\nYirX\nnwMB",
            "uLcq....\nJkuL....\nYirX....\nnwMB....\n....BMwn\n....XriY\n....LukJ\n....qcLu",
        );
    }
}
