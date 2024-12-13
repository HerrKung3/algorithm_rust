pub enum List<T> {
    Const(T, Box<List<T>>),
    Nil
}

impl <T>List<T> {

}

