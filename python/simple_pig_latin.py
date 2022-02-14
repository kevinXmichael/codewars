def pig_it(text):
    result = ""
    for word in text.split():
        if word.isalpha():
            result += word[1:len(word)] + word[0] + "ay "
        else:
            result += word
    return result.strip()


def pig_it2(text):
    result = []
    for word in text.split():
        if word.isalpha():
            result.append(word[1:len(word)] + word[0] + "ay")
        else:
            result.append(word)
    return ' '.join(result)


print(pig_it('Pig latin is cool'), "\nigPay atinlay siay oolcay")
print(pig_it('Hello world !'), "\nelloHay orldway !")

# Move the first letter of each word to the end of it, then add "ay" to the end of the word. Leave punctuation marks untouched.
#
# Examples
# pig_it('Pig latin is cool') # igPay atinlay siay oolcay
# pig_it('Hello world !')     # elloHay orldway !
