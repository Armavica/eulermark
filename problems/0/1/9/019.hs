-- Copyright (C) 2014 Jorge Aparicio

import Data.Time.Calendar (fromGregorian)
import Data.Time.Calendar.WeekDate (toWeekDate)

main :: IO()
main
  = print
  . length
  . filter (\(_, _, d) -> d == 7)
  . map toWeekDate
  $ [ fromGregorian y m 1 | y <- [1901..2000], m <- [1..12] ]
