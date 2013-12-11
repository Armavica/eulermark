import collections
import json
import os
import shutil
import subprocess as sub
import sys


def digest(inp):
    cap = lambda x: x if x < 1000 else 999

    try:
        bolus = sorted(list(map(cap, map(int, inp.split('-')))))
    except:
        raise ValueError

    if len(bolus) == 1:
        bolus *= 2

    low, high = bolus

    return range(low, high + 1)


def pid2str(pid):
    return '{0:0>3}'.format(pid)


def timing2float(timing):
    num, prefix = timing.split()
    return float(num) * {'s': 1, 'ms': 1e-3, 'us': 1e-6}[prefix]


def directory(args):
    if args:
        command = args.pop(0)

        if command == 'create':
            try:
                assert(len(args) == 1)
                pids = digest(args.pop())
            except (AssertionError, ValueError):
                print(create_help)
                sys.exit(1)

            for pid in pids:
                create(pid)
            sys.exit(0)
        elif command == 'prune':
            try:
                assert(len(args) == 1)
            except AssertionError:
                print(prune_help)
                sys.exit(1)

            path = args.pop()
            if os.path.exists(path):
                prune(path)
                sys.exit(0)
            else:
                print("path doesn't exist!")
                sys.exit(1)

    print(directory_help)
    sys.exit(1)


def create(pid):
    os.makedirs('/'.join(pid2str(pid)), exist_ok=True)


def prune(path):
    contents = os.listdir(path)

    if contents:
        for item in contents:
            full_path = os.path.join(path, item)
            if os.path.isdir(full_path):
                prune(full_path)

        if not os.listdir(path):
            os.rmdir(path)
    else:
        os.rmdir(path)


def problem(args):
    if args:
        command = args.pop(0)

        if command == 'benchmark':
            try:
                assert(len(args) == 1)
                pids = digest(args.pop())
            except (AssertionError, ValueError):
                print(benchmark_help)
                sys.exit(1)

            for pid in pids:
                benchmark(pid)
            sys.exit(0)

    print(problem_help)
    sys.exit(1)


def benchmark(pid):
    pid = pid2str(pid)

    with open('settings.json') as f:
        settings = json.load(f)
        valid_extensions = settings['valid_extensions']
        language = settings['language']
        timeout = settings['timeout']
        compiler = settings["compiler"]
        input_needs_extension = settings["input_needs_extension"]
        interpreter = settings["interpreter"]
        output_needs_extension = settings["output_needs_extension"]
        byproduct = settings["byproduct"]

    if os.path.exists('/'.join(pid)):
        os.chdir('/'.join(pid))
    else:
        print("{} - directory doesn't exist!\n".format(pid))
        return

    contents = os.listdir()
    if pid + '.md' not in contents:
        print('{} - problem statement (.md) was not found!\n'.format(pid))
        os.chdir('../../..')
        return

    if pid + '.ans' not in contents:
        print('{} - answer file (.ans) was not found!\n'.format(pid))
        os.chdir('../../..')
        return

    with open(pid + '.md') as f:
        print(f.readline() + f.readline())

    with open(pid + '.ans') as f:
        ans = f.read()

    timing = {}
    for ext in valid_extensions:
        if pid + ext in contents:
            print('Found {} ({})'.format(language[ext], ext))

            # compile
            if ext in compiler:
                if shutil.which(compiler[ext][0]):
                    print('  Compiling...', end=' ')
                    inp = pid + (ext if ext in input_needs_extension else '')
                    if sub.call(compiler[ext] + [inp],
                                stderr=sub.DEVNULL,
                                stdout=sub.DEVNULL):
                        print('FAIL!\n  Skipping\n')
                        continue
                else:
                    print('  Compiler not found!\n  Skipping\n')
                    continue
                print('DONE')

            # validate
            print('  Validating...', end=' ')
            out = './' + pid + (ext if ext in output_needs_extension else '')
            try:
                sol = sub.check_output(interpreter.get(ext, []) + [out],
                                       timeout=timeout,
                                       stderr=sub.DEVNULL)
            except sub.CalledProcessError:
                sol = None
                print('FAIL!')
            except sub.TimeoutExpired:
                sol = None
                print('TIMEOUT!')

            # benchmark
            if sol.decode('utf-8') == ans:
                print('CORRECT!\n  Benchmarking...', end=' ')
                setup = 'import subprocess as sub'
                stmt = 'sub.call({}, stdout=sub.DEVNULL)'.format(
                    interpreter.get(ext, []) + [out])
                timeit = sub.check_output(['python3.3',
                                           '-m', 'timeit',
                                           '-s', setup, stmt])

                timeit = timeit.decode('utf-8').rstrip('\n')
                print(timeit)

                num, prefix = timeit.split(':')[1].split()[:2]
                timing[ext] = ' '.join([num, prefix.rstrip('ec')])

            elif sol is not None:
                print('WRONG!\n  Skipping')

            # clean
            if ext in byproduct:
                print('  Cleaning...', end=' ')
                if isinstance(byproduct[ext], str):
                    byproduct_folder = byproduct[ext]

                    for f in os.listdir(byproduct_folder):
                        os.remove(os.path.join(byproduct_folder, f))

                    os.rmdir(byproduct_folder)
                else:
                    byproduct_extensions = byproduct[ext]
                    for extension in byproduct_extensions:
                        os.remove(pid + extension)
                print('DONE')

            print()

    if timing:
        timing = collections.OrderedDict(sorted(timing.items(),
                                         key=lambda x: timing2float(x[1])))

        with open(pid + '.json', 'w') as f:
            json.dump(timing, f, indent=2)

        print('Summary')
        for key, value in timing.items():
            print('  {}\t{}'.format(key, value))

    print()
    os.chdir('../../..')

directory_help = """usage: ./directory.py <command> <arg>

available commands are:
    create  create a directory tree that represents the problem id (pid)
    prune   recursively remove empty directories under path"""

create_help = """usage: ./directory.py create <pid|range>
create a directory tree that represents the problem id (pid)

examples:
    ./directory_create.py 123
        creates the following directory layout
            .
            `-- 1
                `-- 2
                    `-- 3

    ./directory_create.py 8-12
        creates the following directory layout
            .
            `-- 0
                |-- 0
                |   |-- 8
                |   `-- 9
                `-- 1
                    |-- 0
                    |-- 1
                    `-- 2"""

prune_help = """usage: ./directory.py prune <path>
recursively removes empty directories under path"""

problem_help = """usage: ./problem.py <command> <arg>

available commands are:
    benchmark   benchmark problem using several programming languages"""

benchmark_help = """usage: ./problem.py benchmark <pid|range>
benchmark problem using several programming languages"""
