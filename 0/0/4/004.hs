-- Copyright (C) 2013 Jorge Aparicio

import Data.Maybe (mapMaybe)

main :: IO()
main
  = print
  . head
  . filter isPalindrome
  . mapMaybe fst
  $ iterate next (Nothing, nums)
    where nums = [[x * y | y <- [x,x-1..start]] | x <- [end,end-1..start]]
          start = 100 :: Int
          end = 999

next :: Integral a => (Maybe a, [[a]]) -> (Maybe a, [[a]])
next (_, [[]]) = (Nothing, [[]])
next (_, (x:xs):[]) = (Just x, [xs])
next (_, x@(xh:xt):y@(yh:_):zs)
  | xh > yh = (Just xh, xt:y:zs)
  | otherwise = (h, x:zs')
    where (h, zs') = next (Nothing, y:zs)

isPalindrome :: (Integral a, Show a) => a -> Bool
isPalindrome n = s == reverse s
  where s = show n
