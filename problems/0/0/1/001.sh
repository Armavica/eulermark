# Copyright (C) 2014 Jorge Aparicio

acc=0

for (( i=0; i<1000; i+=1 )); do
  if [ $(($i % 3)) -eq 0 ] || [ $(($i % 5)) -eq 0 ]; then
    acc=$(($i + $acc))
  fi
done

echo $acc
