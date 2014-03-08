-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main
  = print
  . sum
  . map wordify
  $ [1..1000]

wordify :: Int -> Int
wordify 1 = length "one"
wordify 2 = length "two"
wordify 3 = length "three"
wordify 4 = length "four"
wordify 5 = length "five"
wordify 6 = length "six"
wordify 7 = length "seven"
wordify 8 = length "eight"
wordify 9 = length "nine"
wordify 10 = length "ten"
wordify 11 = length "eleven"
wordify 12 = length "twelve"
wordify 13 = length "thirteen"
wordify 15 = length "fifteen"
wordify 18 = length "eighteen"
wordify 20 = length "twenty"
wordify 30 = length "thirty"
wordify 40 = length "forty"
wordify 50 = length "fifty"
wordify 80 = length "eighty"
wordify 1000 = wordify 1 + length "thousand"
wordify n
  | n < 20    = wordify u + length "teen"
  | n < 100   = if u == 0
                   then wordify (t `div` 10) + length "ty"
                   else wordify t + wordify u
  | otherwise = if tau == 0
                   then wordify (h `div` 100) + length "hundred"
                   else wordify h + length "and" + wordify tau
    where tau = n `mod` 100
          u = n `mod` 10
          t  = tau - u
          h = n - tau
