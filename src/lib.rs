const PATTERN_LENGTH:usize = 16;

pub struct Sequencer<CB> where CB: FnMut() {
    current_step: usize,
    callback: CB,
    steps: Vec<bool>
}

impl<CB> Sequencer<CB> where CB: FnMut() {
    pub fn new(cb: CB)->Sequencer<CB> {
        Sequencer {current_step: 0, callback:cb, steps:vec![false; PATTERN_LENGTH]}
    }

    pub fn tick(&mut self) {
        if self.steps[self.current_step]==true {
            (self.callback)();
        }
        self.current_step = (self.current_step + 1) % PATTERN_LENGTH;
    }

    pub fn set(&mut self, step:usize, state:bool) {
        self.steps[step] = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_ticks() {
        let mut s = Sequencer::new(||());
        s.tick();
        assert_eq!(s.current_step, 1);
    }

    #[test]
    fn it_wraps_around() {
        let mut s = Sequencer::new(||());
        for _n in 0..18 {
            s.tick();
        }
        assert_eq!(s.current_step, 2);
    }

    #[test]
    fn it_doesnt_trigger_if_step_is_disabled() {
        let mut called = false;
        let cb = ||called=true;
        let mut s = Sequencer::new(cb);
        s.tick();
        assert_eq!(s.current_step, 1);
        assert_eq!(called, false);
    }
    #[test]
    fn it_triggers_only_if_active_step() {
        let mut called = false;
        let cb = ||called=true;
        let mut s = Sequencer::new(cb);
        s.set(0, true);
        s.tick();
        assert_eq!(s.current_step, 1);
        assert_eq!(called, true);
    }
}
