use prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR;
use prettytable::{Cell, Row, Table};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct InstrumentedBase {
    counts: [usize; InstrumentedBase::COLUMNS],
}
impl InstrumentedBase {
    const NEW: usize = 0;
    const CLONE: usize = 1;
    const DROP: usize = 2;
    const EQ: usize = 3;
    const PARTIAL_CMP: usize = 4;
    const CMP: usize = 5;

    const COLUMNS: usize = 6;

    pub fn counts_names() -> [&'static str; InstrumentedBase::COLUMNS] {
        ["new", "clone", "drop", "eq", "partial_cmp", "cmp"]
    }

    pub fn set(&mut self, c: [usize; InstrumentedBase::COLUMNS]) {
        self.counts = c;
    }

    pub fn get(&self) -> [usize; InstrumentedBase::COLUMNS] {
        self.counts
    }
}
impl std::fmt::Debug for InstrumentedBase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let name = InstrumentedBase::counts_names();
        let n: Vec<_> = name.iter().zip(self.counts.iter()).collect();
        n.fmt(f)
    }
}

#[derive(Eq)]
pub struct Instrumented<T> {
    value: T,
    base: Rc<RefCell<InstrumentedBase>>,
}

impl<T> std::fmt::Debug for Instrumented<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.value.fmt(f)
    }
}

/// Conversion
impl<T> Instrumented<T> {
    pub fn new(value: T, base: Rc<RefCell<InstrumentedBase>>) -> Self {
        base.borrow_mut().counts[InstrumentedBase::NEW] += 1;
        Self { value, base }
    }
}

/// Semi regular
impl<T> Clone for Instrumented<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.base.borrow_mut().counts[InstrumentedBase::CLONE] += 1;
        Self {
            value: self.value.clone(),
            base: self.base.clone(),
        }
    }
}

/// Semi regular
impl<T> Drop for Instrumented<T> {
    fn drop(&mut self) {
        self.base.borrow_mut().counts[InstrumentedBase::DROP] += 1;
    }
}

/// Regular
impl<T> PartialEq for Instrumented<T>
where
    T: PartialEq,
{
    fn eq(&self, x: &Self) -> bool {
        self.base.borrow_mut().counts[InstrumentedBase::EQ] += 1;
        self.value.eq(&x.value)
    }
}

/// Totally-ordered
impl<T> PartialOrd for Instrumented<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, x: &Self) -> Option<std::cmp::Ordering> {
        self.base.borrow_mut().counts[InstrumentedBase::PARTIAL_CMP] += 1;
        self.value.partial_cmp(&x.value)
    }
}

/// Totally-ordered
impl<T> Ord for Instrumented<T>
where
    T: Ord,
{
    fn cmp(&self, x: &Self) -> std::cmp::Ordering {
        self.base.borrow_mut().counts[InstrumentedBase::CMP] += 1;
        self.value.cmp(&x.value)
    }
}

pub fn table_count_operations<F>(mut i: usize, j: usize, f: F)
where
    F: Fn(&mut [Instrumented<u64>]),
{
    let mut table = Table::new();
    table.set_format(*FORMAT_NO_BORDER_LINE_SEPARATOR);
    let hader = InstrumentedBase::counts_names()
        .iter()
        .map(|x| Cell::new(x))
        .collect();
    table.set_titles(Row::new(hader));
    while i <= j {
        let vec = rand_vec(i);

        let c = count_operations(vec, &f)
            .get()
            .iter()
            .map(|x| Cell::new(&x.to_string()))
            .collect();
        table.add_row(Row::new(c));

        i <<= 1;
    }
    table.printstd();
}

fn rand_vec(i: usize) -> Vec<u64> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rnd = thread_rng();
    let mut vec: Vec<u64> = vec![];
    vec.reserve(i);
    (0..i).for_each(|k| vec.push(k as u64));
    vec.shuffle(&mut rnd);

    vec
}

pub fn count_operations<T, F>(vec: Vec<T>, f: F) -> InstrumentedBase
where
    F: Fn(&mut [Instrumented<T>]),
{
    let base = Rc::new(RefCell::new(Default::default()));
    let mut vec: Vec<Instrumented<T>> = vec
        .into_iter()
        .map(|x| Instrumented::new(x, base.clone()))
        .collect();
    f(&mut vec);
    let base2: RefCell<InstrumentedBase> = (*base).clone();
    let base3: InstrumentedBase = *base2.borrow();
    base3
}

#[cfg(test)]
mod tests {
    use super::count_operations;
    use super::InstrumentedBase;
    use std::default::Default;
    #[test]
    fn it_sort1() {
        let mut vec = Vec::new();
        (0..4).for_each(|k| vec.push(k));
        let one = count_operations(vec, |x| x.sort());
        let mut def: InstrumentedBase = Default::default();
        def.set([4, 0, 0, 0, 3, 0]);
        assert_eq!(def, one);
    }
    #[test]
    fn it_sort2() {
        let mut vec = Vec::new();
        (0..4).for_each(|k| vec.push(3 - k));
        let one = count_operations(vec, |x| x.sort());
        let mut def: InstrumentedBase = Default::default();
        def.set([4, 0, 0, 0, 6, 0]);
        assert_eq!(def, one);
    }
    #[test]
    fn print() {
        let n = count_operations::<u64, _>(vec![], |_x| ());
        assert_eq!("[(\"new\", 0), (\"clone\", 0), (\"drop\", 0), (\"eq\", 0), (\"partial_cmp\", 0), (\"cmp\", 0)]", format!("{:?}", n));
    }
}
