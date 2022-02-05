<html lang="en">
<body>
<small>
    Take 2 strings s1 and s2 including only letters from a to z. Return a new sorted string, the longest possible,
    containing distinct letters - each taken only once - coming from s1 or s2.

    Examples:
    a = "xyaabbbccccdefww"
    b = "xxxxyyyyabklmopq"
    longest(a, b) -> "abcdefklmopqwxy"

    a = "abcdefghijklmnopqrstuvwxyz"
    longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"
</small>
<br>
<br>

<?php
function longest($a, $b)
{
    $split = array_unique(str_split($a . $b));
    sort($split);
    return implode($split);
}

$longest = longest("xyaabbbccccdefww", "xxxxyyyyabklmopq");
echo strcmp($longest, "abcdefklmopqwxy");
?>
</body>
</html>