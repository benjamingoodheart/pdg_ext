use pgrx::prelude::*;
use std::collections::HashMap;

::pgrx::pg_module_magic!(name, version);


#[pg_extern]
fn hello_pdg_ext() -> &'static str {
    "Hello, pdg_ext"
}

#[pg_extern]
fn dedupe_int_array<i32: std::cmp::PartialEq + Clone>(arr: Vec<i32>) -> Vec<i32>{
        let iterator = arr.into_iter();
        let mut map = HashMap::new();
        let mut seen = vec![];
        let mut i = 0;
        for x in iterator {
            map.insert(i, x);
            i = i + 1;
        }

        for y in map.values() {
            if !seen.contains(y) {
                seen.push(y.clone());
            }
        }
       seen
    }

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pdg_ext() {
        assert_eq!("Hello, pdg_ext", crate::hello_pdg_ext());
    }

    #[pg_test]
    fn test_dedupe_int_array(){
        let vec_ints = vec![1,2,3,4,5,5];
        let res = crate::dedupe_int_array(vec_ints);
        assert_eq!(res.contains(&5),true);
        assert_eq!(res.len(), 5);
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
