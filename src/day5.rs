use std::fs::read_to_string;

pub fn main() {
    println!("It's day 5 !!!");
    let almanac = Almanac::create_almanac().unwrap();

    part1(&almanac);
    part2(&almanac);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>
}

impl Almanac {
    fn create_almanac() -> Option<Almanac> {
        let file = read_to_string("src/day5_input.txt")
            .unwrap();
        let lines = file
            .lines()
            .collect::<Vec<&str>>();

        let seeds = lines[0].split_once(": ")?.1
            .split(" ")
            .filter_map(|seed_str| seed_str.parse().ok())
            .collect::<Vec<u64>>();

        let maps = Map::parse_maps(&lines[2..])?;

        Some(Almanac { seeds, maps })
    }

    fn compute_location(&self, seed: u64) -> u64 {
        let mut value = seed;
        for map in &self.maps {
            value = map.compute_value(value);
        }

        value
    }

    fn compute_range_location(&self, range: &SeedRange) -> Vec<SeedRange> {
        let mut ranges = vec![range.clone()];
        for map in &self.maps {
            ranges = ranges.iter()
                .flat_map(|range| map.compute_range(range))
                .collect();
        }

        ranges
    }
}

#[derive(Debug)]
struct Map {
   converters: Vec<Converter>
}

impl Map {
    fn parse_map(lines: &[&str]) -> Option<Map> {
        let converters = lines[1..].iter()
            .filter_map(Converter::parse_converter)
            .collect::<Vec<Converter>>();

        Some(Map {
            converters
        })
    }

    fn parse_maps(lines: &[&str]) -> Option<Vec<Map>> {
        let empty_lines: Vec<usize> = lines.iter()
            .enumerate()
            .filter(|(_, line)| line.is_empty())
            .map(|(idx, _)| idx)
            .collect();

        let mut maps: Vec<Map> = vec![];
        let mut prev_map_len = 0;
        let mut rest = lines;
        for i in 0..6 {
            let (map_list, cur_rest) = rest.split_at(empty_lines[i] - prev_map_len);
            prev_map_len += map_list.len() + 1;
            let map = Map::parse_map(map_list)?;
            maps.push(map);
            rest = &cur_rest[1..];
        }

        let map = Map::parse_map(rest)?;
        maps.push(map);

        Some(maps)
    }

    fn compute_value(&self, value: u64) -> u64 {
        match self.find_converter(value) {
            Some(converter) => converter.convert(value),
            None => value
        }
    }

    fn compute_range(&self, range: &SeedRange) -> Vec<SeedRange> {
        let mut converted_ranges: Vec<SeedRange> = vec![];
        let mut rest_list = vec![range.clone()];
        for converter in &self.converters {
            let mut new_rest_list: Vec<SeedRange> = vec![];
            for rest in rest_list {
                let (intersect, cur_rest_list) = &mut converter.intersect(&rest);

                if let Some(inter) = intersect {
                    converted_ranges.push(converter.convert_range(&inter));
                }

                if let Some(cur_rest_l) = cur_rest_list {
                    new_rest_list.append(cur_rest_l);
                }
            }

            if new_rest_list.is_empty() {
                return converted_ranges;
            }

            rest_list = new_rest_list;
        }

        converted_ranges.append(&mut rest_list);

        converted_ranges
    }

    fn find_converter(&self, value: u64) -> Option<Converter> {
        self.converters
            .iter()
            .find(|&converter| converter.accept(value))
            .map(Converter::clone)
    }
}

#[derive(Debug, Clone)]
struct Converter {
    dst: u64,
    src: u64,
    range: u64
}

impl Converter {
    fn parse_converter(&line: &&str) -> Option<Converter> {
        let values: Vec<u64> = line.split(" ")
            .filter_map(|num_str| num_str.parse().ok())
            .collect();

        Some(Converter { dst: values[0], src: values[1], range: values[2] })
    }

    fn accept(&self, value: u64) -> bool {
        self.src <= value && value < self.end()
    }

    fn convert(&self, value: u64) -> u64 {
        self.dst + (value - self.src)
    }

    fn end(&self) -> u64 {
        self.src + self.range
    }

    fn to_range(&self) -> SeedRange {
        SeedRange { start: self.src, range: self.range }
    }

    fn intersect(&self, range: &SeedRange) -> (Option<SeedRange>, Option<Vec<SeedRange>>) {
        if self.src <= range.start && range.end() <= self.end() {
            return (Some(range.clone()), None);
        } else if (range.start < self.src && range.end() < self.src) || (self.end() < range.start) {
            return (None, Some(vec![range.clone()]));
        } else if range.start < self.src && range.end() <= self.end() {
            return (
                Some(SeedRange { start: self.src, range: range.range - (self.src - range.start) }), 
                Some(vec![SeedRange { start: range.start, range: self.src - range.start }])
            );
        } else if self.src <= range.start && self.end() < range.end() {
            return (
                Some(SeedRange { start: range.start, range: self.end() - range.start }),
                Some(vec![SeedRange { start: self.end(), range: range.end() - self.end() }])
            );
        } else if range.start <= self.src && self.end() <= range.end() {
            return (
                Some(self.to_range()),
                Some(vec![
                    SeedRange { start: range.start, range: range.end() - self.src },
                    SeedRange { start: self.end(), range: range.end() - self.end() }
                ])
            );
        } else {
            panic!("Je ne connais pas ce cas :/ : self {:?}, range: {:?}", self, range);
        }
    }

    fn convert_range(&self, range: &SeedRange) -> SeedRange {
        SeedRange { start: self.dst + (range.start - self.src), range: range.range }
    }
}

fn part1(almanac: &Almanac) {
    let result: u64 = almanac.seeds
        .iter()
        .map(|&seed| almanac.compute_location(seed))
        .min()
        .unwrap();

    println!("Part 1: {}", result);
}

#[derive(Debug, Clone)]
struct SeedRange {
    start: u64,
    range: u64
}

impl SeedRange {
    fn end(&self) -> u64 {
        self.start + self.range
    }
}

fn part2(almanac: &Almanac) {
    let seeds = (0..almanac.seeds.len())
        .filter(|&val| val % 2 == 0)
        .zip((0..almanac.seeds.len()).filter(|&val| val % 2 == 1))
        .map(|(even, odd)| SeedRange { start: almanac.seeds[even], range: almanac.seeds[odd] })
        .collect::<Vec<SeedRange>>();

    let result: u64 = seeds
        .iter()
        .flat_map(|seed| almanac.compute_range_location(seed))
        .map(|range| range.start)
        .min()
        .unwrap();

    println!("Part 2: {}", result);
}