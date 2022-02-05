<html lang="en">
<body>
<small>
    Write a function that takes in a string of one or more words, and returns the same string, but with all five or more
    letter words reversed (Just like the name of this Kata).
    Strings passed in will consist of only letters and spaces. Spaces will be included only when more than one word is
    present.

    Examples: spinWords( "Hey fellow warriors" ) => returns "Hey wollef sroirraw" spinWords( "This is a test") =>
    returns "This is a test" spinWords( "This is another test" )=> returns "This is rehtona test"
</small>
<br>
<br>

<?php
function spinWords(string $str): string
{
    $maxLength = 5;
    $result = "";
    foreach (explode(" ", $str) as $word) {
        if (strlen($word) >= $maxLength) {
            $result .= strrev($word) . " ";
        } else {
            $result .= $word . " ";
        }
    }
    return trim($result);
}

echo spinWords("Hello world this is a simple test");
?>
</body>
</html>