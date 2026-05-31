use std::collections::HashMap;

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::Mutex;

    fn double(x: i32) -> i32 {
        x * 2
    }

    #[test]
    fn test_double() {
        // "`fn(i32) -> i32 {ch02::double}` doesn't implement `Debug`"
        // assert_eq!(double, vec![double][0]);
        // "binary operation `==` cannot be applied to type `fn(i32) -> i32 {ch02::double}`"
        // assert!(double == vec![double][0]);
        // "binary operation `==` cannot be applied to type `&fn(i32) -> i32 {ch02::double}`"
        // let rd = &double;
        // assert!(rd == vec![rd][0]);
        let double = |x: i32| x * 2;
        // "doesn't implement `Debug`""
        // assert_eq!(double, vec![double][0]);
        // "binary operation `==` cannot be applied to type `{closure@src/ch02.rs:11:22: 11:29}`"
        // assert!(double == vec![double][0]);
        // "binary operation `==` cannot be applied to type `&{closure@src/ch02.rs:11:22: 11:29}`"
        // let rd = &double;
        // assert!(rd == vec![rd][0]);
        let ptrd = &raw const double;
        assert!(ptrd == vec![ptrd][0]);
    }

    #[test]
    fn test_apply() {
        assert_eq!((|x: i32, y: i32| { x + y })(1, 2), 3);
    }

    #[test]
    fn test_map() {
        assert_eq!(
            [1, 2, 3].into_iter().map(|x| x + 10).collect::<Vec<_>>(),
            [11, 12, 13]
        );
        assert_eq!(
            [1, 2, 3]
                .into_iter()
                .zip([10, 100, 1000].into_iter())
                .map(|(x, y)| x + y)
                .collect::<Vec<_>>(),
            [11, 102, 1003]
        );
    }

    #[test]
    fn test_sort() {
        let mut a = [1, 4, 2, 5, 6, 7, 3];
        a.sort();
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_filter() {
        assert_eq!(
            (1..8).filter(|x| x % 2 == 1).collect::<Vec<_>>(),
            [1, 3, 5, 7]
        );
    }

    #[test]
    fn test_scope() {
        let y = 7;
        // "can't capture dynamic environment in a fn item
        // use the `|| { ... }` closure form instead"
        // fn scope_test(x: i32) -> [i32; 2] {
        //     [x, y]
        // }
        let scope_test = |x| [x, y];
        let y = 5;
        assert_eq!(scope_test(3), [3, 7]);

        fn list_plus(lst: &[i32], n: i32) -> Vec<i32> {
            lst.into_iter().map(|x| x + n).collect()
        }
        assert_eq!(list_plus(&[1, 2, 3], 10), [11, 12, 13]);

        let counter = Mutex::new(0);
        let new_id = || {
            *counter.lock().unwrap() += 1;
        };
        let reset_id = || {
            *counter.lock().unwrap() = 0;
        };
        new_id();
        new_id();
        assert_eq!(*counter.lock().unwrap(), 2);
        reset_id();
        assert_eq!(*counter.lock().unwrap(), 0);
    }

    #[test]
    fn test_scope_adder() {
        fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
            move |x| x + n
        }
        let add2 = make_adder(2);
        let add10 = make_adder(10);
        assert_eq!(add2(5), 7);
        assert_eq!(add10(3), 13);

        fn make_adder_b(mut n: i32) -> impl FnMut(i32, bool) -> i32 {
            move |x, change| {
                if change {
                    n = x;
                    n
                } else {
                    x + n
                }
            }
        }
        let mut addx = make_adder_b(1);
        assert_eq!(addx(3, false), 4);
        assert_eq!(addx(100, true), 100);
        assert_eq!(addx(3, false), 103);
    }

    fn make_dbms(data: HashMap<String, String>) -> impl Fn(String) -> Option<String> {
        move |key| data.get(&key).map(|s| s.clone())
    }
    #[test]
    fn test_dbms() {
        let cities = make_dbms(HashMap::from_iter([
            ("Boston".to_owned(), "US".to_owned()),
            ("Paris".to_owned(), "France".to_owned()),
        ]));
        assert_eq!(cities("Paris".to_owned()), Some("France".to_owned()));
    }
}
