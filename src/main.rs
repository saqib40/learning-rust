mod variables;
mod datatypes;
mod string;
mod conditionals;
mod iterative;
mod mutability;
mod ownership_stack;

fn main() {
    variables :: variables();
    datatypes :: datatypes();
    string :: string();
    conditionals :: conditionals();
    iterative :: iterative();
    mutability :: mutability();
    ownership_stack :: ownership_stack();
}
