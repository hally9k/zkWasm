from string import Template
import sys

# print(sys.argv[1])
size = int(sys.argv[1])

elements = ""
for i in range(size):
    elements = elements + str(i)
    elements = elements + ", "

s=Template('''
#define FAILED $size
#define RIGHT $right

unsigned long long array[$size] = {
    $elements
};
''')

d={
    'size': size,
    'right': size-1,
    'elements': elements,
}

print(s.substitute(d))