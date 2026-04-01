pub struct IndexController {
    pub current: usize,
    pub len: usize,
    pub ignore: Vec<usize>,
}
pub enum IndexAction {
    Increase,
    Decrease,
}

impl IndexController {
    pub fn setLen(&mut self, len: usize) {
        self.len = len;
        self.current = 0
    }
    pub fn setCurrent(&mut self, current: Option<usize>) {
        if let Some(c) = current {
            self.current = c;
        }
    }
    pub fn setIgnore(&mut self, ignore: Vec<usize>) {
        self.ignore = ignore
    }

    pub fn increase(&mut self) {
        let mut current = self.current;
        // Avanza en todas la posiciones, hasta encontrar una posicion valida que no este ignorada
        for _ in 0..self.len {
            current += 1;

            if current == self.len {
                current = 0;
            }
            if !self.ignore.contains(&current) {
                break;
            }
        }
        self.current = current
    }

    pub fn decrease(&mut self) {
        let mut current = self.current;

        for _ in 0..self.len {
            // Si es 0 que vuelva a la ultima pocision para que no se rompa el programa con un -1
            if current == 0 {
                current = self.len - 1
            } else {
                current -= 1;
            }
            if !self.ignore.contains(&current) {
                break;
            }
        }
        self.current = current
    }

    pub fn apply_action(&mut self, action: IndexAction) {
        match action {
            IndexAction::Increase => self.increase(),
            IndexAction::Decrease => self.decrease(),
        }
    }
}
