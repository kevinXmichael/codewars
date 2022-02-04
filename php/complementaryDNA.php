<html lang="en">
<body>
<?php
function DNA_strand($dna) {
    if (empty($dna)) {
        return [];
    } else {
        $result = "";
        foreach (str_split($dna) as $char) {
            switch ($char) {
                case "A":
                    $result = $result . "T";
                    break;
                case "T":
                    $result = $result . "A";
                    break;
                case "C":
                    $result = $result . "G";
                    break;
                case "G":
                    $result = $result . "C";
                    break;
            }
        }
        return $result;
    }
}
echo DNA_strand("ATTGC")
?>
</body>
</html>