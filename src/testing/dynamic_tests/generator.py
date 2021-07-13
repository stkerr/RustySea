import random

ops = [ # op, function, exponent range, name
    ['+', lambda x,y: x+y, 256, 'add'],
    ['-', lambda x,y: x-y, 256, 'sub'],
    ['*', lambda x,y: x*y, 64, 'mul'],
    #['/', lambda x,y: x/y],
    #['<<', lambda x,y: x<<y],
    #['>>', lambda x,y: x>>y],
    ['^', lambda x,y: x^y, 256, 'bitxor'],
    ['|', lambda x,y: x|y, 256, 'bitor'],
    ['&', lambda x,y: x&y, 256, 'bitand']
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

        x = random.randint(-2<<op[2], 2<<op[2])
        y = random.randint(-2<<op[2], 2<<op[2])

        test = template % (
                "dynamic_test_%s_%s" % (op[3], i),
                hex(x).replace('L',''),
                op[0],
                hex(y).replace('L',''),
                hex(op[1](x,y)).replace('L','')
             )
        f.write(test)
