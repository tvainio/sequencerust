pub struct Sequencer {
    current_step: u8,
}

impl Sequencer {
    fn new() -> Sequencer {
        Sequencer { current_step: 0 }
    }
}

pub fn tick(s: Sequencer) -> Sequencer {
    Sequencer {
        current_step: s.current_step + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_ticks() {
        let s = Sequencer::new();
        let s2 = tick(s);
        assert_eq!(s2.current_step, 1);
    }
}
