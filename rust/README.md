'!' after a function name means macro

Rust has two rules for mutable and immutable references. They are very important, but also easy to remember because they make sense.

    Rule 1: If you have only immutable references, you can have as many as you want. 1 is fine, 3 is fine, 1000 is fine. No problem.
    Rule 2: If you have a mutable reference, you can only have one. Also, you can't have an immutable reference and a mutable reference together.
