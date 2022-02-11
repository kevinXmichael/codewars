<html lang="en">
<body>
<small>
    Create a function that takes a positive integer and returns the next bigger number that can be formed by rearranging
    its digits. For example:

    12 ==> 21
    513 ==> 531
    2017 ==> 2071
    nextBigger(num: 12) // returns 21
    nextBigger(num: 513) // returns 531
    nextBigger(num: 2017) // returns 2071
    If the digits can't be rearranged to form a bigger number, return -1 (or nil in Swift):

    9 ==> -1
    111 ==> -1
    531 ==> -1
    nextBigger(num: 9) // returns nil
    nextBigger(num: 111) // returns nil
    nextBigger(num: 531) // returns nil
</small>
<br>
<br>

<?php
function noBiggerInt($num, $digits)
{
    if ($num <= 10) {
        return true;
    }
    $sameDigits = true;
    for ($i = 1; $i < count($digits); $i++) {
        $sameDigits &= ($digits[$i] === $digits[0]);
    }

    // TODO: find next bigger int
    return $sameDigits;
}

function nextBigger($num)
{
    $digits = str_split($num);
    if (noBiggerInt($num, $digits)) {
        return -1;
    }
    return $num;
}

echo nextBigger(12); // should be 21
?>
</body>
</html>