use std::{collections::VecDeque, error::Error, str::FromStr};

use num_integer::lcm;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub struct ModuleNetwork {
    modules: Vec<Module>,
    input_idx: usize,
    indexes: HashMap<String, usize>,
    // outputs[0] == vec![(1, 1), (2, 6), (3, 0)] indicates that module 0
    // outputs to input 1 of module 1, input 6 of module 2 and input 0 of module
    // 3.
    outputs: HashMap<usize, Vec<(usize, usize)>>,
    queue: VecDeque<Pulse>, // pulses that have been sent but not received and processed
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

        // Build the modules now that we know their types and connections, and
        // build the connections in terms of indexes.
        let indexes: HashMap<String, usize> =
            names.into_iter().enumerate().map(|(i, s)| (s, i)).collect();
        let mut modules = Vec::new();
        let mut outputs = HashMap::default();
        let input_idx = indexes["broadcaster"];

        for (name, idx) in &indexes {
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
            indexes,
            outputs,
            queue: VecDeque::new(),
        })
    }
}

impl ModuleNetwork {
    pub fn press_multiple_and_count_pulses(&mut self, n_presses: usize) -> (u32, u32) {
        let (mut n_low_total, mut n_high_total) = (0, 0);
        for _ in 0..n_presses {
            let (n_low, n_high) = self.count_pulses_after_press();
            n_low_total += n_low;
            n_high_total += n_high;
        }

        (n_low_total, n_high_total)
    }

    pub fn steps_until_rx_first_low(&mut self) -> u64 {
        // Through looking a the graph in graphviz, it can be determined that rx
        // first goes low when the conjunctions bl, mr, pv and vv first go low
        // together, which is when ks, kb, sx and jt first go high together.
        // Because the subgraphs corresponding to bl, mr, pv and vv aren't
        // sending pulses to each other and because these subgraphs are behaving
        // periodically, we can take the lcm of the steps required for ks, kb,
        // sx and jt seperately.

        // This could be optimized by determining ks, kb, sx and jt in one go
        // without resetting in between.
        let steps_ks = self.press_until_first_low_received("ks");
        self.reset();

        let steps_kb = self.press_until_first_low_received("kb");
        self.reset();

        let steps_sx = self.press_until_first_low_received("sx");
        self.reset();

        let steps_jt = self.press_until_first_low_received("jt");
        self.reset();

        [steps_ks, steps_kb, steps_sx, steps_jt]
            .into_iter()
            .reduce(lcm)
            .unwrap()
    }

    /// Process 1 pulse from the queue and return it. Panics if the queue is empty.
    fn step(&mut self) -> Pulse {
        let pulse = self.queue.pop_back().unwrap();
        let module = &mut self.modules[pulse.idx_rx_module];
        let Some(out_high) = module.process_pulse(pulse.idx_rx_input, pulse.high) else {
            return pulse;
        };

        for (idx_target_module, idx_target_input) in &self.outputs[&pulse.idx_rx_module] {
            let tx_pulse = Pulse {
                idx_rx_module: *idx_target_module,
                idx_rx_input: *idx_target_input,
                high: out_high,
            };
            self.queue.push_front(tx_pulse);
        }

        pulse
    }

    fn press_button(&mut self) {
        let first_pulse = Pulse {
            idx_rx_module: self.input_idx,
            idx_rx_input: 0,
            high: false,
        };
        self.queue.push_front(first_pulse);
    }

    /// Returns (num low pulses, num high pulses)
    fn count_pulses_after_press(&mut self) -> (u32, u32) {
        let (mut n_low, mut n_high) = (0, 0);
        self.press_button();

        while !self.queue.is_empty() {
            let pulse = self.step();

            if pulse.high {
                n_high += 1;
            } else {
                n_low += 1;
            }
        }
        (
            u32::try_from(n_low).unwrap(),
            u32::try_from(n_high).unwrap(),
        )
    }

    fn press_until_first_low_received(&mut self, module_name: &str) -> u64 {
        let mut n_presses: u64 = 0;
        let module_idx = self.indexes[module_name];

        loop {
            while !self.queue.is_empty() {
                let pulse = self.step();
                if pulse.idx_rx_module == module_idx && !pulse.high {
                    return n_presses;
                }
            }

            self.press_button();
            n_presses += 1;
        }
    }

    fn reset(&mut self) {
        for module in self.modules.iter_mut() {
            match module {
                Module::BroadCast => (),
                Module::FlipFlop(state) => *state = false,
                Module::Conjuction(mem) => {
                    mem.iter_mut().map(|m| *m = false).count();
                }
                Module::UnTyped => (),
            }
        }
        self.queue.clear();
    }
}

struct Pulse {
    idx_rx_module: usize,
    idx_rx_input: usize,
    high: bool,
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
