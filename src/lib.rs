use std::{
    fs::{self, File},
    ops::{Index, IndexMut},
    path::Path,
};

use attributes::attributes::Skip;
use module::LuaModule;

pub mod attributes;
pub mod module;

#[derive(Debug)]
pub struct BuildOptions {
    pub entry_point: String,
    pub src_dir: String,
    pub r#type: BuildType,
    pub output_type: BuildOutputType,
    pub output_path: String,
    pub output_name: String,
}

#[derive(Debug)]
pub enum BuildType {
    Debug,
    Production,
}

#[derive(Debug)]
pub enum BuildOutputType {
    Binary,
    Library,
}

#[derive(Debug, Clone)]
pub struct BundlerEnvoirment {
    pub debug: bool,
}

impl Index<String> for BundlerEnvoirment {
    type Output = bool;

    fn index(&self, index: String) -> &Self::Output {
        match index.as_str() {
            "debug" => &self.debug,
            _ => panic!("Invalid index"),
        }
    }
}

impl IndexMut<String> for BundlerEnvoirment {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        match index.as_str() {
            "debug" => &mut self.debug,
            _ => panic!("Invalid index"),
        }
    }
}

impl BundlerEnvoirment {
    pub fn new(build_type: BuildType) -> Self {
        let debug = match build_type {
            BuildType::Debug => true,
            BuildType::Production => false,
        };

        BundlerEnvoirment { debug }
    }
}

pub fn build(options: BuildOptions) -> Result<(), String> {
    let BuildOptions {
        entry_point,
        src_dir,
        r#type,
        output_type,
        output_path,
        output_name,
    } = options;

    let mut env = BundlerEnvoirment::new(r#type);
    let mut modules: Vec<LuaModule> = vec![LuaModule::new(&entry_point)];

    // for

    for file in glob::glob(format!("./{}/{}", src_dir, "**/*.lua").as_str()).unwrap() {
        let file = file.unwrap();

        if file.to_str().unwrap() != entry_point {
            modules.push(LuaModule::new(file.to_str().unwrap()));
        }
    }

    let mut output_text = String::from(
        r"local ___MAR_MODULES___ = {}

function ___MAR_REQUIRE___(name)
    local module = ___MAR_MODULES___[name]
    if module == nil then
        return require(name)  
    end
    return module
end
",
    );

    let mut i = 0;
    for mut module in modules {
        module.execute(&mut env, vec![Box::new(Skip)]).unwrap();
        if i == 0 {
            output_text.push_str(format!("\n{}", module.output(true)).as_str());
        } else {
            output_text.push_str(format!("\n{}", module.output(false)).as_str());
        }
        i += 1;
    }

    let binary: bool = match output_type {
        BuildOutputType::Binary => true,
        BuildOutputType::Library => false,
    };

    if binary {
        output_text.push_str("\nentrypoint()");
        if !Path::new(&output_path).exists() {
            fs::create_dir_all(Path::new(&output_path)).unwrap();
        }
        File::create(format!("{}/{}", output_path, output_name)).unwrap();
        fs::write(format!("{}/{}", output_path, output_name), output_text).unwrap();
    }

    Ok(())
}
