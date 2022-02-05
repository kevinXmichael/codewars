<html lang="en">
<body>
<small>
    Write a function that takes a string of braces, and determines if the order of the braces is valid. It should return
    true if the string is valid, and false if it's invalid.

    This Kata is similar to the Valid Parentheses Kata, but introduces new characters: brackets [], and curly braces {}.
    Thanks to @arnedag for the idea!

    All input strings will be nonempty, and will only consist of parentheses, brackets and curly braces: ()[]{}.

    What is considered Valid?
    A string of braces is considered valid if all braces are matched with the correct brace.

    Examples
    "(){}[]" => True
    "([{}])" => True
    "(}" => False
    "[(])" => False
    "[({})](]" => False
</small>
<br>
<br>

<?php
function validBraces($braces) {
    $opened = array();
    // Take one char at a time:
    foreach (str_split($braces) as $char) {
        // An opening brace/bracket/paren is always valid:
        if (in_array($char, str_split('([{'))) {
            // Add new opened brace:
            $opened[] = $char;
        }
        // A closing brace/bracket/paren must match last opened:
        else if ($char == ')') {
            // Remove last opened brace for check:
            if (array_pop($opened) !== '(') {
                return false;
            }
        }
        else if ($char == ']') {
            // Remove last opened brace for check:
            if (array_pop($opened) !== '[') {
                return false;
            }
        }
        else if ($char == '}') {
            // Remove last opened brace for check:
            if (array_pop($opened) !== '{') {
                return false;
            }
        }
    }
    return count($opened) == 0;
}
?>
</body>
</html>