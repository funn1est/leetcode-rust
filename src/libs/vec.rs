#[macro_export]
macro_rules! vec_of_string {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
