-- Copyright (C) 2013 Jorge Aparicio

import Control.Arrow ((&&&))
import Control.Monad (liftM2)
import Data.Array.IArray ((!),Array,listArray)
import Data.Numbers.Primes (primes)
import Data.List (group)

limit :: Int
limit = 10000

primes' :: [Int]
primes' = takeWhile (<limit) primes

sums :: Array Int Int
sums = listArray (1,limit) $ 0 : map (sum . properDivisors) [2..limit]

amicable :: Int -> Int -> Bool
amicable x y = x /= y && (sums ! x) == y && (sums ! y) == x

main :: IO()
main
  = print
  . sum
  . filter (\x -> sums ! x < limit && amicable x (sums ! x))
  $ [2..limit]

properDivisors :: Int -> [Int]
properDivisors = init . divisors

divisors :: Int -> [Int]
divisors
  = foldl1 (liftM2 (*))
  . map (\(f,n) -> scanl (*) 1 (replicate n f))
  . factorize

factorize :: Int -> [(Int,Int)]
factorize = map (head &&& length) . group . primeFactors

primeFactors :: Int -> [Int]
primeFactors n = factor n primes'

factor :: Int -> [Int] -> [Int]
factor n (p:ps)
  | p*p > n        = [n]
  | n `mod` p == 0 = p : factor (n `div` p) (p:ps)
  | otherwise      = factor n ps
