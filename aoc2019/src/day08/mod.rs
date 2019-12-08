use crate::day::Day;

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
        Err("Not implemented".into())
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
