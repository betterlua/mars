use std::vec::IntoIter;

use regex::{Regex, Replacer};

use crate::BundlerEnvoirment;

use super::{parse_attr_line, MarAttribute};

pub struct Skip;

pub struct AttrReplacer {
    pub env: BundlerEnvoirment,
}

impl Replacer for AttrReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let name = caps.get(1).unwrap().as_str();
        dst.push_str(self.env[name.to_string()].to_string().as_str())
    }
}

impl MarAttribute for Skip {
    fn execute(
        &self,
        contents: &mut IntoIter<String>,
        _env: &mut BundlerEnvoirment,
    ) -> Result<(), String> {
        let mut source = contents.collect::<Vec<String>>();
        let mut line_num: usize = 0;
        for line in source.clone() {
            if let Some((attr, value)) = parse_attr_line(&line) {
                if attr == "skip" {
                    source.remove(line_num + 1);
                } else if attr == "skip-if" {
                    let regex = Regex::new("\\{(.*?)}").unwrap();

                    let real_value = regex
                        .replace_all(&value, AttrReplacer { env: _env.clone() })
                        .to_string();

                    let boolean = eval::eval(&real_value).unwrap();

                    if eval::to_value(true) == boolean {
                        source.remove(line_num + 1);
                    }
                }
                source.remove(line_num);
            }
            line_num += 1;
        }

        *contents = source.into_iter();

        Ok(())
    }
}
