#![allow(non_snake_case)]
use std::ops::Deref;
use std::rc::Rc;
use std::collections::HashSet;
// use std::string;

pub enum Regex {
    Empty(),
    Eps(),
    Letter(u8),
    Or(Rc<Regex>, Rc<Regex>),
    Concat(Rc<Regex>, Rc<Regex>),
    Star(Rc<Regex>)
}

use crate::Regex::{Empty, Eps, Letter, Or, Concat, Star};

fn findLambda (regexp: &Rc<Regex>) -> Rc<Regex> {
    match regexp.deref() {
        Empty() => Rc::new(Empty()),
        Eps()   =>  Rc::new(Eps()),
        Letter(_) => Rc::new(Empty()),
        Or(r1, r2) => {
            let s1 = findLambda(r1);
            let s2 = findLambda(r2);
            if matches!(*s1.deref(), Eps()) || matches!(*s2.deref(), Eps()) {
                Rc::new(Eps())
            } else {
                Rc::new(Empty())
            }
        },
        Concat(r1, r2) => {
            let s1 = findLambda(r1);
            let s2 = findLambda(r2);
            if matches!(*s1.deref(), Eps()) && matches!(*s2.deref(), Eps()) {
                Rc::new(Eps())
            } else {
                Rc::new(Empty())
            }
        },
        Star(_)    => Rc::new(Eps())
    }
}

fn constructP (regexp: &Rc<Regex>) -> HashSet<u8> {
    match regexp.deref() {
        Or(r1, r2)  => {
            let s1 = constructP(r1);
            let s2 = constructP(r2);
            let union_s: HashSet<_> = s1.union(&s2).collect();
            let mut hset: HashSet<u8> = HashSet::new();
            for x in union_s {
                hset.insert(*x);
                // println!("Hello11 : {}", *x);
            }
            // println!("End11");
            hset
        },
        Concat(r1, r2) => {
            let s = findLambda(r1);
            match s.deref() {
                Empty() => {
                    // println!("Go");
                    constructP(r1)
                    
                }
                Eps() => {
                    let s1 = constructP(r1);
                    let s2 = constructP(r2);
                    let union_s: HashSet<_> = s1.union(&s2).collect();
                    let mut hset: HashSet<u8> = HashSet::new();
                    for x in union_s {
                        hset.insert(*x);
                        // println!("Hello : {}", *x);
                    }
                    // println!("End");
                    hset
                }
                _   => {
                    let hset: HashSet<u8> = HashSet::new();
                    hset
                }
            }
        },
        Star(r1) => constructP(r1),
        Letter(x) => {
            let mut hset: HashSet<u8> = HashSet::new();
            hset.insert(*x);
            hset
        },
        _ => {
            let hset: HashSet<u8> = HashSet::new();
            hset
        }

    }
}

fn constructD (regexp: &Rc<Regex>) -> HashSet<u8> {
    match regexp.deref() {
        Or(r1, r2)  => {
            let s1 = constructD(r1);
            let s2 = constructD(r2);
            let union_s: HashSet<_> = s1.union(&s2).collect();
            let mut hset: HashSet<u8> = HashSet::new();
            for x in union_s {
                hset.insert(*x);
                // println!("Hello11 : {}", *x);
            }
            // println!("End11");
            hset
        },
        Concat(r1, r2) => {
            let s = findLambda(r2);
            match s.deref() {
                Empty() => {
                    // println!("Go");
                    constructD(r2)
                    
                }
                Eps() => {
                    let s1 = constructD(r1);
                    let s2 = constructD(r2);
                    let union_s: HashSet<_> = s1.union(&s2).collect();
                    let mut hset: HashSet<u8> = HashSet::new();
                    for x in union_s {
                        hset.insert(*x);
                        // println!("Hello : {}", *x);
                    }
                    // println!("End");
                    hset
                }
                _   => {
                    let hset: HashSet<u8> = HashSet::new();
                    hset
                }
            }
        }
        Star(r1) => constructD(r1),
        Letter(x) => {
            let mut hset: HashSet<u8> = HashSet::new();
            hset.insert(*x);
            hset
        },
        _ => {
            let hset: HashSet<u8> = HashSet::new();
            hset
        }

    }
}

