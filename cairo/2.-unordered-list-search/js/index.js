const list = [
  6,3,45, 5,79,13,0
]

const x1 = 5
const x2 = 10


let x1Found = {}
let x2Found = {}

for (let i = 0; i < list.length; i++) {
  if (!x1Found.found) {
    x1Found.found = x1 === list[i]
    x1Found.pos = i
  }

  if (!x2Found.found) {
    x2Found.found = x2 === list[i]
    x2Found.pos = i
  }
}

if (x1Found.found) {
  console.log("Found x1 on pos: ", x1Found.pos)
}

if (x2Found.found) {
  console.log("Found x2 on pos: ", x2Found.pos)
}

if (!x1Found.found) {
  console.log("x1 not found...")
}

if (!x2Found.found) {
  console.log("x2 not found...")
}

