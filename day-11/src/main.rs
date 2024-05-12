use std::collections::VecDeque;

const ROUNDS: i32 = 10000;
const GCD: i64 = 19 * 2 * 13 * 5 * 7 * 11 * 17 * 3;

struct Monkey {
    items: VecDeque::<i64>,
    business: i64,
    test_div: i64,
    true_monkey: usize,
    false_monkey: usize,
    mut_func: fn(item: i64) -> i64
}

impl Monkey {

    fn take_turn(&mut self) -> (VecDeque<i64>, VecDeque<i64>) {
        let mut true_rval = VecDeque::<i64>::new();
        let mut false_rval = VecDeque::<i64>::new();

        while !self.items.is_empty() {
            let mut item = self.items.pop_front().unwrap();
            //println!("  Monkey inspects an item with a worry level of {}.", item);
            self.business += 1;

            item = (self.mut_func)(item);
            item = item % GCD;
            //println!("    Worry level is mutated to {}.", item);

            //item /= 3;
            //println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", item);

            if item % self.test_div == 0 {
                //println!("    Current worry level is divisible by {}.", self.test_div);
                true_rval.push_back(item);
                //println!("    Item with worry level {} is thrown to monkey {}.", item, self.true_monkey);
            } else {
                //println!("    Current worry level is not divisible by {}.", self.test_div);
                false_rval.push_back(item);
                //println!("    Item with worry level {} is thrown to monkey {}.", item, self.false_monkey);
            }
        }

        return (true_rval, false_rval);
    }
}

#[allow(dead_code)]
fn test0_mut(item: i64) -> i64 {
    item * 19
}

#[allow(dead_code)]
fn test1_mut(item: i64) -> i64 {
    item + 6
}

#[allow(dead_code)]
fn test2_mut(item: i64) -> i64 {
    item * item
}

#[allow(dead_code)]
fn test3_mut(item: i64) -> i64 {
    item + 3
}

#[allow(dead_code)]
fn test_monkeys() -> Vec::<Monkey> {
    let test_0 = Monkey {
      items: VecDeque::from([79, 98]),
      business: 0,
      mut_func: test0_mut,
      test_div: 23,
      true_monkey: 2,
      false_monkey: 3
    };

    let test_1 = Monkey {
      items: VecDeque::from([54, 65, 75, 74]),
      business: 0,
      mut_func: test1_mut, // Operation: new = old + 6
      test_div: 19,
      true_monkey: 2,
      false_monkey: 0
    };

    let test_2 = Monkey {
      items: VecDeque::from([79, 60, 97]),
      business: 0,
      mut_func: test2_mut, // Operation: new = old * old
      test_div: 13,
      true_monkey: 1,
      false_monkey: 3
    };

    let test_3 = Monkey {
      items: VecDeque::from([74]),
      business: 0,
      mut_func: test3_mut, // Operation: new = old + 3
      test_div: 17,
      true_monkey: 0,
      false_monkey: 1
    };

    vec![test_0, test_1, test_2, test_3]
}

fn input0_mut(item: i64) -> i64 {
    item * 13
}

fn input1_mut(item: i64) -> i64 {
    item + 3
}

fn input2_mut(item: i64) -> i64 {
    item + 6
}

fn input3_mut(item: i64) -> i64 {
    item + 2
}

fn input4_mut(item: i64) -> i64 {
    item * item
}

fn input5_mut(item: i64) -> i64 {
    item + 4
}

fn input6_mut(item: i64) -> i64 {
    item * 7
}

fn input7_mut(item: i64) -> i64 {
    item + 7
}

fn input_monkeys() -> Vec::<Monkey> {
    let monkey_0 = Monkey {
        items: VecDeque::from([71, 86]),
        business: 0,
        mut_func: input0_mut, // Operation: new = old * 13
        test_div: 19,
        true_monkey: 6,
        false_monkey: 7
    };

    let monkey_1 = Monkey {
        items: VecDeque::from([66, 50, 90, 53, 88, 85]),
        business: 0,
        mut_func: input1_mut, // Operation: new = old + 3
        test_div: 2,
        true_monkey: 5,
        false_monkey: 4
    };

    let monkey_2 = Monkey {
        items: VecDeque::from([97, 54, 89, 62, 84, 80, 63]),
        business: 0,
        mut_func: input2_mut, // Operation: new = old + 6
        test_div: 13,
        true_monkey: 4,
        false_monkey: 1
    };

    let monkey_3 = Monkey {
        items: VecDeque::from([82, 97, 56, 92]),
        business: 0,
        mut_func: input3_mut, // Operation: new = old + 2
        test_div: 5,
        true_monkey: 6,
        false_monkey: 0
    };

    let monkey_4 = Monkey {
        items: VecDeque::from([50, 99, 67, 61, 86]),
        business: 0,
        mut_func: input4_mut, // Operation: new = old * old
        test_div: 7,
        true_monkey: 5,
        false_monkey: 3
    };

    let monkey_5 = Monkey {
        items: VecDeque::from([61, 66, 72, 55, 64, 53, 72, 63]),
        business: 0,
        mut_func: input5_mut, // Operation: new = old + 4
        test_div: 11,
        true_monkey: 3,
        false_monkey: 0
    };

    let monkey_6 = Monkey {
        items: VecDeque::from([59, 79, 63]),
        business: 0,
        mut_func: input6_mut, // Operation: new = old * 7
        test_div: 17,
        true_monkey: 2,
        false_monkey: 7
    };

    let monkey_7 = Monkey {
        items: VecDeque::from([55]),
        business: 0,
        mut_func: input7_mut, // Operation: new = old + 7
        test_div: 3,
        true_monkey: 2,
        false_monkey: 1
    };

    return vec![monkey_0, monkey_1, monkey_2, monkey_3, monkey_4, monkey_5, monkey_6, monkey_7]
}

fn monkey_take_turn( monkeys: &mut Vec<Monkey>, i: usize) ->(VecDeque<i64>, VecDeque<i64>) {
    let cur: &mut Monkey = monkeys.get_mut(i).unwrap();

    cur.take_turn()
}

fn main() {
    //let mut monkeys = test_monkeys();
    let mut monkeys = input_monkeys();
    
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            //println!("Monkey {}:", i);
            let (mut t, mut f) = monkey_take_turn(&mut monkeys, i);

            // complete move of items
            let cur = monkeys.get(i).unwrap();
            let true_monkey = cur.true_monkey;
            let false_monkey = cur.false_monkey;
            if !t.is_empty() {
                let m = monkeys.get_mut(true_monkey).unwrap();
                m.items.append(&mut t);
            }
            if !f.is_empty() {
                let m = monkeys.get_mut(false_monkey).unwrap();
                m.items.append(&mut f);
            }
        }
    }

    // TODO: find 2 highest levels of monkey business and multiply them
    let mut max_1 = 0;
    let mut max_2 = 0;
    for (i, m) in monkeys.iter().enumerate() {
        println!("Monkey {}: {}", i, m.business);

        if m.business > max_1 {
            max_2 = max_1;
            max_1 = m.business;
        } else if m.business > max_2 {
            max_2 = m.business;
        }
    }

    println!("Total monkey business: {} * {} = {}", max_1, max_2, max_1 * max_2);
}
