main :: IO()
main
  = print
  . sum
  . takeWhile (<upperLimit)
  . filter even
  . map fibonacci $ nonNegativeIntegers
    where nonNegativeIntegers = [0..]
          upperLimit = 4000000

fibonacci :: Int -> Int
fibonacci = (map fib [0..] !!)
  where fib 0 = 1
        fib 1 = 2
        fib n = fibonacci (n-2) + fibonacci(n-1)
