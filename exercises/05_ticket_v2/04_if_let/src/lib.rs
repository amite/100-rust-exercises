enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    /*
    In Rust, when you destructure an enum variant, you create
    bindings for its fields. In this case, radius becomes a local
    variable holding the value of the circle's radius.
    This allows you to use *radius to dereference it
    (since it's a reference due to the method taking &self)
    and obtain the actual value of type f64.
    */
    // pub fn radius(&self) -> f64 {
    //     if let Shape::Circle { radius } = self {
    //         *radius
    //     } else {
    //         panic!("not a circle")
    //     }
    // }

    pub fn radius(&self) -> f64 {
        let Shape::Circle { radius } = self else {
            panic!("not a circle")
        };

        *radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let _ = Shape::Circle { radius: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_square() {
        let _ = Shape::Square { border: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_rectangle() {
        let _ = Shape::Rectangle {
            width: 1.0,
            height: 2.0,
        }
        .radius();
    }
}
