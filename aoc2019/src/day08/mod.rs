use crate::day::Day;
use itertools::Itertools;

pub struct Day8 {
    image_layers: Vec<Vec<u8>>,
}

impl Day8 {
    pub fn new() -> Result<Day8, String> {
        let input = include_str!("input.txt");
        let width = 25;
        let height = 6;
        Ok(Day8 {
            image_layers: parse_layers(input, width, height)?,
        })
    }
}

impl Day for Day8 {
    fn part1(&mut self) -> Result<String, String> {
        let fewest_zeroes =
            layer_with_fewest(self.image_layers.as_slice(), &0).ok_or("No layers!".to_owned())?;
        let ones = count_digits(fewest_zeroes, &1);
        let twos = count_digits(fewest_zeroes, &2);
        Ok(format!("{}", ones * twos))
    }

    fn part2(&mut self) -> Result<String, String> {
        let pixel_layers = self
            .image_layers
            .iter()
            .map(|l| layer_to_pixels(l.as_slice()))
            .collect::<Result<Vec<Vec<Pixel>>, String>>()?;
        let result = stack_all_layers(pixel_layers.as_slice());
        let result_string = result
            .iter()
            .chunks(25)
            .into_iter()
            .map(|line| {
                line.map(|pixel| match pixel {
                    Pixel::White => '\u{2588}',
                    Pixel::Black => ' ',
                    Pixel::Transparent => ' ',
                })
                .collect::<String>()
            })
            .intersperse("\n".to_owned())
            .collect::<String>();
        Ok(format!("\n{}", result_string))
    }
}

fn parse_layers(input: &str, width: u8, height: u8) -> Result<Vec<Vec<u8>>, String> {
    let layer_size = width as usize * height as usize;
    let input = input.trim();
    if input.len() % layer_size != 0 {
        Err(format!(
            "Input data size {} not evenly divisible by layer size {}",
            input.len(),
            layer_size
        ))
    } else {
        input
            .chars()
            .map(|c| match c {
                '0' => Ok(0),
                '1' => Ok(1),
                '2' => Ok(2),
                '3' => Ok(3),
                '4' => Ok(4),
                '5' => Ok(5),
                '6' => Ok(6),
                '7' => Ok(7),
                '8' => Ok(8),
                '9' => Ok(9),
                _ => Err(format!("Character {} is not a digit", c)),
            })
            .collect::<Result<Vec<u8>, String>>()
            .map(|v| {
                v.chunks(layer_size)
                    .into_iter()
                    .map(|layer| layer.iter().cloned().collect())
                    .collect()
            })
    }
}

fn layer_with_fewest<'a, T: Eq>(layers: &'a [Vec<T>], digit: &T) -> Option<&'a [T]> {
    let mut smallest_layer = None;
    let mut smallest_count = usize::max_value();

    for (index, layer) in layers.iter().enumerate() {
        let count = count_digits(layer, digit);
        if count < smallest_count {
            smallest_count = count;
            smallest_layer = Some(index);
        }
    }

    smallest_layer.map(|i| layers[i].as_slice())
}

fn count_digits<T: Eq>(layer: &[T], digit: &T) -> usize {
    layer.iter().filter(|d| **d == *digit).count()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pixel {
    Black,
    White,
    Transparent,
}

fn layer_to_pixels(layer: &[u8]) -> Result<Vec<Pixel>, String> {
    layer
        .iter()
        .map(|b| match b {
            0 => Ok(Pixel::Black),
            1 => Ok(Pixel::White),
            2 => Ok(Pixel::Transparent),
            _ => Err(format!("Invalid image data {}", b)),
        })
        .collect()
}

fn stack_layers(upper: &[Pixel], lower: &[Pixel]) -> Vec<Pixel> {
    upper
        .iter()
        .zip(lower.iter())
        .map(|(u, l)| match u {
            Pixel::Transparent => *l,
            _ => *u,
        })
        .collect()
}

fn stack_all_layers(layers: &[Vec<Pixel>]) -> Vec<Pixel> {
    let transparent_layer = layers[0].iter().map(|_| Pixel::Transparent).collect();
    layers.iter().fold(transparent_layer, |upper, lower| {
        stack_layers(upper.as_slice(), lower.as_slice())
    })
}

#[test]
fn test_parse_layers() {
    let input = "123456789012";
    let width = 3;
    let height = 2;
    let layers = parse_layers(input, width, height).expect("Layers should parse");
    assert_eq!(layers[0], vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(layers[1], vec![7, 8, 9, 0, 1, 2]);
}

#[test]
fn test_layer_with_fewest() {
    let input = "123456789012";
    let width = 3;
    let height = 2;
    let layers = parse_layers(input, width, height).expect("Layers should parse");
    let fewest_zeroes = layer_with_fewest(layers.as_slice(), &0);

    assert_eq!(fewest_zeroes, Some(layers[0].as_slice()));

    let fewest_fives = layer_with_fewest(layers.as_slice(), &5);
    assert_eq!(fewest_fives, Some(layers[1].as_slice()));
}

#[test]
fn test_stacking() {
    let input = "0222112222120000";
    let width = 2;
    let height = 2;
    let layers = parse_layers(input, width, height)
        .expect("Layers should parse")
        .into_iter()
        .map(|v| layer_to_pixels(v.as_slice()))
        .collect::<Result<Vec<Vec<Pixel>>, String>>()
        .expect("All pixel data should be valid");
    let result = stack_all_layers(layers.as_slice());
    assert_eq!(
        result,
        vec![Pixel::Black, Pixel::White, Pixel::White, Pixel::Black]
    )
}
