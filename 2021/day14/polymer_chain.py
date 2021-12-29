'''Usage: polymer_chain.py [num_iterations] [input_file]'''
import sys
import collections

def parse_input(input_filename):
  with open(input_filename) as fp:
    chain = fp.readline().strip()
    rules = {}
    for line in fp:
      line = line.strip()
      if not line:
        continue
      sequence, insertion = line.split('->')
      rules[sequence.strip()] = insertion.strip()

    return chain, rules


def apply_iteration(state, rules):
  new_state = collections.defaultdict(lambda: 0)
  for sequence, count in state.items():
    # Single letter sequences don't do anything
    if len(sequence) == 1:
      new_state[sequence] += count
    if sequence in rules:
      for generated in [sequence[0] + rules[sequence],
                        rules[sequence] + sequence[1]]:
        if generated in rules:
          new_state[generated] += count
        else:
          new_state[generated[0]] += count

  return new_state

if __name__ == '__main__':
  chain, rules = parse_input(sys.argv[2])

  # States are just going to be a dictionary of sequences, plus single letters
  # when there is no rule that matches representing the polymer chain
  state = collections.defaultdict(lambda: 0)
  for a, b in zip(list(chain), list(chain[1:]) + ['']):
    sequence = a + b
    if sequence in rules:
      state[sequence] += 1
    else:
      # This sequence doesn't exist, but we need to track the 'leading' element
      # in the chain so it can exist in the next state.
      state[sequence[0]] += 1

  for i in range(int(sys.argv[1])):
    state = apply_iteration(state, rules)

  # Figure out the number of elements of each type
  total_counts = collections.defaultdict(lambda: 0)
  for sequence, count in state.items():
    total_counts[sequence[0]] += count

  sorted_counts = sorted(total_counts.items(), key=lambda x: x[1])
  # print(sorted_counts)
  print(sorted_counts[-1][1] - sorted_counts[0][1])
