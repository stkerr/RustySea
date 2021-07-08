import random

ops = [ # op, function, exponent range
    ['+', lambda x,y: x+y, 256],
    ['-', lambda x,y: x-y, 256],
    ['*', lambda x,y: x*y, 64],
    #['/', lambda x,y: x/y],
    #['<<', lambda x,y: x<<y],
    #['>>', lambda x,y: x>>y],
    ['^', lambda x,y: x^y, 256],
    ['|', lambda x,y: x|y, 256],
    ['&', lambda x,y: x&y, 256]
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

    tests = 100
    for i in range(tests):
        op = random.choice(ops)
        op = ops[2]
        x = random.randint(-2<<op[2], 2<<op[2])
        y = random.randint(-2<<op[2], 2<<op[2])

        test = template %(
                "dynamic_test_%s" % i,
                hex(x).replace('L',''),
                op[0],
                hex(y).replace('L',''),
                hex(op[1](x,y)).replace('L','')
             )
        f.write(test)
