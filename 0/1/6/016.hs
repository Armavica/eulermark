-- Copyright (C) 2013 Jorge Aparicio

import Data.Char (digitToInt)

main :: IO()
main = print . sum . map digitToInt . show $ base ^ power
  where base = 2 :: Integer
        power = 1000 :: Int
