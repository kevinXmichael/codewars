// Create a function add(n)/Add(n) which returns a function that always adds n to any number

func add(_ n: Int) -> ((Int) -> Int) {
    // Or just use "{ n + $0 }" as return value.

    return { (other: Int) in
        return n + other
    }
}

let addOne = add(1)
print(addOne(3))
