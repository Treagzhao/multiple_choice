extern crate multiple_choice;

use multiple_choice::triple_verify;

#[derive(PartialEq, Debug, Clone)]
struct TestStruct {
    id: i32,
    name: String,
    value: f64,
    active: bool,
}

#[triple_verify]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[triple_verify]
fn create_tuple(a: i32, b: &str) -> (i32, String) {
    (a, b.to_string())
}

#[triple_verify]
fn create_array(a: i32) -> [i32; 3] {
    [a, a, a]
}

#[triple_verify]
fn create_test_struct(id: i32, name: &str, value: f64, active: bool) -> TestStruct {
    TestStruct {
        id,
        name: name.to_string(),
        value,
        active,
    }
}

#[triple_verify]
fn create_struct_tuple(a: &TestStruct, b: &TestStruct) -> (TestStruct, TestStruct) {
    (a.clone(), b.clone())
}

#[triple_verify]
fn create_struct_vec(count: usize, base_id: i32) -> Vec<TestStruct> {
    (0..count)
        .map(|i| TestStruct {
            id: base_id + i as i32,
            name: format!("Test {}", i),
            value: i as f64 * 0.5,
            active: i % 2 == 0,
        })
        .collect()
}

#[triple_verify]
fn create_struct_array() -> [TestStruct; 2] {
    [
        TestStruct {
            id: 1,
            name: "First".to_string(),
            value: 1.1,
            active: true,
        },
        TestStruct {
            id: 2,
            name: "Second".to_string(),
            value: 2.2,
            active: false,
        },
    ]
}

use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[triple_verify]
fn different_value() -> i32 {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    match count {
        0 => 1,
        1 => 2,
        _ => 3,
    }
}

fn main() {
    println!("Testing add function: {}", add(2, 3));
    println!(
        "Testing create_tuple function: {:?}",
        create_tuple(42, "test")
    );
    println!("Testing create_array function: {:?}", create_array(10));
    println!(
        "Testing create_test_struct function: {:?}",
        create_test_struct(1, "test", 3.14, true)
    );

    let struct1 = create_test_struct(1, "First", 1.1, true);
    let struct2 = create_test_struct(2, "Second", 2.2, false);
    println!(
        "Testing create_struct_tuple function: {:?}",
        create_struct_tuple(&struct1, &struct2)
    );

    println!(
        "Testing create_struct_vec function: {:?}",
        create_struct_vec(3, 10)
    );

    println!(
        "Testing create_struct_array function: {:?}",
        create_struct_array()
    );

    // Test panic with function name
    println!("Testing panic with function name...");
    // different_value(); // This should panic
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_match() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic]
    fn test_all_different() {
        different_value();
    }

    #[test]
    fn test_tuple_return() {
        assert_eq!(create_tuple(42, "test"), (42, "test".to_string()));
    }

    #[test]
    fn test_array_return() {
        assert_eq!(create_array(10), [10, 10, 10]);
    }

    #[test]
    fn test_struct_return() {
        let expected = TestStruct {
            id: 1,
            name: "test".to_string(),
            value: 3.14,
            active: true,
        };
        assert_eq!(create_test_struct(1, "test", 3.14, true), expected);
    }

    #[test]
    fn test_struct_tuple_return() {
        let struct1 = TestStruct {
            id: 1,
            name: "First".to_string(),
            value: 1.1,
            active: true,
        };
        let struct2 = TestStruct {
            id: 2,
            name: "Second".to_string(),
            value: 2.2,
            active: false,
        };
        let expected = (struct1.clone(), struct2.clone());
        assert_eq!(create_struct_tuple(&struct1, &struct2), expected);
    }

    #[test]
    fn test_struct_vec_return() {
        let result = create_struct_vec(3, 10);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, 10);
        assert_eq!(result[1].id, 11);
        assert_eq!(result[2].id, 12);
    }

    #[test]
    fn test_struct_array_return() {
        let result = create_struct_array();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[1].id, 2);
    }
}
