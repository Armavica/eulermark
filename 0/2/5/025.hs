-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main
  = print
  . snd
  . head
  . filter (\(f,_) -> numberOfDigits f == 1000)
  $ zip fibonacci positiveIntegers
    where positiveIntegers = [1..] :: [Int]

fibonacci :: [Integer]
fibonacci = map fst $ iterate (\(a,b) -> (b,a+b)) (1,1)

numberOfDigits :: Integer -> Int
numberOfDigits = numberOfDigits' 0

numberOfDigits' :: Int -> Integer -> Int
numberOfDigits' d 0 = d
numberOfDigits' d n = numberOfDigits' (d+1) (n `div` 10)
