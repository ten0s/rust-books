use std::fmt::Display;

fn largest_i32(list: &[i32]) -> Option<i32> {
    if list.len() == 0 {
        return None;
    }

    let mut largest = list[0];

    for &item in list.iter().skip(1) {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

fn largest_char(list: &[char]) -> Option<char> {
    if list.len() == 0 {
        return None;
    }

    let mut largest = list[0];

    for &item in list.iter().skip(1) {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.len() == 0 {
        return None;
    }

    let mut largest = &list[0];

    for item in list.iter().skip(1) {
        if item > largest {
            largest = &item;
        }
    }

    Some(&largest)
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn point(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md
/*
impl Point<i32, i32> {
    fn point(&self) -> (i32, i32) {
        println!("point<i32, i32>");
        (&self.x, &self.y)
    }
}
*/

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

// new implemented for all types
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// cmp_display implemented only for types implementing Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest member is x = {}", self.x);
        } else {
            println!("Largest member is y = {}", self.y);
        }
    }
}

/*
// Blanket implementation
Doesn't work here due to the coherence principle
impl<T: Display> ToString for T {}
*/

fn main() {
    let number_list = vec![32, 50, 25, 100, 65];
    let result = largest_i32(&number_list).unwrap();
    println!("The largest number is {}", result);

    let number_list = vec![6000, 102, 34, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list).unwrap();
    println!("The largest number is {}", result);

    let number_list = vec![];
    let default = largest_i32(&number_list).unwrap_or_default();
    let other = largest_i32(&number_list).unwrap_or_else(|| -1);
    println!("The largest number is {}", default);
    println!("The largest number is {}", other);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list).unwrap();
    println!("The largest char is {}", result);

    let both_int = Point { x: 5, y: 10 };
    let point = both_int.point();
    println!("point {:?}", point);
    let both_float = Point { x: 1.0, y: 4.0 };
    let point = both_float.point();
    println!("point {:?}", point);
    let int_and_float = Point { x: 5, y: 4.0 };
    let point = int_and_float.point();
    println!("point {:?}", point);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let number_list = vec![32, 50, 25, 100, 65];
    let result = largest(&number_list).unwrap();
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list).unwrap();
    println!("The largest char is {}", result);

    let point_list = vec![Point { x: 0, y: 0 }, Point { x: 1, y: 1 }];
    let result = largest(&point_list).unwrap();
    println!("The largest point is {:?}", result);
}
