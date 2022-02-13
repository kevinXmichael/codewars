function sumStrings(a, b) {
  //   const getNum = (num) => (isNaN(num) ? BigInt("0") : BigInt(num))
  //   return (getNum(a) + getNum(b)).toString()
  return `${BigInt(a) + BigInt(b)}`
}

console.log(sumStrings("123", "456"), "579")

// Given the string representations of two integers, return the string representation of the sum of those integers.

// For example:

// sumStrings('1','2') // => '3'
// A string representation of an integer will contain no characters besides the ten numerals "0" to "9".
