-- Copyright (C) 2013 Jorge Aparicio

import Data.List (group)
import Data.Numbers.Primes (primes)

main :: IO()
main
  = print
  . head
  . filter (\x -> nDivisors x > threshold)
  $ triangleNumbers
    where triangleNumbers = scanl1 (+) [1..] :: [Int]
          threshold = 500

nDivisors :: Integral a => a -> Int
nDivisors = product . map ((+1) . length) . group . primeFactors

primeFactors :: Integral a => a -> [a]
primeFactors n = factor n primes

factor :: Integral a => a -> [a] -> [a]
factor n (p:ps)
  | p*p > n        = [n]
  | n `mod` p == 0 = p : factor (n `div` p) (p:ps)
  | otherwise      = factor n ps
