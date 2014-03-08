main :: IO()
main
  = print
  . sum
  . takeWhile (<upperLimit)
  . filter even
  $ fibonacci
    where upperLimit = 4000000

fibonacci :: [Int]
fibonacci = map fst $ iterate (\(a,b) -> (b,a+b)) (1,1)
