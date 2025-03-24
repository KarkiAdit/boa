mod infra;

success_tests! {
    add: "add.snek",
    add1: "add1.snek",
    binding: "binding.snek",
    nested_arith: "nested_arith.snek",
}

failure_tests! {
    duplicate_binding: "duplicate_binding.snek",
    unbound_id: "unbound_id.snek",
}
