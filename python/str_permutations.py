def permutations(seq):
    return list(set(permutations_(seq)))


def permutations_(seq):
    if not seq:
        yield seq
    else:
        for i in range(len(seq)):
            rest = seq[:i] + seq[i + 1:]
            for x in permutations(rest):
                yield seq[i:i + 1] + x


print(permutations("ab"))

# In this kata you have to create all permutations of an input string and remove duplicates, if present. This means, you have to shuffle all letters from the input in all possible orders.
# Examples:
# permutations('a'); # ['a']
# permutations('ab'); # ['ab', 'ba']
# permutations('aabb'); # ['aabb', 'abab', 'abba', 'baab', 'baba', 'bbaa']
# The order of the permutations doesn't matter.
