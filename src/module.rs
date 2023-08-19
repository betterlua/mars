use std::fs;

use crate::{attributes::MarAttribute, BundlerEnvoirment};

#[derive(Debug)]
pub struct LuaModule {
    pub name: String,
    pub path: String,

    pub contents: Vec<String>,
    pub output: String,
}

impl LuaModule {
    pub fn new(path: &str) -> Self {
        let name = path
            .split("/")
            .last()
            .unwrap()
            .strip_suffix(".lua")
            .unwrap();
        let contents = fs::read_to_string(&path).unwrap();
        let contents = contents
            .split("\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        LuaModule {
            name: name.to_string(),
            path: path.to_string(),
            contents,
            output: "".to_string(),
        }
    }

    pub fn execute(
        &mut self,
        env: &mut BundlerEnvoirment,
        attrs: Vec<Box<dyn MarAttribute>>,
    ) -> Result<(), String> {
        let mut source = self.contents.clone().into_iter();

        for attr in attrs.into_iter() {
            attr.execute(&mut source, env)?;
        }

        self.output = source.collect::<Vec<String>>().join("\n    ").replace("require", "___MAR_REQUIRE___");

        Ok(())
    }

    pub fn output(&self, entry: bool) -> String {
        let binding = path_to_module(&self.path);
        let module_name = binding.split("_").collect::<Vec<&str>>();
        let module_name = module_name[module_name.len() - 2];
        if entry {
            format!(
                r"---- {} ----
function entrypoint()
    {}
end

---- end {} ----
        ",
                path_to_module(&self.path),
                self.output,
                path_to_module(&self.path)
            )
        } else {
            format!(
                r"---- {} ----
___MAR_MODULES___['{}'] = (function()
    {}
end)()                
---- end {} ----
            ",
                path_to_module(&self.path),
                String::from(module_name),
                self.output,
                path_to_module(&self.path)
            )
        }
    }
}

pub fn path_to_module(path: &str) -> String {
    path.replace("/", "_").replace("-", "_").replace(".", "_")
}
