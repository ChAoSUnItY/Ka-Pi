#[macro_export]
macro_rules! cp {
    ($($id:ident).+) => {
        vec![$(stringify!($id),)*].join("/")
    };
    ($($id:ident)::+) => {
        vec![$(stringify!($id),)*].join("/")
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_class_path_macro() {
        let single_id = cp!(java);
        
        let multiple_dot_id = cp!(java.lang.Object);
        let multiple_double_colon_id = cp!(java::lang::String);

        assert_eq!(single_id, "java");
        assert_eq!(multiple_dot_id, "java/lang/Object");
        assert_eq!(multiple_double_colon_id, "java/lang/String");
    }
}
