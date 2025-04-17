#[macro_export]
macro_rules! options {
    ($($item:expr),*) => {
        vec![
            $(Box::new($item) as Box<dyn ToString>),*
        ]
    };
}