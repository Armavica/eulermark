-- Copyright (C) 2013 Jorge Aparicio

import Data.Char (ord)
import Data.List (sort)

import qualified Data.Text.IO as T
import qualified Data.Text as T

main :: IO()
main = do
  contents <- T.readFile "022.in"
  let names = map (T.dropAround (== '"')) . T.splitOn (T.pack ",") $ contents
  let sortedNames = sort names
  let nameScores = map (sum . map (subtract 64 . ord) . T.unpack) sortedNames
  let totalScore = sum $ zipWith (*) nameScores [1..(length nameScores)]
  print totalScore
