use std::{error::Error, str::FromStr};

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
