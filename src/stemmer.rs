use regex::Regex;

// This is a direct port of the stemmer from elasticlunr.js
// It's not very efficient and very not-rusty, but it
// generates identical output.

mod phf_maps {
    include!(concat!(env!("OUT_DIR"), "/stemmer_maps.rs"));
}

struct Stemmer {
    re_mgr0: Regex,
    re_mgr1: Regex,
    re_meq1: Regex,
    re_s_v: Regex,

    re_1a: Regex,
    re2_1a: Regex,
    re_1b: Regex,
    re2_1b: Regex,
    re_1b_2: Regex,
    re2_1b_2: Regex,
    re3_1b_2: Regex,
    re4_1b_2: Regex,

    re_1c: Regex,
    re_2: Regex,

    re_3: Regex,

    re_4: Regex,
    re2_4: Regex,

    re_5: Regex,
    re_5_1: Regex,
    re3_5: Regex,
}

#[allow(non_upper_case_globals)]
const v: &str = "[aeiouy]"; // vowel
#[allow(non_upper_case_globals)]
const C: &str = "[^aeiou][^aeiouy]*"; // consonant sequence
#[allow(non_upper_case_globals)]
const V: &str = "[aeiouy][aeiou]*"; // vowel sequence

macro_rules! concat_buf {
    ($($st:expr),*) => {
        {
            let mut temp_str = String::new();
            $(temp_str.push_str($st);)*
            temp_str
        }
    }
}

impl Stemmer {
    fn new() -> Self {
        let mgr0 = concat_buf!("^(", C, ")?", V, C);
        let meq1 = concat_buf!("^(", C, ")?", V, C, "(", V, ")?$");
        let mgr1 = concat_buf!("^(", C, ")?", V, C, V, C);
        let s_v = concat_buf!("^(", C, ")?", v);

        let re_mgr0 = Regex::new(&mgr0).unwrap();
        let re_mgr1 = Regex::new(&mgr1).unwrap();
        let re_meq1 = Regex::new(&meq1).unwrap();
        let re_s_v = Regex::new(&s_v).unwrap();

        let re_1a = Regex::new("^(.+?)(ss|i)es$").unwrap();
        let re2_1a = Regex::new("^(.+?)([^s])s$").unwrap();
        let re_1b = Regex::new("^(.+?)eed$").unwrap();
        let re2_1b = Regex::new("^(.+?)(ed|ing)$").unwrap();
        let re_1b_2 = Regex::new(".$").unwrap();
        let re2_1b_2 = Regex::new("(at|bl|iz)$").unwrap();
        let re3_1b_2 = Regex::new("([^aeiouylsz]{2})$").unwrap();
        let re4_1b_2 = Regex::new(&concat_buf!("^", C, v, "[^aeiouwxy]$")).unwrap();

        let re_1c = Regex::new("^(.+?[^aeiou])y$").unwrap();
        let re_2 = Regex::new("^(.+?)(ational|tional|enci|anci|izer|bli|alli|entli|eli|ousli|\
            ization|ation|ator|alism|iveness|fulness|ousness|aliti|iviti|biliti|logi)$").unwrap();

        let re_3 = Regex::new("^(.+?)(icate|ative|alize|iciti|ical|ful|ness)$").unwrap();

        let re_4 = Regex::new("^(.+?)(al|ance|ence|er|ic|able|ible|ant|ement|ment|ent|ou|ism|ate|iti|ous|ive|ize)$").unwrap();
        let re2_4 = Regex::new("^(.+?)(s|t)(ion)$").unwrap();

        let re_5 = Regex::new("^(.+?)e$").unwrap();
        let re_5_1 = Regex::new("ll$").unwrap();
        let re3_5 = Regex::new(&concat_buf!("^", C, v, "[^aeiouwxy]$")).unwrap();

        Stemmer {
            re_mgr0,
            re_mgr1,
            re_meq1,
            re_s_v,
            re_1a,
            re2_1a,
            re_1b,
            re2_1b,
            re_1b_2,
            re2_1b_2,
            re3_1b_2,
            re4_1b_2,
            re_1c,
            re_2,
            re_3,
            re_4,
            re2_4,
            re_5,
            re_5_1,
            re3_5,
        }
    }

