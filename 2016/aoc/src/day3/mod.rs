mod input;

use self::input::get_input;
use geometry::triangle::sides_can_form_triangle;

fn count_valid_triangles<'a, I>(input: I) -> usize
    where I: Iterator<Item = &'a (u32, u32, u32)>
{
    input.filter_map(|&(a, b, c)| if sides_can_form_triangle(a, b, c) {
            Some((a, b, c))
        } else {
            None
        })
        .collect::<Vec<(u32, u32, u32)>>()
        .len()
}
/// How many triples in the day three input are valid triangles?
pub fn do_day3() {
    let valid_triangles = count_valid_triangles(get_input().iter());

    println!("valid triangles: {}", valid_triangles);

    let rotated_input = rotate_input(&get_input());
    let valid_triangles = count_valid_triangles(rotated_input.iter());

    println!("valid triangles in rotated input: {}", valid_triangles);
}

/// Rotate 3-by-3 chunks, by column
/// Can't return an iterator because we don't have "impl trait" in stable yet
/// which means we have to specify an explicit iterator type which is deeply
/// unpleasant in this case.
fn rotate_input<'a, T>(input: &'a [(T, T, T)]) -> Vec<(T, T, T)>
    where T: Clone
{
    input.chunks(3)
        // pretty sure I can do this in one flat_map call but can't get the lifetime to work out
        .map(|x| {
            let (a1, b1, c1) = x[0].clone();
            let (a2, b2, c2) = x[1].clone();
            let (a3, b3, c3) = x[2].clone();

            vec![(a1, a2, a3), (b1, b2, b3), (c1, c2, c3)]
        })
        .flat_map(|v| v.into_iter())
        .collect()
}
