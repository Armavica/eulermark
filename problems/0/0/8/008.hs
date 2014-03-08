-- Copyright (C) 2013 Jorge Aparicio

import Data.Char (digitToInt)

main :: IO()
main = do
  contents <- readFile "008.in"
  let digits = concat . lines $ contents
  print . fst . foldr maxProduct (0, "") $ digits

maxProduct :: Char -> (Int, String) -> (Int, String)
maxProduct a (n, s@(b:c:d:e:_)) = (max n m, a:s)
  where m = product . map digitToInt $ [a,b,c,d,e]
maxProduct a (n, s) = (n, a:s)
