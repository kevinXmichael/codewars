<html lang="en">
<body>
<small>
    Return the number (count) of vowels in the given string.
    We will consider a, e, i, o, u as vowels for this Kata (but not y).
    The input string will only consist of lower case letters and/or spaces.
</small>
<br>
<br>

<?php
function getCount($str)
{
    $vowelsCount = 0;
    $vowels = array("a", "e", "i", "o", "u");
//    foreach (str_split($str) as $char) {
//        if (in_array($char, $vowels)) {
//            $vowelsCount += 1;
//        }
//    }
    str_replace($vowels , '', $str, $vowelsCount);
    return $vowelsCount;
}

echo getCount("hallo");
?>
</body>
</html>