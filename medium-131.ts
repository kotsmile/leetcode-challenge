function isPolindrome(arr: string): boolean {
  for (let i = 0; i < Math.floor(arr.length / 2); i++) {
    if (arr[i] !== arr[arr.length - 1 - i]) {
      return false
    }
  }

  return true
}

// ["abc"] 0
//    ["a", "bc"] 1
//       ["a", "b", "c"] 11
//    ["ab","c"] 10
//       ["a", "b", "c"] 11

// R: [0, 2**(length - 1)] -> string[]

function partitionLong(s: string): string[][] {
  let answer: string[][] = []
  outer: for (let i = 0; i < Math.pow(2, s.length - 1); i++) {
    let partition: string[] = []
    let startSlice = 0
    for (let index = 0; index < s.length - 1; index++) {
      if ((Math.pow(2, index) & i) != 0) {
        const endSlice = index + 1
        const candidate = s.slice(startSlice, endSlice)
        if (!isPolindrome(candidate)) {
          continue outer
        }
        partition.push(candidate)
        startSlice = endSlice
      }
    }

    const candidate = s.slice(startSlice)
    if (!isPolindrome(candidate)) {
      continue outer
    }
    partition.push(candidate)
    answer.push(partition)
  }
  return answer
}

console.log(partitionLong('aab'))

function partition(s: string): string[][] {
  const result: string[][] = []

  const dp = (i: number, s: string, st: string[]) => {
    if (i >= s.length) {
      result.push([...st])
      return
    }

    for (let j = i; j < s.length; j++) {
      const vic = s.slice(i, j + 1)
      if (isPolindrome(vic)) {
        st.push(vic)
        dp(j + 1, s, st)
        st.pop()
      }
    }
  }

  dp(0, s, [])

  return result
}

console.log(partition('aab'))
