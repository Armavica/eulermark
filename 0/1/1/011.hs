-- Copyright (C) 2013 Jorge Aparicio

import Data.Array.IArray ((!), Array, listArray)
import Data.Maybe (catMaybes)

stride :: Int
stride = 4

size :: Int
size = 20

dirs :: [(Int, Int)]
dirs = [(1,0), (0,1), (1,1), (-1,1)]

main :: IO()
main = do
  contents <- readFile "011.in"
  let nums = map read . words $ contents
  let arr = listArray ((1,1), (size,size)) nums :: Array (Int, Int) Int
  print . maximum . catMaybes $ [ arrProduct arr (y, x) dir
                                | x <- [1..size]
                                , y <- [1..size]
                                , dir <- dirs ]

arrProduct
  :: Array (Int, Int) Int -- array
  -> (Int, Int) -- beginning
  -> (Int, Int) -- direction
  -> Maybe Int -- product
arrProduct a b@(by, bx) d@(dy, dx)
  | ex > size || ey > size || ey < 1 = Nothing
  | otherwise = Just . product . map (a !) $ map (move b d) [0..stride-1]
    where ex = bx + (stride - 1) * dx
          ey = by + (stride - 1) * dy


move
  :: (Int, Int) -- beginning
  -> (Int, Int) -- direction
  -> Int -- steps
  -> (Int, Int) -- end
move (by, bx) (dy, dx) s = (by + s * dy, bx + s * dx)
