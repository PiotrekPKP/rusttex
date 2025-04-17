#[macro_export]
/// A macro to easily mix Strings and enums as options
macro_rules! options {
    ($($item:expr),*) => {
        vec![
            $(Box::new($item) as Box<dyn ToString>),*
        ]
    };
}
