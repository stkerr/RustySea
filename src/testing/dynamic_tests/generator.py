import random

ops = [ # op, function, exponent range, name, allow_negative_x, allow_negative_y
    ['+', lambda x,y: x+y, 256, 'add', True, True],
    ['-', lambda x,y: x-y, 256, 'sub', True, True],
    ['*', lambda x,y: x*y, 64, 'mul', True, True],
    #['/', lambda x,y: x/y],
    ['<<', lambda x,y: x<<abs(y), 16, 'bitshl', False, False],
    ['>>', lambda x,y: x>>abs(y), 16, 'bitshr', False, False],
    ['^', lambda x,y: x^y, 256, 'bitxor', True, True],
    ['|', lambda x,y: x|y, 256, 'bitor', True, True],
    ['&', lambda x,y: x&y, 256, 'bitand', True, True]
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

    tests = 1000
    for i in range(tests):
        op = random.choice(ops)

        x = random.randint(-2<<op[2], 2<<op[2])
        if not op[4]:
            x = abs(x)

        y = random.randint(-2<<op[2], 2<<op[2])
        if not op[5]:
            y = abs(y)

        test = template % (
                "dynamic_test_%s_%s" % (op[3], i),
                hex(x).replace('L',''),
                op[0],
                hex(y).replace('L',''),
                hex(op[1](x,y)).replace('L','')
             )
        f.write(test)
