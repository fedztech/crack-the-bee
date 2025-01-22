


pub trait WordReader<T> {
    fn lines(self) -> std::io::Lines<T>;
}