fn constructF (regexp: &Rc<Regex>) -> HashSet<(u8, u8)> {
    match regexp.deref() {
        Or(r1, r2)  => {
            let s1 = constructF(r1);
            // for x in &s1 {
            //     println!("in s1 : {:?}", *x);
            // }
            // println!("End or s1");
            let s2 = constructF(r2);
            // for x in &s2 {
            //     println!("in s2 : {:?}", *x);
            // }
            // println!("End or s2");
            let union_s: HashSet<_> = s1.union(&s2).collect();
            let mut hset: HashSet<(u8, u8)> = HashSet::new();
            for x in union_s {
                hset.insert(*x);
                // println!("Finial union or: {:?}", *x);
            }
            // println!("End10");
            hset
        },
        Concat(r1, r2) => {
            let s1 = constructF(r1);
            // for x in &s1 {
            //     println!("in s1 : {:?}", *x);
            // }
            // println!("End concat s1");
            let s2 = constructF(r2);
            // for x in &s2 {
            //     println!("in s2 : {:?}", *x);
            // }
            // println!("End concat s2");
            let mut hset0: HashSet<(u8, u8)> = HashSet::new();
            let union_helper: HashSet<_> = s1.union(&s2).collect();
            for x in union_helper {
                hset0.insert(*x);
                // println!("in union helper concat : {:?}", *x);
            }
            // println!("End union helper concat");
            let hs1 = constructD(r1);
            let hs2 = constructP(r2);
            let mut hset1: HashSet<(u8, u8)> = HashSet::new();
            for x in hs1 {
                for y in &hs2 {
                    hset1.insert((x, *y));
                }
            }
            // for x in &hset1 {
            //     println!("in hset1 : {:?}", *x);
            // }
            // println!("End concat hset1");
            let union_s: HashSet<_> = hset0.union(&hset1).collect();
            let mut hset: HashSet<(u8, u8)> = HashSet::new();
            for x in union_s {
                hset.insert(*x);
                // println!("Hello12 : {:?}", *x);
            }
            // for x in &hset {
            //     // println!("in hset : {:?}", *x);
            // }
            // println!("Final concat hset");
            hset
        }
        Star(r1) => {
            let s1 = constructF(r1);
            // for x in &s1 {
            //     println!("in s1 : {:?}", *x);
            // }
            // println!("End star s1");
            let hs1 = constructD(r1);
            let hs2 = constructP(r1);
            let mut hset0: HashSet<(u8, u8)> = HashSet::new();
            for x in hs1 {
                for y in &hs2 {
                    hset0.insert((x, *y));
                }
            }
            let union_s: HashSet<_> = s1.union(&hset0).collect();
            let mut hset: HashSet<(u8, u8)> = HashSet::new();
            for x in union_s {
                hset.insert(*x);
                // println!("Hello12 : {:?}", *x);
            }
            hset
        },
        _ => {
            let hset: HashSet<(u8, u8)> = HashSet::new();
            hset
        }
    }
}

fn checkstate (curr: usize, final_states: &HashSet<u8>) -> bool {
    let mut res = false;
    for x in final_states {
        if usize::from(*x)/10 == curr {
            res = true;
            break;
        }
    }
    res
}

fn checkstr(s: &str, nfa: &Vec<Vec<u8>>, final_states: &HashSet<u8>, curr: usize, idx: usize, states: u8) -> bool {
    let mut res = false;
    let mut next_state = 0;
    
    while next_state != states.into() {
        if checkstate(curr, final_states) && idx == s.len() {
            res = true;
            break;
        }
        if !checkstate(curr, final_states) && idx == s.len() {
            res = false;
            break;
        }
        if nfa[curr][next_state] == (s.chars().nth(idx).unwrap() as u8 - '0' as u8) {
            println!("Char encounterd: {}, Going to State: {}", s.chars().nth(idx).unwrap(), next_state);
            res = res || checkstr(s, &nfa, &final_states, next_state, idx+1, states);
        }
        if res {
            break;
        }
        next_state += 1;
    }
    res
}

fn augment(regexp: &Rc<Regex>, cnt: &mut u8) -> Rc<Regex> {
    match regexp.deref() {
        Letter(a) => {
            *cnt = *cnt + 1;
            // println!("cnt {}", a);
            // println!("cnt {}", ((*cnt - 1)*10 + a));
            let ret = Rc::new(Letter((*cnt - 1)*10 + a));
            ret
        },
        Or(r1, r2) => Rc::new(Or(augment(r1, cnt), augment(r2, cnt))),
        Concat(r1, r2) => Rc::new(Concat(augment(r1, cnt), augment(r2, cnt))),
        Star(r1) => Rc::new(Star(augment(r1, cnt))),
        Empty() => Rc::new(Empty()),
        Eps() => Rc::new(Eps())
    }
}

fn main() {
    let x = Rc::new(Or(Rc::new(Concat(Rc::new(Letter(2)),Rc::new(Concat(Rc::new(Star(Rc::new(Letter(1)))), Rc::new(Letter(2)))))), Rc::new(Concat(Rc::new(Letter(2)), Rc::new(Star(Rc::new(Concat(Rc::new(Letter(2)), Rc::new(Letter(1))))))))));
    let mut cnt = 1;
    let a = augment(&x, &mut cnt); 
    // let a = Rc::new(Or(Rc::new(Concat(Rc::new(Letter(12)),Rc::new(Concat(Rc::new(Star(Rc::new(Letter(21)))), Rc::new(Letter(32)))))), Rc::new(Concat(Rc::new(Letter(42)), Rc::new(Star(Rc::new(Concat(Rc::new(Letter(52)), Rc::new(Letter(61))))))))));
    let P_set = constructP(&a);
    let D_set = constructD(&a);
    let F_set = constructF(&a);
    let states = 7;
    let width = 7;
    let height = 7;

    let mut array = vec![vec![0; width]; height];
    for x in &P_set {
        array[0][(*x/10) as usize] = *x%10;
    }
    for x in &F_set {
        array[(x.0/10) as usize][(x.1/10) as usize] = x.1%10;
    }
    println!("P: {:?}", P_set);
    println!("D: {:?}", D_set);
    println!("F: {:?}", F_set);
    println!("NFA Adjacency Matrix : {:?}", array);
    let s: &str = "221212121";

    if s.len() == 0 && matches!(findLambda(&a).deref(), Eps()) {
        println!("Accepted");
    }
    let res = checkstr(s, &array, &D_set, 0, 0, states);
    if res {
        println!("Accepted");
    } else {
        println!("Rejected");
    }
}