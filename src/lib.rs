use std::collections::HashMap;
// C FFI functions
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// JSON processing
use serde::{Serialize, Deserialize};

// SystemVerilog parser
use sv_parser::{parse_sv, SyntaxTree, unwrap_node, Locate, RefNode};



// =======================================================================================
// ================================= C FFI FUNCTIONS =====================================
// =======================================================================================

// function to free the memory allocated
#[no_mangle]
pub extern "C" fn free_string(msg: *const c_char) {
    if msg.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(msg as *mut c_char);
    }
}

#[no_mangle]
pub extern "C" fn run_sv_parser(input: *const libc::c_char) -> *const libc::c_char {
    // parse the json string
    let input: Input = parse_input(input);
    let output = run_parser(input);
    let output = parse_output(output);

    // return the C string
    return CString::new(output).unwrap().into_raw();
}

// =======================================================================================
// ================================= HELPER STRUCTURES ===================================
// =======================================================================================

#[derive(Serialize, Deserialize)]
struct Input {
    pub files: FilesInput,
}

#[derive(Serialize, Deserialize)]
struct FilesInput {
    pub include: Vec<String>,
    pub source: Vec<String>,
}

// ------------------------------

#[derive(Serialize, Deserialize)]
struct Output {
    pub modules: Modules,
}

#[derive(Serialize, Deserialize)]
struct Modules {
    pub declarations: HashMap<String, ModuleInfo>,
    pub instances: HashMap<String, Vec<ModuleInfo>>,
    pub exports: HashMap<String, ModuleInfo>,
    pub missing: HashMap<String, Vec<ModuleInfo>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ModuleInfo {
    pub name: String,
    pub file: String,
    pub line: u32,
}

// =======================================================================================
// ================================= HELPER FUNCTIONS ====================================
// =======================================================================================

fn parse_input(input: *const libc::c_char) -> Input {
    let input = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input).to_str().expect("Invalid UTF-8")
    };

    return serde_json::from_str(input).unwrap();
}

fn parse_output(output: Output) -> String {
    return serde_json::to_string(&output).unwrap();
}

fn run_parser(data: Input) -> Output {
    // output
    let mut output = Output {
        modules: Modules {
            declarations: HashMap::new(),
            instances: HashMap::new(),
            exports: HashMap::new(),
            missing: HashMap::new(),
        },
    };
    
    // vector string
    let mut defines = HashMap::new();

    // parse the files into declarations and instances
    for path in &data.files.source {
        match parse_sv(&path, &defines, &data.files.include, false, false) {
            Ok((syntax_tree, new_defines)) => {
                defines = new_defines;
                analyze_defs(&syntax_tree, &mut output);
            }
            Err(e) => {
                println!("Error parsing file: {}", path);
                println!("{:?}", e);
            }
        }
    }

    // account for exports and missing modules
    for module in output.modules.declarations.keys() {
        if !output.modules.instances.contains_key(module) {
            output.modules.exports.insert(
                module.clone(),
                output.modules.declarations.get(module).unwrap().clone(),
            );
        }
    }
    for module in output.modules.instances.keys() {
        if !output.modules.declarations.contains_key(module) {
            output.modules.missing.insert(
                module.clone(),
                output.modules.instances.get(module).unwrap().clone(),
            );
        }
    }
    
    return output;
}

fn analyze_defs(syntax_tree: &SyntaxTree, output: &mut Output) {
    // &SyntaxTree is iterable
    for node in syntax_tree {
        // The type of each node is RefNode
        match node {
            RefNode::ModuleDeclarationNonansi(x) => {
                // new module info
                let mut module_info = ModuleInfo {
                    name: String::new(),
                    file: String::new(),
                    line: 0,
                };
                
                // unwrap_node! gets the nearest ModuleIdentifier from x
				let id = match unwrap_node!(x, ModuleIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};
                let loc = syntax_tree.get_origin(&id);
                module_info.file = match loc {
                    None => String::new(),
                    Some(x) => x.0.clone().canonicalize().unwrap().to_str().unwrap().to_string(),
                };
                module_info.line = id.line;
                // Original string can be got by SyntaxTree::get_str(self, node: &RefNode)
                let id = match syntax_tree.get_str(&id) {
                    None => { continue; },
                    Some(x) => x
                };
                module_info.name = id.to_string();

                // add the module info to the output
                output.modules.declarations.insert(module_info.name.clone(), module_info);
            }
            RefNode::ModuleDeclarationAnsi(x) => {
                let mut module_info = ModuleInfo {
                    name: String::new(),
                    file: String::new(),
                    line: 0,
                };

				let id = match unwrap_node!(x, ModuleIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};
                let loc = syntax_tree.get_origin(&id);
                module_info.file = match loc {
                    None => String::new(),
                    Some(x) => x.0.clone().canonicalize().unwrap().to_str().unwrap().to_string(),
                };
                module_info.line = id.line;
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};
                module_info.name = id.to_string();

                // add the module info to the output
                output.modules.declarations.insert(module_info.name.clone(), module_info);
            }
            RefNode::ModuleInstantiation(x) => {
                let mut module_info = ModuleInfo {
                    name: String::new(),
                    file: String::new(),
                    line: 0,
                };

				// write the module name
				let id = match unwrap_node!(x, ModuleIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};
                let loc = syntax_tree.get_origin(&id);
                module_info.file = match loc {
                    None => String::new(),
                    Some(x) => x.0.clone().canonicalize().unwrap().to_str().unwrap().to_string(),
                };
                module_info.line = id.line;
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};
                module_info.name = id.to_string();

                // add the module info to the output instances
                if output.modules.instances.contains_key(&module_info.name) {
                    output.modules.instances.get_mut(&module_info.name).unwrap().push(module_info);
                } else {
                    output.modules.instances.insert(module_info.name.clone(), vec![module_info]);
                }
			}
            _ => (),
        }
    }
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    // unwrap_node! can take multiple types
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        Some(RefNode::EscapedIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        _ => None,
    }
}