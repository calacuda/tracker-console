pub trait Incrementable {
    fn get_big_n(&self) -> usize {
        16
    }

    fn small_inc(&mut self);
    fn big_inc(&mut self) {
        for _ in 0..self.get_big_n() {
            self.small_inc();
        }
    }

    fn small_dec(&mut self);
    fn big_dec(&mut self) {
        for _ in 0..self.get_big_n() {
            self.small_dec();
        }
    }
}
