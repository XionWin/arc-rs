#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

#[macro_export]
macro_rules! expect {
    ($t: expr) => {{
        match $t {
            Ok(v) => v,
            Err(msg) => {
                util::print_panic!(
                    "Error in {} (in {} [{}:{}:{}]) Message: {}",
                    util::function!(),
                    module_path!(),
                    file!(),
                    line!(),
                    column!(),
                    msg
                );
            }
        }
    }};
}
