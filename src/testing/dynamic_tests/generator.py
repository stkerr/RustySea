import random

ops = [
    ['+', lambda x,y: x+y],
    ['-', lambda x,y: x-y],
    ['*', lambda x,y: x*y],
    #['/', lambda x,y: x/y],
    #['<<', lambda x,y: x<<y],
    #['>>', lambda x,y: x>>y],
    ['^', lambda x,y: x^y],
    ['|', lambda x,y: x|y],
    ['&', lambda x,y: x&y]
]

header = """
extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;
"""

template = """
op_test!(%s, "%s" %s "%s" == "%s");
"""

with open('dynamic_tests.rs', 'w') as f:
    f.write(header)

    tests = 10
    for i in range(tests):
        op = random.choice(ops)

        x = random.randint(-2<<128, 2<<128)
        y = random.randint(-2 << 128, 2 << 128)

        test = template %(
                "dynamic_test_%s" % i,
                hex(x),
                op[0],
                hex(y),
                hex(op[1](x,y))
             )
        f.write(test)
