-- Copyright (C) 2013 Jorge Aparicio

import Data.Numbers.Primes (primes)

main :: IO()
main = print (primes !! 10000 :: Int)
