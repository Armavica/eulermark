-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main = do
  contents <- readFile "013.in"
  let nums = map read . lines $ contents :: [Integer]
  putStrLn . take 10 . show . sum $ nums