    /// Implements the Porter stemming algorithm
    pub fn stem(&self, mut w: String) -> String {
        let step2list = &phf_maps::STEMMER_STEP_2;
        let step3list = &phf_maps::STEMMER_STEP_3;

        if w.len() < 3 {
            return w;
        }

        let firstch = &w[..1].to_string();
        if firstch == "y" {
            w.remove(0);
            w.insert(0, 'Y');
        }

        // TODO: There's probably a better way to handle the
        // borrowchecker than cloning w a million times

        // Step 1a
        if let Some(caps) = self.re_1a.captures(&w.clone()) {
            w = concat_buf!(caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
        }
        if let Some(caps) = self.re2_1a.captures(&w.clone()) {
            w = concat_buf!(caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
        }

        // Step 1b
        if let Some(caps) = self.re_1b.captures(&w.clone()) {
            let stem = caps.get(1).unwrap().as_str();
            if self.re_mgr0.is_match(stem) {
                w = self.re_1b_2.replace(&w, "").into();
            }
        } else if let Some(caps) = self.re2_1b.captures(&w.clone()) {
            let stem = caps.get(1).unwrap().as_str();
            if self.re_s_v.is_match(&stem) {
                w = stem.into();
                if self.re2_1b_2.is_match(&w) {
                    w.push('e');
                } else if let Some(caps) = self.re3_1b_2.captures(&w.clone()) {
                    let suffix = caps.get(0).unwrap().as_str();
                    // Make sure the two characters are the same since we can't use backreferences
                    if suffix[0..1] == suffix[1..2] {
                        w = self.re_1b_2.replace(&w, "").into();
                    }
                } else if self.re4_1b_2.is_match(&w) {
                    w.push('e');
                }
            }
        }

        // Step 1c - replace suffix y or Y by i if preceded by a non-vowel which is not the first
        // letter of the word (so cry -> cri, by -> by, say -> say)
        if let Some(caps) = self.re_1c.captures(&w.clone()) {
            let stem = caps.get(1).unwrap().as_str();
            w = concat_buf!(stem, "i");
        }

        // Step 2
        if let Some(caps) = self.re_2.captures(&w.clone()) {
            let stem = caps.get(1).unwrap().as_str();
            let suffix = caps.get(2).unwrap().as_str();
            if self.re_mgr0.is_match(&stem) {
                w = concat_buf!(stem, step2list.get(suffix).unwrap());
            }
        }

        // Step 3
        if let Some(caps) = self.re_3.captures(&w.clone()) {
            let stem = caps.get(1).unwrap().as_str();
            let suffix = caps.get(2).unwrap().as_str();
            if self.re_mgr0.is_match(&stem) {
                w = concat_buf!(stem, step3list.get(suffix).unwrap());
            }
        }

        // Step 4
        if let Some(caps) = self.re_4.captures(&w.clone()) {
            let stem = caps.get(1).unwrap().as_str();
            if self.re_mgr1.is_match(&stem) {
                w = stem.into();
            }
        } else if let Some(caps) = self.re2_4.captures(&w.clone()) {
            let stem = concat_buf!(caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
            if self.re_mgr1.is_match(&stem) {
                w = stem.into();
            }
        }

        // Step 5
        if let Some(caps) = self.re_5.captures(&w.clone()) {
            let stem = caps.get(1).unwrap().as_str();
            if self.re_mgr1.is_match(&stem) ||
                (self.re_meq1.is_match(&stem) && !(self.re3_5.is_match(&stem)))
            {
                w = stem.into();
            }
        }

        if self.re_5_1.is_match(&w) && self.re_mgr1.is_match(&w) {
            w = self.re_1b_2.replace(&w, "").into();
        }

        // and turn initial Y back to y
        if firstch == "y" {
            w.remove(0);
            w.insert(0, 'y');
        }

        w
    }
}

pub fn stemmer(input: String) -> Option<String> {
    lazy_static! {
        static ref STEMMER: Stemmer = Stemmer::new();
    }
    Some(STEMMER.stem(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stemmer() {
        let cases = [
            ("consign", "consign"),
            ("consigned", "consign"),
            ("consigning", "consign"),
            ("consignment", "consign"),
            ("consist", "consist"),
            ("consisted", "consist"),
            ("consistency", "consist"),
            ("consistent", "consist"),
            ("consistently", "consist"),
            ("consisting", "consist"),
            ("consists", "consist"),
            ("consolation", "consol"),
            ("consolations", "consol"),
            ("consolatory", "consolatori"),
            ("console", "consol"),
            ("consoled", "consol"),
            ("consoles", "consol"),
            ("consolidate", "consolid"),
            ("consolidated", "consolid"),
            ("consolidating", "consolid"),
            ("consoling", "consol"),
            ("consols", "consol"),
            ("consonant", "conson"),
            ("consort", "consort"),
            ("consorted", "consort"),
            ("consorting", "consort"),
            ("conspicuous", "conspicu"),
            ("conspicuously", "conspicu"),
            ("conspiracy", "conspiraci"),
            ("conspirator", "conspir"),
            ("conspirators", "conspir"),
            ("conspire", "conspir"),
            ("conspired", "conspir"),
            ("conspiring", "conspir"),
            ("constable", "constabl"),
            ("constables", "constabl"),
            ("constance", "constanc"),
            ("constancy", "constanc"),
            ("constant", "constant"),
            ("knack", "knack"),
            ("knackeries", "knackeri"),
            ("knacks", "knack"),
            ("knag", "knag"),
            ("knave", "knave"),
            ("knaves", "knave"),
            ("knavish", "knavish"),
            ("kneaded", "knead"),
            ("kneading", "knead"),
            ("knee", "knee"),
            ("kneel", "kneel"),
            ("kneeled", "kneel"),
            ("kneeling", "kneel"),
            ("kneels", "kneel"),
            ("knees", "knee"),
            ("knell", "knell"),
            ("knelt", "knelt"),
            ("knew", "knew"),
            ("knick", "knick"),
            ("knif", "knif"),
            ("knife", "knife"),
            ("knight", "knight"),
            ("knights", "knight"),
            ("knit", "knit"),
            ("knits", "knit"),
            ("knitted", "knit"),
            ("knitting", "knit"),
            ("knives", "knive"),
            ("knob", "knob"),
            ("knobs", "knob"),
            ("knock", "knock"),
            ("knocked", "knock"),
            ("knocker", "knocker"),
            ("knockers", "knocker"),
            ("knocking", "knock"),
            ("knocks", "knock"),
            ("knopp", "knopp"),
            ("knot", "knot"),
            ("knots", "knot"),
            ("lay", "lay"),
            ("try", "tri"),
        ];

        for &(input, output) in cases.iter() {
            assert_eq!(&stemmer(input.into()).unwrap(), output);
        }
    }
}
