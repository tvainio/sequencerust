const PATTERN_LENGTH:u32 = 16;

pub struct Sequencer {
    current_step: u32,
}

impl Sequencer {
    fn new() -> Sequencer {
        Sequencer { current_step: 0 }
    }
}

pub fn tick(s: Sequencer) -> Sequencer {
    Sequencer {
        current_step: (s.current_step + 1) % PATTERN_LENGTH,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_ticks() {
        let s = Sequencer::new();
        let s = tick(s);
        assert_eq!(s.current_step, 1);
    }

    #[test]
    fn it_wraps_around() {
        let s = Sequencer::new();
        let a = 0..18;
        let s = a.fold(s, |s, _i| tick(s));
        assert_eq!(s.current_step, 2);
    }
}
