-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main
  = print
  . foldr1 lcm
  $ numbers
  where numbers = [2..20] :: [Int]
