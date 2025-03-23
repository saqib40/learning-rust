mod variables;
mod datatypes;
mod string;
mod conditionals;
mod iterative;
mod mutability;
mod ownership_stack;
mod move_semantics;
mod borrow;
mod structs;

fn main() {
    variables :: variables();
    datatypes :: datatypes();
    string :: string();
    conditionals :: conditionals();
    iterative :: iterative();
    mutability :: mutability();
    ownership_stack :: ownership_stack();
    move_semantics :: move1();
    borrow :: mutable :: mutable();
    borrow :: immutable ::immutable();
    borrow :: rules :: rules();
    structs ::classic::classic();
    structs::elipsis::elipsis();
    structs::heap_trait::heap_trait();
    structs::implementation::implement();
    structs::stack_trait::stack_trait();
    structs::tuple::tuple();
    structs::unit::unit();
}
