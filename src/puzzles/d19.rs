use std::{error::Error, ops::Range, str::FromStr};

use rustc_hash::FxHashMap;

pub struct System {
    workflows: FxHashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl FromStr for System {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (workflows_str, parts_str) = s
            .split_once("\n\n")
            .ok_or::<String>("Invalid syntax".into())?;
        let workflows = workflows_str
            .lines()
            .map(|l| l.parse().map(|wf: Workflow| (wf.name.clone(), wf)))
            .collect::<Result<FxHashMap<String, Workflow>, _>>()?;
        let parts = parts_str
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Part>, _>>()?;

        Ok(System { workflows, parts })
    }
}

impl System {
    pub fn sum_accepted(&self) -> u32 {
        self.parts
            .iter()
            .filter(|&p| self.is_accepted(p))
            .map(|p| p.categories_total())
            .sum()
    }

    pub fn n_distinct_accepted(&self) -> u64 {
        let mut matched_ranges = FxHashMap::default();
        matched_ranges.insert("in".to_string(), vec![PartsRange::full_range()]);
        let mut frontier = vec!["in".to_string()];

        while let Some(wf_name) = frontier.pop() {
            let wf = &self.workflows[&wf_name];
            let Some(mut curr_ranges) = matched_ranges.remove(&wf_name) else {
                continue;
            };

            for rule in &wf.rules {
                let target = match &rule.action {
                    Action::SendTo(wf_name) => {
                        frontier.push(wf_name.to_string());
                        wf_name
                    }
                    Action::Accept => "A",
                    Action::Reject => "R",
                };
                let target_ranges = matched_ranges
                    .entry(target.to_string())
                    .or_insert(Vec::new());
                let mut next_ranges = Vec::new();

                for range in curr_ranges {
                    if let Some(ref cond) = rule.condition {
                        let (matched, mismatched) = range.split_by(cond);
                        target_ranges.push(matched);
                        next_ranges.push(mismatched);
                    } else {
                        target_ranges.push(range.clone());
                    }
                }

                curr_ranges = next_ranges;
            }
        }

        matched_ranges["A"].iter().map(|r| r.n_distinct()).sum()
    }

    fn is_accepted(&self, part: &Part) -> bool {
        let mut wf = &self.workflows["in"];
        loop {
            match wf.apply(part) {
                Action::SendTo(wf_name) => wf = &self.workflows[wf_name],
                Action::Accept => return true,
                Action::Reject => return false,
            }
        }
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get(&self, category: u8) -> Option<u32> {
        match category {
            b'x' => Some(self.x),
            b'm' => Some(self.m),
            b'a' => Some(self.a),
            b's' => Some(self.s),
            _ => None,
        }
    }

    fn categories_total(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xmas = s[1..(s.len() - 1)].split(',');
        let x = xmas
            .next()
            .ok_or::<String>("Not enough categories".into())?[2..]
            .parse()?;
        let m = xmas
            .next()
            .ok_or::<String>("Not enough categories".into())?[2..]
            .parse()?;
        let a = xmas
            .next()
            .ok_or::<String>("Not enough categories".into())?[2..]
            .parse()?;
        let s = xmas
            .next()
            .ok_or::<String>("Not enough categories".into())?[2..]
            .parse()?;

        Ok(Part { x, m, a, s })
    }
}

#[derive(Clone)]
struct PartsRange {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl PartsRange {
    fn n_distinct(&self) -> u64 {
        u64::try_from(self.x.len() * self.m.len() * self.a.len() * self.s.len()).unwrap()
    }

