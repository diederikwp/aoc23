use std::{collections::VecDeque, error::Error, str::FromStr};

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub struct ModuleNetwork {
    modules: Vec<Module>,
    input_idx: usize,
    // outputs[0] == vec![(1, 1), (2, 6), (3, 0)] indicates that module 0
    // outputs to input 1 of module 1, input 6 of module 2 and input 0 of module
    // 3.
    outputs: HashMap<usize, Vec<(usize, usize)>>,
}

impl FromStr for ModuleNetwork {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Gather module names, types and connections in terms of names
        let mut names = HashSet::default();
        let mut from_to = HashMap::default();
        let mut to_from = HashMap::default();
        let mut kinds = HashMap::default();

        for line in s.lines() {
            let (in_str, out_str) = line
                .split_once(" -> ")
                .ok_or::<String>("Invalid syntax".into())?;

            let in_name;
            if in_str == "broadcaster" {
                in_name = in_str.to_string();
                kinds.insert(in_name.clone(), b'b');
            } else if let Some(stripped) = in_str.strip_prefix('%') {
                in_name = stripped.to_string();
                kinds.insert(in_name.clone(), b'%');
            } else if let Some(stripped) = in_str.strip_prefix('&') {
                in_name = stripped.to_string();
                kinds.insert(in_name.clone(), b'&');
            } else {
                return Err("Invalid module".into());
            }
            names.insert(in_name.clone());

            let out_names: Vec<String> = out_str.split(", ").map(|s| s.to_string()).collect();
            names.extend(out_names.iter().cloned());
            from_to.insert(in_name.clone(), out_names.clone());

            for on in out_names {
                to_from
                    .entry(on)
                    .or_insert(Vec::new())
                    .push(in_name.clone());
            }
        }
        let indexes: HashMap<&str, usize> = names
            .iter()
            .enumerate()
            .map(|(i, s)| (s.as_str(), i))
            .collect();

        // Build the modules now that we know their types and connections, and
        // build the connections in terms of indexes.
        let mut modules = Vec::new();
        let mut outputs = HashMap::default();
        let input_idx = indexes["broadcaster"];

        for (&name, idx) in &indexes {
            let n_inputs = match to_from.get(name) {
                Some(from) => from.len(),
                None => 0,
            };

            let module = match kinds.get(name) {
                Some(b'b') => Module::BroadCast,
                Some(b'%') => Module::FlipFlop(false),
                Some(b'&') => Module::Conjuction(vec![false; n_inputs]),
                None => Module::UnTyped,
                Some(_) => panic!("Invalid module kind"),
            };
            modules.push(module);

            let Some(output_names) = from_to.get(name) else {
                continue;
            };

            let output_idxs = output_names
                .iter()
                .map(|to| {
                    let idx_module = indexes[to.as_str()];
                    let idx_input = to_from[to].iter().position(|s| s == name).unwrap();

                    (idx_module, idx_input)
                })
                .collect();
            outputs.insert(*idx, output_idxs);
        }

        Ok(ModuleNetwork {
            modules,
            input_idx,
            outputs,
        })
    }
}

impl ModuleNetwork {
    pub fn press_multiple_and_count_pulses(&mut self, n_presses: usize) -> (u32, u32) {
        let (mut n_low_total, mut n_high_total) = (0, 0);
        for _ in 0..n_presses {
            let (n_low, n_high) = self.press_once_and_count_pulses();
            n_low_total += n_low;
            n_high_total += n_high;
        }

        (n_low_total, n_high_total)
    }

    fn press_once_and_count_pulses(&mut self) -> (u32, u32) {
        let (mut n_low, mut n_high) = (1, 0);

        let mut queue = VecDeque::default();
        queue.push_front((self.input_idx, 0, false));

        while let Some((idx_module, idx_input, pulse_high)) = queue.pop_back() {
            let module = &mut self.modules[idx_module];
            let Some(out_high) = module.process_pulse(idx_input, pulse_high) else {
                continue;
            };

            let n_out = &self.outputs[&idx_module].len();
            if out_high {
                n_high += n_out;
            } else {
                n_low += n_out;
            }

            for (idx_target_module, idx_target_input) in &self.outputs[&idx_module] {
                queue.push_front((*idx_target_module, *idx_target_input, out_high))
            }
        }

        (
            u32::try_from(n_low).unwrap(),
            u32::try_from(n_high).unwrap(),
        )
    }
}

enum Module {
    BroadCast,
    FlipFlop(bool),
    Conjuction(Vec<bool>),
    UnTyped,
}

impl Module {
    fn process_pulse(&mut self, idx_input: usize, input_high: bool) -> Option<bool> {
        match self {
            Module::BroadCast => Some(input_high),
            Module::FlipFlop(state) => {
                if input_high {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            Module::Conjuction(memory) => {
                memory[idx_input] = input_high;
                Some(!memory.iter().all(|&m| m))
            }
            Module::UnTyped => None,
        }
    }
}
