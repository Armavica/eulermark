-- Copyright (C) 2013 Jorge Aparicio

import Data.Numbers.Primes (primes)

main :: IO()
main
  = print
  . last
  . primeFactors
  $ number
    where number = 600851475143 :: Int

primeFactors :: Integral a => a -> [a]
primeFactors n = factor n primes

factor :: Integral a => a -> [a] -> [a]
factor n (p:ps)
  | p*p > n        = [n]
  | n `mod` p == 0 = p : factor (n `div` p) (p:ps)
  | otherwise      = factor n ps
