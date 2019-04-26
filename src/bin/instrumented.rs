use instrumented_stepanov::*; //count_operations;

fn main() {
    count_operations(2, 2 * 1024, |x| x.sort());
}
