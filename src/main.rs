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

impl Term {
    fn is_val(&self) -> bool {
        matches!(self, Term::Abs(..))
    }

    fn no_rule_applies(&self) -> ! {
        panic!("No rule applies");
    }

    // Shift (Def 6.2.1)
    // ↑[c,d](k) = { k (k<c), k+d (k>=c)
    // ↑[c,d](λ.t) = λ.↑[c+1,d](t)
    // ↑[c,d](t1,t2) = ↑[c,d](t1) ↑[c,d](t2)
    fn term_shift(&self, c: i32) -> Rc<Term> {
        todo!()
    }

    // (λ. t12) v2 => t12.term_substr_top(v2)
    fn term_subst_top(&self, v: Rc<Term>) -> Rc<Term> {
        todo!()
    }

    fn eval1(&self) -> Rc<Term> {
        match self {
            Term::App(l, r) => {
                // E-AppAbs
                // (λx.t12) v2 → [x → v2] t12
                //
                // Assignment
                // [j → s] k = { s (k=j), k (otherwise)
                // [j → s] (λ.t) = λ.[j+1 → ↑[0,1](s)] t
                // [j → s] (t1 t2) = ([j → s] t1) ([j → s] t2)
                if let Term::Abs(t) = &**l {
                    if r.is_val() {
                        println!("E-AppAbs");
                        todo!();
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
    ); //.term_shift(2, -1);

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
