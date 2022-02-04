<html lang="en">
<body>
<small>
    Simple, given a string of words, return the length of the shortest word(s).
    String will never be empty and you do not need to account for different data types.
</small>
<br>
<br>

<?php
//function findShort($str){
//    return min(array_map('strlen', (explode(' ', $str))));
//}

function findShort($str){
    $smallest = PHP_INT_MAX;
    $words = explode(" ", $str);
    foreach ($words as $word) {
        $length = strlen($word);
        if ($length < $smallest) {
            $smallest = $length;
        }
    }
    return $smallest;
}

echo findShort("bitcoin take over the world maybe who knows perhaps");
?>
</body>
</html>