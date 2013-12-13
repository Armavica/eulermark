-- Copyright (C) 2013 Jorge Aparicio

import Data.Numbers.Primes (primes)

main :: IO()
main
  = print
  . sum
  . takeWhile (<limit)
  $ primes
    where limit = 2000000 :: Int
