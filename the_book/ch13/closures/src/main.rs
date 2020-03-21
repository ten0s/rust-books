use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<Fun, A, R>
where
    Fun: Fn(&A) -> R,
{
    fun: Fun,
    map: HashMap<A, R>,
}

impl<Fun, A: Eq + Hash + Copy, R: Copy> Cacher<Fun, A, R>
where
    Fun: Fn(&A) -> R,
{
    fn new(fun: Fun) -> Self {
        Self {
            fun,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &A) -> &R {
        match self.map.get(arg) {
            Some(&v) => &v,
            None => {
                let v = (self.fun)(arg);
                self.map.insert(*arg, v);
                &v
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: &u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num.clone()
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(&intensity)
            )
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_diff_values() {
        let mut c = Cacher::new(|x| x);

        let v1 = c.value(&1);
        let v2 = c.value(&2);

        assert_eq!(*v2, &2);
    }

    #[test]
    fn call_with_diff_types() {
        let mut c1 = Cacher::new(|x| 2 * x);
        let v1 = c1.value(&1);
        assert_eq!(*v1, 2);

        /*
        let mut c2 = Cacher::new(|x| x);
        let v2 = c2.value("hello");
        assert_eq!(v2, "hello");
        */
    }
}
