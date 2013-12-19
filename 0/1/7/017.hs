-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main
  = print
  . sum
  . map (length . filter (\x -> x /= ' ' && x /= '-') . wordify)
  $ [1..1000]

wordify :: Int -> String
wordify 1 = "one"
wordify 2 = "two"
wordify 3 = "three"
wordify 4 = "four"
wordify 5 = "five"
wordify 6 = "six"
wordify 7 = "seven"
wordify 8 = "eight"
wordify 9 = "nine"
wordify 10 = "ten"
wordify 11 = "eleven"
wordify 12 = "twelve"
wordify 13 = "thirteen"
wordify 15 = "fifteen"
wordify 18 = "eighteen"
wordify 20 = "twenty"
wordify 30 = "thirty"
wordify 40 = "forty"
wordify 50 = "fifty"
wordify 80 = "eighty"
wordify 1000 = wordify 1 ++ " thousand"
wordify n
  | n < 20  = wordify (n-10) ++ "teen"
  | n < 100 = if u == 0
                 then wordify t ++ "ty"
                 else wordify (n - u) ++ '-' : wordify u
  | otherwise = if u == 0 && t == 0
                   then wordify h ++ " hundred"
                   else wordify (n - 10*t - u) ++ " and " ++ wordify (10*t + u)
    where u = n `mod` 10
          t  = ((n - u) `div` 10) `mod` 10
          h = n `div` 100
