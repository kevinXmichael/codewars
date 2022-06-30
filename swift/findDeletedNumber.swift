// An ordered sequence of numbers from 1 to N is given. One number might have deleted from it, then the remaining numbers were mixed. Find the number that was deleted.

// Example:

// The starting array sequence is [1,2,3,4,5,6,7,8,9]
// The mixed array with one deleted number is [3,2,4,6,7,8,1,9]
// Your function should return the int 5.
// If no number was deleted from the starting array, your function should return the int 0.

// Note: N may be 1 or less (in the latter case, the first array will be []).

func findDeletedNumber(_ array: [Int], _ mixArray: [Int]) -> Int {
    var deletedNumber = 0
    let sorted = mixArray.sorted()
    for (index, _) in sorted.enumerated() {
        guard index > 0 else { continue }

        let diff = abs(sorted[index-1] - sorted[index])
        if (diff > 1) {
            deletedNumber = sorted[index-1] + 1
            break
        }
    }
    return deletedNumber
}

// smart solution:
// func findDeletedNumber(_ array: [Int], _ mixArray: [Int]) -> Int {
//   return array.first(where: { !mixArray.contains($0) }) ?? 0
// }
