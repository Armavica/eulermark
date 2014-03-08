-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main = do
  contents <- readFile "018.in"
  let triangle = map (map read . words) $ lines contents :: [[Int]]
  print . maximum . foldl1 traverse $ triangle

traverse
  :: [Int]  -- previous row
  -> [Int]  -- current row
  -> [Int]  -- traversed row
traverse p = traverse' (0 : p)

traverse' :: [Int] -> [Int] -> [Int]
traverse' [p] [c] = [p+c]
traverse' (p1:p2:ps) (c:cs) = (c + max p1 p2) : traverse' (p2:ps) cs

