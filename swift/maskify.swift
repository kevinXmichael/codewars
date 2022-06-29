// Usually when you buy something, you're asked whether your credit card number, phone number or answer to your most secret question is still correct. However, since someone could look over your shoulder, you don't want that shown on your screen. Instead, we mask it.

// Your task is to write a function maskify, which changes all but the last four characters into '#'.

// Examples
// "4556364607935616" --> "############5616"
//      "64607935616" -->      "#######5616"
//                "1" -->                "1"
//                 "" -->                 ""

import Foundation

func maskify(_ string:String) -> String {
    var result: String = ""
    for (index, char) in string.enumerated() {
        result += string.count - index > 4 ? "#" : String(char)
    }
    return result
}

print(maskify("halloworld"))
print(maskify("123456789"))
print(maskify("12345"))
print(maskify("1234"))
print(maskify("123"))
print(maskify(""))
