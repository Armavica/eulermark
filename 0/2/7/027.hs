-- Copyright (C) 2014 Jorge Aparicio

import Data.List (foldl')

main :: IO()
main
  = print
  . uncurry (*)
  . fst
  . foldl' origami ((0,0), 0)
  $ [(a,b) | a <- [-limit..limit], b <- [-limit..limit]]
    where limit = 999
          origami ((a,b),n) (a',b')
            | any ((<2) . polyval (a',b')) [0..n] = ((a,b),n)
            | any (not . isPrime . polyval (a',b')) [0..n] = ((a,b),n)
            | otherwise = ((a',b'),primeLength (a',b'))

polyval :: (Int,Int) -> Int -> Int
polyval (a,b) n = n * n + a * n + b

isPrime :: Int -> Bool
isPrime n = not $ any divisible $ takeWhile notTooBig (2:[3,5..])
  where divisible p = n `mod` p == 0
        notTooBig p = p*p <= n

primeLength :: (Int,Int) -> Int
primeLength (a,b)
  = length $ takeWhile ((\x -> x>1 && isPrime x) . polyval (a,b)) [0..]
