-- Copyright (C) 2013 Jorge Aparicio

import Data.Array.IArray ((!), Array, assocs, listArray)
import Data.List (foldl1')

main :: IO()
main
  = print
  . fst
  . foldl1' (\x y -> if snd x > snd y then x else y)
  . assocs
  . collatzLengths
  $ 1000000

collatzLengths :: Int -> Array Int Int
collatzLengths n = arr
  where arr = listArray (1, n) $ 1:[1 + len x | x <- [2..n]]
        len x = if y <= n then arr ! y else 1 + len y
          where y = next x

next :: Int -> Int
next n = if even n then n `div` 2 else 3*n + 1

