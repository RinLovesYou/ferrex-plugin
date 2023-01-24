struct UnityAssembly {
    pointer: i32
}

impl UnityAssembly {
    fn new(pointer: i32) -> Self {
        UnityAssembly { 
            pointer 
        }
    }

    fn get_name(&self) -> String {
        let mut name = [0 ; 2048];

        let length = unsafe {
            fx_get_assembly_name(self.pointer, name.as_ptr() as i32)
        } as usize;

        if length <= 0 {
            return "".to_string();
        }

        unsafe {
            String::from_raw_parts(name.as_mut_ptr(), length, length)
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    log!("Hello from wasm!");
    
    let assemblies = get_assemblies();

    for asm in assemblies {
        log!(asm.get_name());
    }
}

#[macro_export]
macro_rules! log {
    //case 1: empty
    () => {{
        let string = std::format!("");
        unsafe {
            fx_log_str(string.as_ptr() as i32, string.len() as i32);
        }
    }};

    //case 2: single argument
    ($msg:expr) => {{
        let string = std::format!("{}", $msg);
        unsafe {
            fx_log_str(string.as_ptr() as i32, string.len() as i32);
        }
    }};

    //case 3: multiple arguments
    ($($arg:tt)*) => {{
        let msg = &format_args!($($arg)*).to_string();

        let string = std::format!("{}", msg);
        unsafe {
            fx_log_str(string.as_ptr() as i32, string.len() as i32);
        }
    }};
}

fn get_assemblies() -> Vec<UnityAssembly> {
    let assembly_count = unsafe {
        fx_get_assembly_count()
    };

    if assembly_count <= 0 {
        return Vec::new();
    }

    let mut assemblies: Vec<i32> = Vec::new();
    assemblies.resize(assembly_count as usize, 0);

    unsafe {
        fx_get_assemblies(assemblies.as_ptr() as usize as i32, assembly_count);
    }

    assemblies.iter().map(|ptr| UnityAssembly{pointer: ptr.to_owned()}).collect()
}

#[link(wasm_import_module = "")]
extern "C" {
    fn fx_get_assembly_count() -> i32;
    fn fx_get_assemblies(ptr: i32, len: i32);
    fn fx_get_assembly_name(assembly: i32, name: i32) -> i32;
    fn fx_log_str(ptr: i32, len: i32);
}