use crate::error::AoCError;
use crate::utils::lcm;
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

trait Module<'a> {
    fn tick(&mut self, pulse: bool, from: &'a str) -> Option<bool>;
    fn connect(&mut self, from: &'a str);
    fn reset(&mut self);
}

struct FlipFlop(bool);
impl<'a> Module<'a> for FlipFlop {
    fn tick(&mut self, pulse: bool, _: &'a str) -> Option<bool> {
        (!pulse).then(|| {
            self.0 = !self.0;
            self.0
        })
    }

    fn connect(&mut self, _: &str) {}

    fn reset(&mut self) {
        self.0 = false;
    }
}

struct Conjunction<'a>(BTreeMap<&'a str, bool>);
impl<'a> Module<'a> for Conjunction<'a> {
    fn tick(&mut self, pulse: bool, from: &'a str) -> Option<bool> {
        self.0.insert(from, pulse);
        Some(self.0.values().any(|v| !v))
    }

    fn connect(&mut self, from: &'a str) {
        self.0.insert(from, false);
    }

    fn reset(&mut self) {
        self.0.values_mut().for_each(|v| *v = false);
    }
}

struct Broadcaster;
impl<'a> Module<'a> for Broadcaster {
    fn tick(&mut self, pulse: bool, _: &'a str) -> Option<bool> {
        Some(pulse)
    }

    fn connect(&mut self, _: &str) {}

    fn reset(&mut self) {}
}

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let (mut modules, wires) = parse_input(&input)?;

    let mut counts = [0, 0];
    for _ in 0..1000 {
        push_button(&mut modules, &wires, |_, pulse| {
            counts[pulse as usize] += 1;
        });
    }

    Ok((counts[0] * counts[1]).to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let (mut modules, wires) = parse_input(&input)?;
    let prev = wires
        .iter()
        .find_map(|(&name, links)| links.contains(&"rx").then_some(name))
        .ok_or(AoCError::from("not solvable"))?;
    let branches = wires
        .iter()
        .filter_map(|(&name, links)| links.contains(&prev).then_some(name))
        .collect::<Vec<_>>();
    let values = branches
        .iter()
        .map(|&name| {
            modules.values_mut().for_each(|m| m.reset());
            let mut answer = 0u64;
            loop {
                answer += 1;
                let mut found = false;
                push_button(&mut modules, &wires, |from, pulse| {
                    if pulse && from == name {
                        found = true;
                    }
                });
                if found {
                    break;
                }
            }
            answer
        })
        .collect::<Vec<_>>();
    Ok(values.into_iter().fold(1, lcm).to_string())
}

fn parse_input(
    input: &str,
) -> Result<
    (
        BTreeMap<&str, Box<dyn Module + '_>>,
        BTreeMap<&str, Vec<&str>>,
    ),
    AoCError,
> {
    let mut modules = BTreeMap::<&str, Box<dyn Module>>::new();
    let mut wires = BTreeMap::<&str, Vec<&str>>::new();

    input
        .trim()
        .lines()
        .map(|line| {
            let (name, links) = line
                .split_once(" -> ")
                .ok_or(AoCError::from("invalid format"))?;
            let links = links.split(", ").collect::<Vec<_>>();
            let (name, module) = match name {
                "broadcaster" => ("broadcaster", Box::new(Broadcaster) as Box<dyn Module>),
                ff if ff.starts_with('%') => {
                    (&ff[1..], Box::new(FlipFlop(false)) as Box<dyn Module>)
                }
                c if c.starts_with('&') => (
                    &c[1..],
                    Box::new(Conjunction(BTreeMap::new())) as Box<dyn Module>,
                ),
                _ => return Err(AoCError::from("invalid module")),
            };
            modules.insert(name, module);
            wires.insert(name, links);
            Ok(())
        })
        .collect::<Result<Vec<_>, _>>()?;

    wires.iter().for_each(|(from, to)| {
        to.iter().for_each(|to| {
            if let Some(module) = modules.get_mut(to) {
                module.connect(from);
            }
        });
    });

    Ok((modules, wires))
}

fn push_button<'a, F: FnMut(&str, bool)>(
    modules: &mut BTreeMap<&'a str, Box<(dyn Module<'a> + 'a)>>,
    wires: &BTreeMap<&'a str, Vec<&'a str>>,
    mut callback: F,
) {
    let mut queue = VecDeque::from([("button", "broadcaster", false)]);
    while let Some((from, to, pulse)) = queue.pop_front() {
        callback(from, pulse);

        if let Some(module) = modules.get_mut(to) {
            if let Some(pulse) = module.tick(pulse, from) {
                if let Some(links) = wires.get(to) {
                    links.iter().for_each(|link| {
                        queue.push_back((to, link, pulse));
                    });
                }
            }
        }
    }
}
