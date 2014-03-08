-- Copyright (C) 2013 Jorge Aparicio

import Data.Char

main :: IO()
main = print . sum . map digitToInt . show . factorial $ 100

factorial :: Integer -> Integer
factorial n = product [1..n]
