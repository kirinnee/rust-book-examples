use std::ops::Add;

#[cfg(test)]
mod test {
    use super::*;

    mod square_tests {
        use super::*;

        #[test]
        fn area_of_square() {
            let s1 = Square::new(5);
            let s2 = Square::new(6);

            assert_eq!(s1.area(), 25);
            assert_eq!(s2.area(), 36);
        }

        #[test]
        fn perimeter_of_square() {
            let s1 = Square::new(12);
            let s2 = Square::new(1000);

            assert_eq!(s1.perimeter(), 48);
            assert_eq!(s2.perimeter(), 4000);
        }

        #[test]
        fn add_square_same_width() {
            let s1 = Square::new(5);
            let s2 = Square::new(5);

            let expected = Rect::new(10, 5);

            assert_eq!(s1 + s2, Some(expected));
        }


        #[test]
        fn add_square_diff_width_should_return_none() {
            let s1 = Square::new(5);
            let s2 = Square::new(6);

            assert_eq!(s1 + s2, None);
        }

        #[test]
        fn add_square_diff_width_and_height_from_rect_should_return_none() {
            let s1 = Square::new(5);
            let s2 = Rect::new(6, 7);

            assert_eq!(s1 + s2, None);
        }

        #[test]
        fn add_square_same_width_and_diff_height_from_rect() {
            let s = Square::new(5);
            let r = Rect::new(5, 7);

            let expected = Rect::new(5, 12);
            assert_eq!(s + r, Some(expected));
        }

        #[test]
        fn add_square_diff_width_and_same_height_from_rect() {
            let s1 = Rect::new(5, 6);
            let s2 = Square::new(6);

            let expected = Rect::new(11, 6);
            assert_eq!(s1 + s2, Some(expected));
        }

        #[test]
        fn add_square_same_width_same_height_from_rect() {
            let s1 = Rect::new(7, 7);
            let s2 = Square::new(7);

            let expected = Rect::new(14, 7);
            assert_eq!(s1 + s2, Some(expected))
        }
    }

    mod rect_test {
        use super::*;

        #[test]
        fn area_of_rect() {
            let r1 = Rect::new(4, 5);
            let r2 = Rect::new(90, 70);

            assert_eq!(r1.area(), 20);
            assert_eq!(r2.area(), 6300);
        }

        #[test]
        fn perimeter_of_rect() {
            let r1 = Rect::new(50, 200);
            let r2 = Rect::new(1, 2);

            assert_eq!(r1.perimeter(), 500);
            assert_eq!(r2.perimeter(), 6);
        }

        #[test]
        fn add_rect_with_same_width_diff_height() {
            let r1 = Rect::new(5, 10);
            let r2 = Rect::new(5, 20);

            let expected = Rect::new(5, 30);
            assert_eq!(r1 + r2, Some(expected));
        }

        #[test]
        fn add_rect_with_diff_width_same_height() {
            let r1 = Rect::new(5, 15);
            let r2 = Rect::new(10, 15);

            let expected = Rect::new(15, 15);
            assert_eq!(r1 + r2, Some(expected));
        }

        #[test]
        fn add_rect_with_diff_width_diff_height() {
            let r1 = Rect::new(5, 10);
            let r2 = Rect::new(15, 20);

            assert_eq!(r1 + r2, None);
        }


        #[test]
        fn add_rect_with_same_width_same_height() {
            let r1 = Rect::new(5, 10);
            let r2 = Rect::new(5, 10);

            let expected = Rect::new(10, 10);
            assert_eq!(r1 + r2, Some(expected));
        }

        #[test]
        fn add_square_same_as_width_but_diff_height() {
            let r = Rect::new(5, 10);
            let s = Square::new(5);

            let expected = Rect::new(5, 15);
            assert_eq!(r + s, Some(expected));
        }

        #[test]
        fn add_square_same_as_height_but_diff_width() {
            let r = Rect::new(5, 10);
            let s = Square::new(10);

            let expected = Rect::new(15, 10);
            assert_eq!(r + s, Some(expected));
        }

        #[test]
        fn add_square_diff_both_height_and_width() {
            let r = Rect::new(5, 10);
            let s = Square::new(7);

            assert_eq!(r + s, None);
        }


        #[test]
        fn add_square_same_both_height_and_width() {
            let r = Rect::new(10, 10);
            let s = Square::new(10);

            let expected = Rect::new(20, 10);
            assert_eq!(r + s, Some(expected));
        }
    }
}


trait Shape {
    fn area(&self) -> u64;
    fn perimeter(&self) -> u64;
}

#[derive(Debug, PartialEq)]
struct Rect {
    width: u64,
    height: u64,
}

#[derive(Debug, PartialEq)]
struct Square {
    width: u64,
}

impl Rect {
    fn new(width: u64, height: u64) -> Rect {
        Rect { width, height }
    }
}

impl Square {
    fn new(width: u64) -> Square {
        Square { width }
    }
}

impl Shape for Square {
    fn area(&self) -> u64 {
        self.width * self.width
    }

    fn perimeter(&self) -> u64 {
        self.width * 4
    }
}

impl Shape for Rect {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn perimeter(&self) -> u64 {
        self.width * 2 + self.height * 2
    }
}

impl Add for Square {
    type Output = Option<Rect>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.width == rhs.width {
            Some(Rect { width: self.width + rhs.width, height: self.width })
        } else {
            None
        }
    }
}

impl Add<Rect> for Square {
    type Output = Option<Rect>;

    fn add(self, rhs: Rect) -> Self::Output {
        if self.width == rhs.height {
            Some(Rect {
                width: self.width + rhs.width,
                height: self.width,
            })
        } else if self.width == rhs.width {
            Some(Rect {
                width: self.width,
                height: self.width + rhs.height,
            })
        } else {
            None
        }
    }
}

impl Add for Rect {
    type Output = Option<Rect>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.height == rhs.height {
            Some(Rect {
                width: self.width + rhs.width,
                height: self.height,
            })
        } else if self.width == rhs.width {
            Some(Rect {
                width: self.width,
                height: self.height + rhs.height,
            })
        } else {
            None
        }
    }
}

impl Add<Square> for Rect {
    type Output = Option<Rect>;

    fn add(self, rhs: Square) -> Self::Output {
        if self.height == rhs.width {
            Some(Rect {
                height: self.height,
                width: self.width + rhs.width,
            })
        } else if self.width == rhs.width {
            Some(Rect {
                height: self.height + rhs.width,
                width: self.width,
            })
        } else {
            None
        }
    }
}

fn main() {}



