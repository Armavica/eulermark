-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main
  = print
  $ head
  [ a * b * c
  | c <- [halfPerimeter,halfPerimeter-1..1]
  , b <- [c-1,c-2..1]
  , let a = perimeter - b - c
  , a*a + b*b == c*c
  ] where perimeter = 1000 :: Int
          halfPerimeter = perimeter `div` 2
