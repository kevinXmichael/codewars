// You receive a list of todos that contain a "created" property meaning when the todo was created first.
// Your task is to return how many todos are older than 24 hours (expired).
// If a date cannot be parsed, do not count it. Consider wrong dates like:
//      date = -1
//      date = undefined
//      date = false
//      date = "" (empty)
// The purpose of the task is to get you working with date objects.

// SOLUTION
function expiredTodos(todos = []) {
  function isOlderThan1Day(date) {
    if (date && date > 0 && date !== "") {
      const today = new Date()
      const ONE_DAY = 1000 * 60 * 60 * 24
      const dateDiff = today - new Date(date)
      const olderThan1Day = dateDiff >= ONE_DAY
      return olderThan1Day
    }
    return false
  }
  return todos.reduce((previousValue, currentValue) => {
    return isOlderThan1Day(currentValue.created)
      ? previousValue + 1
      : previousValue
  }, 0)
}

// TESTS
function testExpiredTodos() {
  const tests = [
    {
      todos: [
        { created: new Date("2020-01-01") },
        { created: new Date("2021-01-01") },
        { created: new Date("2042-01-01") },
        { created: new Date() }
      ],
      result: 2
    },
    {
      todos: [
        { created: new Date("202-1-01") },
        { created: false },
        { created: "" },
        { created: -1 },
        { created: undefined }
      ],
      result: 0
    },
    {
      todos: [
        { created: new Date("2036-01-01") },
        { created: new Date("2042-01-01") },
        { created: new Date() }
      ],
      result: 0
    },
    {
      todos: [
        { created: new Date("2016-01-01") },
        { created: new Date("2017-02-01") },
        { created: new Date("2018-03-01") }
      ],
      result: 3
    }
  ]

  const assert = require("assert")
  describe("expiredTodos", () => {
    for (const test of tests) {
      it(`Expecting ${test.result} expired todos`, () => {
        assert.equal(test.result, expiredTodos(test.todos))
      })
    }
  })
}

testExpiredTodos()
