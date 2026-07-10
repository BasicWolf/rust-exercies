use orderable_person::OrderablePerson;

mod insertion_sort;
mod orderable_person;

fn main() {}

#[test]
fn f001_hello_world() {
    println!("Hello, world!");
}

#[test]
fn f002_math_power() {
    assert!(2u32.pow(2u32) == 4, "2^2 == 4");
    assert!(2f32.powi(-2i32) == 0.25, "2f32^-2i32 == 0.25");
    // but we cannot just do 2.pow(2), because type is ambiguous
    // hence, compiler can't understand which pow() it should use.
}

#[test]
fn f003_for_loop() {
    let mut j: i32 = 0;
    for i in 0..10 {
        if i == 9 {
            j = 1;
        }
    }
    assert!(1 == j);

    for i in (0..=10).rev() {
        if i == 10 {
            j = 2;
        }
    }
    assert!(2 == j);
}

#[test]
fn f004_insertion_sort_vector_of_integers() {
    let mut input_vec = vec![3, 2, 1, 4, 6, 5, 7, 9, 8];
    let v: &mut Vec<i32> = &mut input_vec;
    for i in 1..v.len() {
        if v[i] < v[i - 1] {
            for j in (1..=i).rev() {
                if v[j] < v[j - 1] {
                    println!("Swapping v_{} = {}, v_{} = {}", j, v[j], j - 1, v[j - 1]);
                    let vj = v[j];
                    v[j] = v[j - 1];
                    v[j - 1] = vj;
                }
            }
        }
    }

    assert_eq!(*v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn f005_sort_with_ord() {
    let mut persons = vec![
        OrderablePerson {
            name: "Alex".to_string(),
            age: 30,
        },
        OrderablePerson {
            name: "Bart".to_string(),
            age: 28,
        },
        OrderablePerson {
            name: "Cal".to_string(),
            age: 29,
        },
    ];

    persons.sort();

    let sorted_ages: Vec<u8> = persons.iter().map(|p| p.age).collect();
    assert_eq!(vec![28, 29, 30], sorted_ages);
}

#[test]
fn f006_custom_generic_sort_function() {
    let mut v = vec![9, 8, 7, 1, 3, 2];
    insertion_sort::insertion_sort(&mut v);
    println!("v is {:?}", v);
}
