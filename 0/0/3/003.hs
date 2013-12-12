-- Copyright (C) 2013 Jorge Aparicio

main :: IO()
main
  = print
  . maximum
  . primeFactors
  $ n
    where n = 600851475143 :: Int

primeFactors :: Integral a => a -> [a]
primeFactors n = primeFactors' n 2

primeFactors' :: Integral a => a -> a -> [a]
primeFactors' 1 _ = [1]
primeFactors' m n
  | m `rem` n == 0 = n:primeFactors' (m `div` n) n
  | otherwise      = primeFactors' m (n+1)
