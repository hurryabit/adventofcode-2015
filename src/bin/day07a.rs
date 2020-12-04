use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;

type Var = String;

enum Input {
    Var(Var),
    Const(u16),
}

enum Gate {
    Copy(Input),
    And(Input, Input),
    Or(Input, Input),
    Not(Input),
    LShift(Input, u8),
    RShift(Input, u8),
}

struct Circuit {
    gates: HashMap<Var, Gate>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse() {
            Ok(n) => Self::Const(n),
            Err(_) => Self::Var(s.to_string()),
        })
    }
}

impl FromStr for Gate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        let num_words = words.len();
        assert!(1 <= num_words && num_words <= 3);
        Ok(match num_words {
            1 => Self::Copy(words[0].parse()?),
            2 => {
                assert_eq!(words[0], "NOT");
                Self::Not(words[1].parse()?)
            }
            3 => match words[1] {
                "AND" => Self::And(words[0].parse()?, words[2].parse()?),
                "OR" => Self::Or(words[0].parse()?, words[2].parse()?),
                "LSHIFT" => Self::LShift(words[0].parse()?, words[2].parse()?),
                "RSHIFT" => Self::RShift(words[0].parse()?, words[2].parse()?),
                instr => panic!("Unknown instruction {}", instr),
            },
            _ => panic!("Cannot parse Expr: {}", s),
        })
    }
}

impl FromStr for Circuit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut gates = HashMap::new();
        for line in s.lines() {
            let arrow = line.find("->").unwrap();
            let output = line[arrow + 2..].trim().to_string();
            let gate = line[..arrow].trim().parse()?;
            gates.insert(output, gate);
        }
        Ok(Self { gates })
    }
}

type Store<'a> = HashMap<&'a Var, u16>;

impl Input {
    fn eval(&self, store: &Store) -> u16 {
        match self {
            Self::Var(x) => *store.get(x).unwrap(),
            Self::Const(n) => *n,
        }
    }

    fn for_each_var<'a>(&'a self, f: &mut dyn FnMut(&'a Var)) {
        match self {
            Input::Var(var) => f(var),
            Input::Const(_) => {}
        }
    }
}

impl Gate {
    fn eval(&self, store: &Store) -> u16 {
        match self {
            Self::Copy(input) => input.eval(store),
            Self::And(input1, input2) => input1.eval(store) & input2.eval(store),
            Self::Or(input1, input2) => input1.eval(store) | input2.eval(store),
            Self::Not(input) => !input.eval(store),
            Self::LShift(input, width) => input.eval(store) << width,
            Self::RShift(input, width) => input.eval(store) >> width,
        }
    }

    fn for_each_input_var<'a>(&'a self, f: &mut dyn FnMut(&'a Var)) {
        match self {
            Gate::Copy(input)
            | Gate::Not(input)
            | Gate::LShift(input, _)
            | Gate::RShift(input, _) => input.for_each_var(f),
            Gate::And(input1, input2) | Gate::Or(input1, input2) => {
                input1.for_each_var(f);
                input2.for_each_var(f);
            }
        }
    }
}

impl Circuit {
    #[allow(clippy::ptr_arg)]
    fn eval(&self, output: &Var) -> u16 {
        let mut stack = vec![(output, false)];
        let mut store = Store::new();
        while let Some((var, up)) = stack.pop() {
            let gate = self.gates.get(var).unwrap();
            if up {
                store.insert(var, gate.eval(&store));
            } else if !store.contains_key(var) {
                stack.push((var, true));
                gate.for_each_input_var(&mut |input| stack.push((input, false)));
            }
        }
        *store.get(output).unwrap()
    }
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("inputs/day07.txt")?;
    let circuit: Circuit = text.parse()?;
    let result = circuit.eval(&String::from("a"));
    println!("The result is {}", result);
    Ok(())
}
