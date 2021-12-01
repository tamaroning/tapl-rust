use std::rc::Rc;

// Term (Def )
#[derive(Debug, PartialEq, Eq, Clone)]
enum Term {
    // variables
    Var(i32),
    // lambda abstractions
    Abs(Rc<Term>),
    // application
    App(Rc<Term>, Rc<Term>),
}

fn shift1(d: i32, c: i32, tm: Rc<Term>) -> Rc<Term> {
    match &*tm {
        Term::Var(x) => {
            if *x >= c {
                Rc::new(Term::Var(*x + d))
            } else {
                tm.clone()
            }
        }
        Term::Abs(t) => Rc::new(Term::Abs(shift1(d, c + 1, t.clone()))),
        Term::App(t1, t2) => Rc::new(Term::App(
            shift1(d, c, t1.clone()),
            shift1(d, c, t2.clone()),
        )),
    }
}

fn term_shift(d: i32, tm: Rc<Term>) -> Rc<Term> {
    shift1(d, 0, tm)
}

fn term_subst_walk(j: i32, s: Rc<Term>, c: i32, t: Rc<Term>) -> Rc<Term> {
    todo!()
}

fn term_subst(j: i32, s: Rc<Term>, tm: Rc<Term>) -> Rc<Term> {
    term_subst_walk(j, s, 0, tm)
}

// (λ. 2 0) 4 の場合, 以下の手順で代入を行う
// 関数呼び出し前に t = 2 0 と v = 4 を取り出しておく
// 1. vのインデックスを+1して5にする
// 2. tの0の項をv=5で置き換える => t = 2 5
// 3. t全体を-1する => t = 1 4
fn term_subst_top(s: Rc<Term>, tm: Rc<Term>) -> Rc<Term> {
    term_shift(-1, term_subst(0, term_shift(1, s), tm))
}

impl Term {
    fn is_val(&self) -> bool {
        matches!(self, Term::Abs(..))
    }

    fn no_rule_applies(&self) -> ! {
        panic!("No rule applies");
    }

    fn eval1(&self) -> Rc<Term> {
        match self {
            Term::App(l, r) => {
                // E-AppAbs
                if let Term::Abs(t) = &**l {
                    if r.is_val() {
                        println!("E-AppAbs");
                        term_subst_top(r.clone(), t.clone());
                    }
                }
                // E-App1
                if !l.is_val() {
                    println!("E-App1");
                    return Rc::new(Term::App(l.eval1(), r.clone()));
                }
                // E-App2
                if l.is_val() && !r.is_val() {
                    println!("E-App2");
                    return Rc::new(Term::App(l.clone(), self.eval1()));
                }
                self.no_rule_applies();
            }
            _ => self.no_rule_applies(),
        }
    }
}

#[test]
fn test() {
    let term = Term::Var(0);
    println!("{:?}", term);
}

#[test]
fn test_shift() {
    let tm = Term::App(
        Rc::new(Term::Abs(Rc::new(Term::Var(3)))),
        Rc::new(Term::Abs(Rc::new(Term::Var(2)))),
    );

    println!("{:?}", tm);
}

#[test]
fn test_eval() {
    let tm = Term::App(
        Rc::new(Term::Abs(Rc::new(Term::Var(0)))),
        Rc::new(Term::Abs(Rc::new(Term::Var(0)))),
    );
    let tm_eval1 = Term::Abs(Rc::new(Term::Var(0)));

    // (λ. 0) (λ. 0) -> (λ. 0)
    assert_eq!(*tm.eval1(), tm_eval1);
}

fn main() {
    println!("Hello, tapl-rust!");
}
