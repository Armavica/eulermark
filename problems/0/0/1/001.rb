# Copyright (C) 2013 Jorge Aparicio

puts((1..999).select { |i| i % 3 == 0 || i % 5 == 0 }.inject(:+))
