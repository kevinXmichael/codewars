def solution(string, markers):
    sentences = []
    for sentence in string.split("\n"):
        sentence_clean = sentence
        for idx, char in enumerate(sentence):
            if char in markers:
                sentence_clean = sentence[:idx]
                break
        sentences.append(sentence_clean.strip())
    return "\n".join(sentences)


print(solution("apples, pears # and bananas\ngrapes\nbananas !apples", ["#", "!"]))

# Complete the solution so that it strips all text that follows any of a set of comment markers passed in.
# Any whitespace at the end of the line should also be stripped out.
#
# Example:
#
# Given an input string of:
#
# apples, pears # and bananas
# grapes
# bananas !apples
# The output expected would be:
#
# apples, pears
# grapes
# bananas
# The code would be called like so:
#
# result = solution("apples, pears # and bananas\ngrapes\nbananas !apples", ["#", "!"])
# # result should == "apples, pears\ngrapes\nbananas"