    fn full_range() -> Self {
        PartsRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    fn split_by(&self, condition: &Condition) -> (PartsRange, PartsRange) {
        let (mut matched, mut mismatched) = (self.clone(), self.clone());

        match condition.category {
            b'x' => {
                matched.x = Self::restrict_single_range(
                    &matched.x,
                    condition.operator,
                    condition.value,
                    true,
                );
                mismatched.x = Self::restrict_single_range(
                    &mismatched.x,
                    condition.operator,
                    condition.value,
                    false,
                );
            }
            b'm' => {
                matched.m = Self::restrict_single_range(
                    &matched.m,
                    condition.operator,
                    condition.value,
                    true,
                );
                mismatched.m = Self::restrict_single_range(
                    &mismatched.m,
                    condition.operator,
                    condition.value,
                    false,
                );
            }
            b'a' => {
                matched.a = Self::restrict_single_range(
                    &matched.a,
                    condition.operator,
                    condition.value,
                    true,
                );
                mismatched.a = Self::restrict_single_range(
                    &mismatched.a,
                    condition.operator,
                    condition.value,
                    false,
                );
            }
            b's' => {
                matched.s = Self::restrict_single_range(
                    &matched.s,
                    condition.operator,
                    condition.value,
                    true,
                );
                mismatched.s = Self::restrict_single_range(
                    &mismatched.s,
                    condition.operator,
                    condition.value,
                    false,
                );
            }
            _ => panic!("Invalid category"),
        }

        (matched, mismatched)
    }

    fn restrict_single_range(
        range: &Range<u32>,
        operator: u8,
        value: u32,
        matched: bool,
    ) -> Range<u32> {
        let (min, max);

        if matched {
            match operator {
                b'>' => {
                    min = u32::max(range.start, value + 1);
                    max = range.end;
                }
                b'<' => {
                    min = range.start;
                    max = u32::min(range.end, value);
                }
                _ => panic!("Invalid operator"),
            };
        } else {
            match operator {
                b'>' => {
                    min = range.start;
                    max = u32::min(range.end, value + 1);
                }
                b'<' => {
                    min = u32::max(range.start, value);
                    max = range.end;
                }
                _ => panic!("Invalid operator"),
            };
        }

        min..max
    }
}

struct Workflow {
    name: String,
    rules: Vec<WorkflowRule>,
}

impl FromStr for Workflow {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules_str) = s[..(s.len() - 1)]
            .split_once('{')
            .ok_or::<String>("Invalid syntax".into())?;
        let rules = rules_str
            .split(',')
            .map(|r| r.parse())
            .collect::<Result<Vec<WorkflowRule>, _>>()?;

        Ok(Workflow {
            name: name.to_string(),
            rules,
        })
    }
}

impl Workflow {
    fn apply(&self, part: &Part) -> &Action {
        for rule in &self.rules {
            if rule.matches(part) {
                return &rule.action;
            }
        }

        panic!("No rule matched");
    }
}

struct WorkflowRule {
    condition: Option<Condition>,
    action: Action,
}

impl FromStr for WorkflowRule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (condition, action_str);

        if let Some(pos) = s.find(':') {
            condition = Some(s[..pos].parse()?);
            action_str = &s[(pos + 1)..];
        } else {
            condition = None;
            action_str = &s;
        }
        let action = action_str.parse()?;

        Ok(WorkflowRule { condition, action })
    }
}

impl WorkflowRule {
    fn matches(&self, part: &Part) -> bool {
        let Some(condition) = &self.condition else {
            return true;
        };

        match condition.operator {
            b'>' => part.get(condition.category).unwrap() > condition.value,
            b'<' => part.get(condition.category).unwrap() < condition.value,
            _ => panic!("Invalid operator"),
        }
    }
}

struct Condition {
    category: u8,
    operator: u8,
    value: u32,
}

impl FromStr for Condition {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let category = s
            .as_bytes()
            .first()
            .copied()
            .ok_or::<String>("Condition string to short".into())?;
        let operator = s
            .as_bytes()
            .get(1)
            .copied()
            .ok_or::<String>("Condition string to short".into())?;
        let value = s[2..].parse()?;

        Ok(Condition {
            category,
            operator,
            value,
        })
    }
}

enum Action {
    SendTo(String),
    Accept,
    Reject,
}

impl FromStr for Action {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "R" {
            Ok(Action::Reject)
        } else if s == "A" {
            Ok(Action::Accept)
        } else {
            Ok(Action::SendTo(s.to_string()))
        }
    }
}
