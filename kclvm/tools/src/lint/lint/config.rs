use super::super::checker::base_checker::Checker;
pub struct Config{
    check_list: Vec<Checker>,
    ignore: Vec<String>,
    max_line_length: u32,
    output: Vec<String>,
    output_path: Option<String>,
    module_naming_style: String,
    package_naming_style: String,
    schema_naming_style: String,
    mixin_naming_style: String,
    protocol_naming_style: String,
    argument_naming_style: String,
    variable_naming_style: String,
    schema_attribute_naming_style: String,
    module_rgx: Option<String>,
    package_rgx: Option<String>,
    schema_rgx: Option<String>,
    mixin_rgx: Option<String>,
    protocol_rgx: Option<String>,
    argument_rgx: Option<String>,
    variable_rgx: Option<String>,
    schema_attribute_rgx: Option<String>,
    bad_names: Vec<String>,
}
impl Config {
    pub fn DEFAULT_CONFIG() -> Config{
        Self { 
            check_list: vec![Checker::ImportCheck, Checker::BasicChecker, Checker::MiscChecker],
            ignore: vec![],
            max_line_length: 200,
            output: vec![String::from("stdout")],
            output_path: None,
            module_naming_style: String::from("ANY"),
            package_naming_style: String::from("ANY"),
            schema_naming_style: String::from("PascalCase"),
            mixin_naming_style: String::from("PascalCase"),
            protocol_naming_style: String::from("PascalCase"),
            argument_naming_style: String::from("camelCase"),
            variable_naming_style: String::from("ANY"),
            schema_attribute_naming_style: String::from("ANY"),
            module_rgx: None,
            package_rgx: None,
            schema_rgx: None,
            mixin_rgx: None,
            protocol_rgx: None,
            argument_rgx: None,
            variable_rgx: None,
            schema_attribute_rgx: None,
            bad_names: vec![
                String::from("foo"),
                String::from("bar"),
                String::from("baz"),
                String::from("toto"),
                String::from("tutu"),
                String::from("I"),
                String::from("l"),
                String::from("O"),
            ]
        }
    }
    pub fn update(&mut self){
        // todo
    }
}


// pub DEFAULT_CONFIG: Config = Config {
//     check_list: vec![Checker::ImportCheck, Checker::BasicChecker, Checker::MiscChecker],
//     ignore: vec![],
//     max_line_length: 200,
//     output: vec![String::from("stdout")],
//     output_path: None,
//     module_naming_style: String::from("ANY"),
//     package_naming_style: String::from("ANY"),
//     schema_naming_style: String::from("PascalCase"),
//     mixin_naming_style: String::from("PascalCase"),
//     protocol_naming_style: String::from("PascalCase"),
//     argument_naming_style: String::from("camelCase"),
//     variable_naming_style: String::from("ANY"),
//     schema_attribute_naming_style: String::from("ANY"),
//     module_rgx: None,
//     package_rgx: None,
//     schema_rgx: None,
//     mixin_rgx: None,
//     protocol_rgx: None,
//     argument_rgx: None,
//     variable_rgx: None,
//     schema_attribute_rgx: None,
//     bad_names: vec![
//         String::from("foo"),
//         String::from("bar"),
//         String::from("baz"),
//         String::from("toto"),
//         String::from("tutu"),
//         String::from("I"),
//         String::from("l"),
//         String::from("O"),
//     ]
// };
