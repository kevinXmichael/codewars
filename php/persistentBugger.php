<html lang="en">
<body>
Write a function, persistence, that takes in a positive parameter num and returns its multiplicative persistence, which is the number of times you must multiply the digits in num until you reach a single digit.

For example (Input --> Output):
39 --> 3 (because 3*9 = 27, 2*7 = 14, 1*4 = 4 and 4 has only one digit)
999 --> 4 (because 9*9*9 = 729, 7*2*9 = 126, 1*2*6 = 12, and finally 1*2 = 2)
4 --> 0 (because 4 is already a one-digit number)

<?php
function persistence(int $num): int {
    return real_persistence($num, 0);
}

function real_persistence(int $num, int $persistence): int {
    if ($num < 10) {
        return $persistence;
    }
    $result = 1;
    $nums = str_split(strval($num));
    foreach ($nums as $num) {
        $result *= $num;
    }
    return real_persistence($result, $persistence + 1);
}

print "Result is " . persistence(39) . "<br>";
print "Result is " . persistence(999) . "<br>";
print "Result is " . persistence(4) . "<br>";
?>
</body>
</html>