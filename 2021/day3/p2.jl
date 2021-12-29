# find oxygen generator rating and CO2 scrubbing ratio

# consider the first bit of the diagnostic numbers

# 1. keep only numbers selected by the bit criteria of the metric.
# 2. if you have only one number left, stop. this is the number.
# 3. otherwise, repeat the process, considering the bit to the right.

# bit criterias:

# oxygen generator rating:
#   determine most common value (0 or 1) in the current bit position.
#   keep only numbers with that bit in that position.
#   if 0 and 1 are equally common, keep values w/ a 1 in the position.

# CO2 scrubber rating:
#   determine least common value (0 or 1) in the current bit position.
#   keep only numbers with that bit in that position. if 0 and 1 are
#   equally common, keep values with a 0 in the current bit position.

# multiply oxygen generator rating by CO2 scrubbing ratio to get answer.

