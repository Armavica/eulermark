-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main
  = print
  . sum
  . takeWhile (<upperLimit)
  . filter (\x -> x `rem` 3 == 0 || x `rem` 5 == 0)
  $ positiveIntegers
  where positiveIntegers = [1..] :: [Int]
        upperLimit = 1000
