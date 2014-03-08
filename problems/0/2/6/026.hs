-- Copyright (C) 2014 Jorge Aparicio

import Data.Numbers.Primes (primes)
import Data.List (maximumBy)
import Data.Function (on)

main :: IO()
main =
  print .
  maximumBy (compare `on` cycleSize) .
  takeWhile (<1000) $
  primes

cycleSize :: Int -> Int
cycleSize = cycleSize' 0 10

cycleSize' :: Int -> Int -> Int -> Int
cycleSize' _ 0 _ = 0
cycleSize' s 1 _ = s
cycleSize' s n d
  | n < d     = cycleSize' s (10*n) d
  | otherwise = cycleSize' (s+1) (n `rem` d) d
