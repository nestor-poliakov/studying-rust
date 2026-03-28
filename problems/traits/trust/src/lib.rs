#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    left: Box<dyn Agent>,
    right: Box<dyn Agent>,
    result: Vec<RoundOutcome>,
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Self {
            left,
            right,
            result: Vec::new(),
        }
    }

    pub fn left_score(&self) -> i32 {
        self.score().0
    }

    pub fn right_score(&self) -> i32 {
        self.score().1
    }

    fn score(&self) -> (i32, i32) {
        let mut score = (0, 0);
        for o in &self.result {
            let round_score = match o {
                RoundOutcome::BothCooperated => (2, 2),
                RoundOutcome::BothCheated => (0, 0),
                RoundOutcome::LeftCheated => (3, -1),
                RoundOutcome::RightCheated => (-1, 3),
            };
            score.0 += round_score.0;
            score.1 += round_score.1;
        }
        score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let la = self.left.next();
        let ra = self.right.next();
        let outcome = match (la, ra) {
            (Action::Cooperate, Action::Cooperate) => RoundOutcome::BothCooperated,
            (Action::Cooperate, Action::Cheat) => RoundOutcome::RightCheated,
            (Action::Cheat, Action::Cooperate) => RoundOutcome::LeftCheated,
            (Action::Cheat, Action::Cheat) => RoundOutcome::BothCheated,
        };
        self.result.push(outcome);
        self.left.result(ra);
        self.right.result(la);
        outcome
    }
}

pub trait Agent {
    fn next(&self) -> Action;
    fn result(&mut self, a: Action);
}

#[derive(PartialEq, Clone, Copy)]
pub enum Action {
    Cheat,
    Cooperate,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

impl Agent for CheatingAgent {
    fn next(&self) -> Action {
        Action::Cheat
    }
    fn result(&mut self, _: Action) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Agent for CooperatingAgent {
    fn next(&self) -> Action {
        Action::Cooperate
    }
    fn result(&mut self, _: Action) {}
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct GrudgerAgent {
    rival_has_cheated: bool,
}

impl Agent for GrudgerAgent {
    fn next(&self) -> Action {
        match self.rival_has_cheated {
            true => Action::Cheat,
            false => Action::Cooperate,
        }
    }
    fn result(&mut self, a: Action) {
        if a == Action::Cheat {
            self.rival_has_cheated = true;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct CopycatAgent {
    rival_last_action: Option<Action>,
}

impl Agent for CopycatAgent {
    fn next(&self) -> Action {
        match self.rival_last_action {
            None => Action::Cooperate,
            Some(a) => a,
        }
    }
    fn result(&mut self, a: Action) {
        self.rival_last_action = Some(a);
    }
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct DetectiveAgent {
    turn_num: usize,
    copycat: Option<CopycatAgent>,
}

impl Agent for DetectiveAgent {
    fn next(&self) -> Action {
        match self.turn_num {
            0 => Action::Cooperate,
            1 => Action::Cheat,
            2 => Action::Cooperate,
            3 => Action::Cooperate,
            _ if self.copycat.is_none() => Action::Cheat,
            _ => self.copycat.as_ref().unwrap().next(),
        }
    }
    fn result(&mut self, a: Action) {
        match (self.turn_num, a, &mut self.copycat) {
            (0..4, Action::Cheat, None) => {
                let mut cc = CopycatAgent::default();
                cc.result(Action::Cheat);
                self.copycat = Some(cc);
            }
            (0..4, Action::Cheat, Some(_)) => {
                self.copycat.as_mut().unwrap().result(Action::Cheat);
            }
            (0..4, Action::Cooperate, None) => {}
            (0..4, Action::Cooperate, Some(_)) => {
                self.copycat.as_mut().unwrap().result(Action::Cooperate);
            }
            (_, _, Some(c)) => c.result(a),
            _ => {}
        }
        self.turn_num += 1;
    }
}
