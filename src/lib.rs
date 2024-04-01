use std::collections::HashMap;
// C FFI functions
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// JSON processing
use serde::{Serialize, Deserialize};

// SystemVerilog parser
use sv_parser::{parse_sv, SyntaxTree, unwrap_node, Locate, RefNode, Define, DefineText};

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

    // convert the struct to a json string
    let new_json = serde_json::to_string(&input).unwrap();

    run_parser(input);

    // convert the json string to a C string
    let c_str = CString::new(new_json).unwrap();

    // return the C string
    return c_str.into_raw();    
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

struct Output {
    pub modules: Modules,
}

struct Modules {
    pub declarations: HashMap<String, ModuleInfo>,
    pub instances: HashMap<String, Vec<String>>,
}

struct ModuleInfo {
    pub name: String,
    pub ports: Vec<String>,
    pub instances: Vec<String>,
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

fn run_parser(data: Input) -> Output {
    // output
    let mut output = Output {
        modules: Modules {
            declarations: HashMap::new(),
            instances: HashMap::new(),
        },
    };
    
    // vector string
    let mut defines = HashMap::new();


    for path in &data.files.source {
        match parse_sv(&path, &defines, &data.files.include, false, false) {
            Ok((syntax_tree, new_defines)) => {
                defines = new_defines;
                analyze_defs(&syntax_tree);
            }
            Err(e) => {
                println!("Error parsing file: {}", path);
                println!("{:?}", e);
            }
        }
    }
    
    return output;
}

fn analyze_defs(syntax_tree: &SyntaxTree) {
    // &SyntaxTree is iterable
    for node in syntax_tree {
        // The type of each node is RefNode
        match node {
            RefNode::ModuleDeclarationNonansi(x) => {
                // unwrap_node! gets the nearest ModuleIdentifier from x
				let id = match unwrap_node!(x, ModuleIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};				
                // Original string can be got by SyntaxTree::get_str(self, node: &RefNode)
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};	
                // Declare the new module
				println!("      - mod_name: {}", escape_str(id));
				println!("        insts:");
            }
            RefNode::ModuleDeclarationAnsi(x) => {
				let id = match unwrap_node!(x, ModuleIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};		
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};	
				println!("      - mod_name: {}", escape_str(id));
				println!("        insts:");
            }
            RefNode::PackageDeclaration(x) => {
				let id = match unwrap_node!(x, PackageIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};		
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};	
				println!("      - pkg_name: {}", escape_str(id));
				println!("        insts:");
            }
            RefNode::InterfaceDeclaration(x) => {
				let id = match unwrap_node!(x, InterfaceIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};		
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};
				println!("      - intf_name: {}", escape_str(id));
				println!("        insts:");
            }
            RefNode::ModuleInstantiation(x) => {
				// write the module name
				let id = match unwrap_node!(x, ModuleIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};		
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};
                println!("          - mod_name: {}", escape_str(id));
                // write the instance name
				let id = match unwrap_node!(x, InstanceIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};		
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};
                println!("            inst_name: {}", escape_str(id));
			}
            RefNode::PackageImportItem(x) => {
				// write the package name
				let id = match unwrap_node!(x, PackageIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};		
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};
                println!("          - pkg_name: {}", escape_str(id));
			}
			RefNode::ImplicitClassHandleOrClassScope(x) => {
				// write the package name
				let id = match unwrap_node!(x, ClassIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};		
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};
                println!("          - pkg_name: {}", escape_str(id));
			}
			RefNode::ImplicitClassHandleOrClassScopeOrPackageScope(x) => {
				// write the package name
				let id = match unwrap_node!(x, ClassIdentifier) {
					None => { continue; },
					Some(x) => x
				};
				let id = match get_identifier(id) {
					None => { continue; },
					Some(x) => x
				};
                let id = match syntax_tree.get_str(&id) {
					None => { continue; },
					Some(x) => x
				};
                println!("          - pkg_name: {}", escape_str(id));
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

fn escape_str(v: &str) -> String {
    let mut wr = String::new();
    
    wr.push_str("\"");

    let mut start = 0;

    for (i, byte) in v.bytes().enumerate() {
        let escaped = match byte {
            b'"' => "\\\"",
            b'\\' => "\\\\",
            b'\x00' => "\\u0000",
            b'\x01' => "\\u0001",
            b'\x02' => "\\u0002",
            b'\x03' => "\\u0003",
            b'\x04' => "\\u0004",
            b'\x05' => "\\u0005",
            b'\x06' => "\\u0006",
            b'\x07' => "\\u0007",
            b'\x08' => "\\b",
            b'\t' => "\\t",
            b'\n' => "\\n",
            b'\x0b' => "\\u000b",
            b'\x0c' => "\\f",
            b'\r' => "\\r",
            b'\x0e' => "\\u000e",
            b'\x0f' => "\\u000f",
            b'\x10' => "\\u0010",
            b'\x11' => "\\u0011",
            b'\x12' => "\\u0012",
            b'\x13' => "\\u0013",
            b'\x14' => "\\u0014",
            b'\x15' => "\\u0015",
            b'\x16' => "\\u0016",
            b'\x17' => "\\u0017",
            b'\x18' => "\\u0018",
            b'\x19' => "\\u0019",
            b'\x1a' => "\\u001a",
            b'\x1b' => "\\u001b",
            b'\x1c' => "\\u001c",
            b'\x1d' => "\\u001d",
            b'\x1e' => "\\u001e",
            b'\x1f' => "\\u001f",
            b'\x7f' => "\\u007f",
            _ => continue,
        };

        if start < i {
            wr.push_str(&v[start..i]);
        }

        wr.push_str(escaped);

        start = i + 1;
    }

    if start != v.len() {
        wr.push_str(&v[start..]);
    }

    wr.push_str("\"");
    
    wr
}