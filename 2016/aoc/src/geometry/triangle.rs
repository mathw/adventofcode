/// Use the triangle inequality theorem to see if three side lengths can form a triangle
pub fn sides_can_form_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}





#[test]
fn test_sides_can_form_triangle() {
    fn check_sides_can_form_triangle(a: u32, b: u32, c: u32, expected: bool) {
        if expected {
            assert!(sides_can_form_triangle(a, b, c));
            assert!(sides_can_form_triangle(b, c, a));
            assert!(sides_can_form_triangle(c, a, b));
            assert!(sides_can_form_triangle(b, a, c));
            assert!(sides_can_form_triangle(c, b, a));
        } else {
            assert!(!sides_can_form_triangle(a, b, c));
            assert!(!sides_can_form_triangle(b, c, a));
            assert!(!sides_can_form_triangle(c, a, b));
            assert!(!sides_can_form_triangle(b, a, c));
            assert!(!sides_can_form_triangle(c, b, a));
        }
    }
    check_sides_can_form_triangle(3, 4, 5, true);
    check_sides_can_form_triangle(1, 2, 3, false);
}
