-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main = print $ iterate (scanl1 (+)) (repeat 1 :: [Int]) !! size !! size
  where size = 20 :: Int

