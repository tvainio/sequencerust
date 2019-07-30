/*

pub struct Sequencer<F> where F:Fn() {
    current_step: u32,
    callback: F
}
 
impl Sequencer<Fn()> {
    fn new<F>(f: F) -> Sequencer<F> {
        Sequencer { current_step: 0, callback: f }
    }
}

pub fn tick(s: Sequencer<) -> Sequencer<F> {
    Sequencer {
        current_step: (s.current_step + 1) % PATTERN_LENGTH,
    }
}

 */
const PATTERN_LENGTH:u32 = 16;

pub struct Sequencer<CB> where CB: FnMut() {
    current_step: u32,
    callback: CB,
}

impl<CB> Sequencer<CB> where CB: FnMut() {
    pub fn new(cb: CB)->Sequencer<CB> {
        Sequencer {current_step: 0, callback:cb}
    }

    pub fn tick(&mut self) {
        self.current_step = (self.current_step + 1) % PATTERN_LENGTH;
        (self.callback)();
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
    fn it_triggers() {
        let mut called = false;
        let cb = ||called=true;
        let mut s = Sequencer {current_step: 0, callback:cb};
        s.tick();
        assert_eq!(s.current_step, 1);
        assert_eq!(called, true);
    }
}
