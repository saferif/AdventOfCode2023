use crate::error::AoCError;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ops::RangeInclusive;

struct Condition<'a> {
    key: &'a str,
    value: i64,
    valid: Option<i64>,
}

impl<'a> Condition<'a> {
    fn check(&self, part: &BTreeMap<&str, i64>) -> bool {
        part.get(self.key).iter().all(|&&v| self.check_point(v))
    }
    fn check_point(&self, p: i64) -> bool {
        self.valid
            .iter()
            .all(|&valid| (p - self.value).signum() == valid)
    }
}

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let (workflows, parts) = input
        .trim()
        .split_once("\n\n")
        .ok_or(AoCError::from("invalid format"))?;
    let workflows = parse_workflows(workflows)?;
    let parts = parse_parts(parts)?;

    let s = parts
        .into_iter()
        .map(|part| {
            let mut workflow_name = "in";
            while workflow_name != "R" && workflow_name != "A" {
                let workflow = workflows
                    .get(workflow_name)
                    .ok_or(AoCError::from("unknown workflow"))?;
                workflow_name = workflow
                    .iter()
                    .find_map(|(condition, next_workflow)| {
                        condition.check(&part).then_some(next_workflow)
                    })
                    .ok_or(AoCError::from("unknown transition"))?;
            }
            if workflow_name == "A" {
                Ok(part.values().sum::<i64>())
            } else {
                Ok(0)
            }
        })
        .collect::<Result<Vec<_>, AoCError>>()?;
    let s = s.into_iter().sum::<i64>();
    Ok(s.to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let (workflows, _) = input
        .trim()
        .split_once("\n\n")
        .ok_or(AoCError::from("invalid format"))?;
    let workflows = parse_workflows(workflows)?;
    Ok(dfs(&workflows)?.to_string())
}

fn parse_workflows(s: &str) -> Result<BTreeMap<&str, Vec<(Condition, &str)>>, AoCError> {
    s.lines()
        .map(|line| {
            let (name, rules) = line
                .split_once('{')
                .ok_or(AoCError::from("invalid format"))?;
            let rules = rules.trim_end_matches('}');
            Ok((
                name,
                rules
                    .split(',')
                    .map(|rule| {
                        if let Some((cond, next_name)) = rule.split_once(':') {
                            if let Some((key, value)) = cond.split_once('<') {
                                let value = value.parse::<i64>()?;
                                Ok((
                                    Condition {
                                        key,
                                        value,
                                        valid: Some(-1),
                                    },
                                    next_name,
                                ))
                            } else if let Some((key, value)) = cond.split_once('>') {
                                let value = value.parse::<i64>()?;
                                Ok((
                                    Condition {
                                        key,
                                        value,
                                        valid: Some(1),
                                    },
                                    next_name,
                                ))
                            } else {
                                Err(AoCError::from("invalid condition"))
                            }
                        } else {
                            Ok((
                                Condition {
                                    key: "",
                                    value: 0,
                                    valid: None,
                                },
                                rule,
                            ))
                        }
                    })
                    .collect::<Result<Vec<_>, AoCError>>()?,
            ))
        })
        .collect()
}

fn parse_parts(s: &str) -> Result<Vec<BTreeMap<&str, i64>>, AoCError> {
    s.lines()
        .map(|line| {
            Ok(line
                .trim_matches(['{', '}'].as_slice())
                .split(',')
                .map(|cat| {
                    let (name, value) = cat
                        .split_once('=')
                        .ok_or(AoCError::from("invalid format"))?;
                    let value = value.parse::<i64>()?;
                    Ok((name, value))
                })
                .collect::<Result<BTreeMap<_, _>, AoCError>>()?)
        })
        .collect()
}

fn dfs(workflows: &BTreeMap<&str, Vec<(Condition, &str)>>) -> Result<u64, AoCError> {
    let mut answer = 0u64;
    let state = (
        "in",
        BTreeMap::from([
            ("x", RangeInclusive::new(1, 4000)),
            ("m", RangeInclusive::new(1, 4000)),
            ("a", RangeInclusive::new(1, 4000)),
            ("s", RangeInclusive::new(1, 4000)),
        ]),
    );
    let mut stack = Vec::from([state]);
    while let Some((workflow, mut ranges)) = stack.pop() {
        if workflow == "R" {
            continue;
        }
        if workflow == "A" {
            answer += ranges
                .values()
                .map(|r| r.end() - r.start() + 1)
                .product::<i64>() as u64;
            continue;
        }
        let workflow = workflows
            .get(workflow)
            .ok_or(AoCError::from("unknown workflow"))?;
        for (condition, &ref next_workflow) in workflow {
            if let Some(valid) = condition.valid {
                let range = ranges
                    .get(condition.key)
                    .ok_or(AoCError::from("invalid attribute"))?;
                if range.contains(&condition.value) {
                    let left;
                    let right;
                    if valid < 0 {
                        left = RangeInclusive::new(*range.start(), condition.value - 1);
                        right = RangeInclusive::new(condition.value, *range.end());
                    } else {
                        left = RangeInclusive::new(*range.start(), condition.value);
                        right = RangeInclusive::new(condition.value + 1, *range.end());
                    }
                    for range in [left, right] {
                        if range.is_empty() {
                            continue;
                        }
                        let new_ranges = ranges
                            .iter()
                            .map(|(&n, r)| {
                                if n == condition.key {
                                    (n, range.clone())
                                } else {
                                    (n, r.clone())
                                }
                            })
                            .collect::<BTreeMap<_, _>>();
                        if condition.check_point(*range.start()) {
                            stack.push((next_workflow, new_ranges));
                        } else {
                            ranges = new_ranges;
                        }
                    }
                } else {
                    if condition.check_point(*range.start()) {
                        stack.push((next_workflow, ranges.clone()));
                        break;
                    }
                }
            } else {
                stack.push((next_workflow, ranges.clone()));
            }
        }
    }
    Ok(answer)
}
