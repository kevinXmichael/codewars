<html lang="en">
<body>
<small>
    You probably know the "like" system from Facebook and other pages. People can "like" blog posts, pictures or other
    items. We want to create the text that should be displayed next to such an item.

    Implement the function which takes an array containing the names of people that like an item. It must return the
    display text as shown in the examples:

    [] --> "no one likes this"
    ["Peter"] --> "Peter likes this"
    ["Jacob", "Alex"] --> "Jacob and Alex like this"
    ["Max", "John", "Mark"] --> "Max, John and Mark like this"
    ["Alex", "Jacob", "Mark", "Max"] --> "Alex, Jacob and 2 others like this"
</small>
<br>
<br>

<?php
function likes($names)
{
    switch (count($names)) {
        case 0:
            return "no one likes this";
        case 1:
            return $names[0] . " likes this";
        case 2:
            return $names[0] . " and " . $names[1] . " like this";
        case 3:
            return $names[0] . ", " . $names[1] . " and " . $names[2] . " like this";
        default:
            return $names[0] . ", " . $names[1] . " and " . (count($names) - 2) . " others like this";
    }
}

// CodeWars does not like the match expression as this task does only provide PHPv7
//    return match (count($names)) {
//        0 => "no one likes this",
//        1 => $names[0] . " likes this",
//        2 => $names[0] . " and " . $names[1] . " like this",
//        3 => $names[0] . ", " . $names[1] . " and " . $names[2] . " like this",
//        default => $names[0] . ", " . $names[1] . " and " . (count($names) - 2) . " others like this",
//    };

// This is another very smart solution for this problem:
//function likes($names): string {
//    return array(
//        '0' => "no one likes this",
//        '1' => "{$names[0]} likes this",
//        '2' => "{$names[0]} and {$names[1]} like this",
//        '3' => "{$names[0]}, {$names[1]} and {$names[2]} like this",
//        '4' => "{$names[0]}, {$names[1]} and ". (sizeof($names) - 2) ." others like this")[min(4, sizeof($names))];
//}
?>
</body>
</html>