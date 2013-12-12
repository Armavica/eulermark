# eulermark

Benchmark your solutions to [Project Euler](https://projecteuler.net) problems
written in several languages and display the results in tables using github
flavored markdown
([GFM](https://help.github.com/articles/github-flavored-markdown)).

Currently C, Haskell, Python, Ruby, Rust are supported. But you can add support
for more languages easily, and are encouraged to contribute back.

And hopefully the `contrib` branch will become a database of solutions and
benchmark results.

## TL; DR

A ~~picture~~ table is worth a thousand words - Head to the `contrib` branch,
you'll see all the benchmark results aggregated in a table, and if you click
any of the pids (problem ids) you'll see the ranking of solutions for that
particular problem.

## What does this exactly do?

eulermark uses a directory layout that looks like this:

```
$ ./directory.py create 0-1
$ tree .
.
`-- 0
    `-- 0
        |-- 0             # problem 0 folder
        |   |-- 000.ans   # the answer to the problem
        |   |-- 000.hs    # solution in haskell
        |   |-- 000.json  # benchmark results
        |   |-- 000.md    # problem statemet
        |   |-- 000.py    # solution in python
        |   |-- 000.rb    # solution in ruby
        |   |-- 000.rs    # solution in rust
        |   `-- README.md # problem statement + solution ranking
        `-- 1             # problem 1 folder
```

eulermark can then benchmark all the solutions found under the problem
directory.

```
$ ./problem.py benchmark 0
000 - Hello World!
------------------

Found haskell (.hs)
  Compiling... DONE
  Validating... CORRECT!
  Benchmarking... 1000 loops, best of 3: 1.21 msec per loop
  Cleaning... DONE

Found python (.py)
  Compiling... DONE
  Validating... CORRECT!
  Benchmarking... 10 loops, best of 3: 22.5 msec per loop
  Cleaning... DONE

Found ruby (.rb)
  Validating... CORRECT!
  Benchmarking... 100 loops, best of 3: 18 msec per loop

Found rust (.rs)
  Compiling... DONE
  Validating... CORRECT!
  Benchmarking... 100 loops, best of 3: 7.34 msec per loop
  Cleaning... DONE

Summary
  .hs   1.21 ms
  .rs   7.34 ms
  .rb   18 ms
  .py   22.5 ms
```

Using the results stored in the json file, eulermark can generate a ranking
of solutions to an specific problem and also aggregate all the results in a
table.

```
./generate.py ranking 1
./generate.py table
```

## Dependencies

* python, to run the scripts
* git, to manage your `personal` branch
* github, to render the GFM in the README.md files
* BeautifulSoup4 (optinal), to fetch problem statements

## Start your own

```
# 1) fork this repository on github then clone your new repo
$ git clone https://github.com/your-username/eulermark

# 2) start off your personal branch
$ git checkout -b personal

# 3) fetch problem 1 from ProjectEuler
$ ./problem.py fetch 1

# 4) solve the problem in as many languages as you can/want
$ $EDITOR 0/0/1/001.{py,rb}

# 5) write down the answer to problem 1
$ echo **** > 0/0/1/001.ans

# 6) benchmark your solutions to problem 1
$ ./problem.py benchmark 1

# 7) generate a ranking for problem 1
$ ./generate.py ranking 1

# 8) update your benchmark table
$ ./generate.py table

# 9) commit your solutions and results
$ git commit -a -m "solved problem 1 in python and ruby"

# 10) head to your repo in github and glance at your new tables

# 11) rinse and repeat
```

## Contributing

### I want to add support for language X

Hopefully, this should only require additions to the `compiler` and/or
`interpreter` sections in settings.json.

Please send your pull request to the `scratch` branch.

### I have a faster solution for problem X!

`faster` means a reduction in the execution time (duh!) caused by a change in
the source code of the solution and **not** by a change in the compiler flags.

If you introduce a new algorithm to solve the problem, then be sure to write
your name in the header comment.

If you improve an existing solution, then append your name to the header
comment.

Please send your pull request to the `contrib` branch.

## FAQ

### Q: How do I pass the flag -O to compiler X?

Open the settings.json file, locate the `compiler` object, then locate the
desired extension (e.g. .py) under it, and append the desired flags to the
list.

```
"compiler": {
  ".hs": ["ghc", "-O"],
...
},
```

### Q: eulermark failed to fetch problem x!

This is the first time I wrote an HTML -> markdown converter, so don't expect
it to be perfect ;-)

Please open an issue reporting the problem so I can look into it.

## Licensing

Problem statements (in .md files) are copyrigth of Project Euler, and are
distributed here under
[CC-BY-NC-SA 2.0 U.K](http://creativecommons.org/licenses/by-nc-sa/2.0/uk/).

All the source code is licensed under the [MIT license](LICENSE) and its
copyrigth of the authors listed in the header comment.
