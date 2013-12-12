-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main
  = print
  $ squaredSumOfIntegers - sumOfSquaredIntegers
    where integers = [1..100] :: [Int]
          sumOfIntegers = sum integers
          squaredSumOfIntegers = sumOfIntegers * sumOfIntegers
          squaredIntegers = zipWith (*) integers integers
          sumOfSquaredIntegers = sum squaredIntegers
