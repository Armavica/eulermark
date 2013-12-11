# Copyright (C) 2013 Jorge Aparicio

BS4_NOT_FOUND = False
try:
    import bs4
except:
    BS4_NOT_FOUND = True
import collections
import json
import os
import shutil
import subprocess as sub
import sys
import textwrap
import urllib.request


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


def soupwalker(soup, p, ps, in_blockquote=False, in_ul=False):
    image = '![{}](https://projecteuler.net/project/images/{}.gif)'

    for element in soup.contents:
        if isinstance(element, bs4.Comment):
            pass
        elif isinstance(element, bs4.NavigableString):
            element = element.strip('\r\n').replace('²', '^2')
            if element:
                p += element
        elif element.name in ['a', 'dfn', 'span']:
            (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
        elif element.name == 'b':
            if in_blockquote:
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
            else:
                p += '**'
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
                p += '**'
        elif element.name == 'br':
            if in_blockquote:
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
                p += '\n'
            elif in_ul:
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
                p += '\n  '
            else:
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
                if p:
                    ps.append(p)
                    p = ''
        elif element.name == 'blockquote':
            p += '```\n'
            in_blockquote = True
            (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
            in_blockquote = False
            p = p.rstrip('\n') + '\n```'
            ps.append(p)
            p = ''
        elif element.name == 'div':
            if in_blockquote:
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
                p += '\n'
            else:
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
                if p:
                    ps.append(p)
                    p = ''
        elif element.name == 'i':
            if in_blockquote:
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
            else:
                p += '*'
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
                p += '*'
        elif element.name == 'img':
            if '_le' in element['src']:
                p += '<='
            elif '_lt' in element['src']:
                p += '<'
            elif '_maps' in element['src']:
                p += '->'
            elif '_minus' in element['src']:
                p += '-'
            elif '_ne' in element['src']:
                p += '/='
            elif '_times' in element['src']:
                p += '*'
            elif '_015' in element['src']:
                p += image.format('p015', 'p_015')
            else:
                print('Unexpected image: ' + element['src'])
                sys.exit(1)
        elif element.name == 'li' and in_ul:
            p += '* '
            (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
            p += '\n'
        elif element.name == 'p':
            if in_blockquote:
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
                p += '\n'
            else:
                (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
                if p:
                    ps.append(p)
                    p = ''
        elif element.name == 'sub':
            p += '_'
            (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
        elif element.name == 'sup':
            p += '^'
            (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
        elif element.name == 'ul':
            in_ul = True
            (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
            in_ul = False
            ps.append(p.rstrip('\n'))
            p = ''
        elif element.name == 'var':
            p += '`'
            (p, ps) = soupwalker(element, p, ps, in_blockquote, in_ul)
            p += '`'
        else:
            print('Unexpected tag: ' + element.name)
            sys.exit(1)

    return (p, ps)


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
        elif command == 'fetch':
            if BS4_NOT_FOUND:
                print('BeautifulSoup4 is required to fetch problem statemets')
                sys.exit(1)

            try:
                assert(len(args) == 1)
                pids = digest(args.pop())
            except (AssertionError, ValueError):
                print(fetch_help)
                sys.exit(1)

            for pid in pids:
                fetch(pid)
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


def fetch(pid):
    problem = "https://projecteuler.net/problem={}".format(pid)
    pid = pid2str(pid)

    if not os.path.exists('/'.join(pid)):
        print("{} - directory doesn't exist!\n".format(pid))
        return

    with urllib.request.urlopen(problem) as url:
        soup = bs4.BeautifulSoup(url)

    title = '{} - {}'.format(pid, soup.h2.text)
    soup = soup.find('div', role='problem')

    if soup:
        p, ps = soupwalker(soup, '', [])
        assert(not p)

        os.chdir('/'.join(pid))

        with open(pid + '.md', 'w') as f:
            f.write(title + '\n')
            f.write('-' * len(title) + '\n\n')

            for p in ps:
                f.write('\n'.join(textwrap.wrap(p, width=79)) + '\n\n')

        os.chdir('../../..')
    else:
        print(pid + ' - problem not found!')


def generate(args):
    if args:
        command = args.pop(0)

        if command == 'ranking':
            try:
                assert(len(args) == 1)
                pids = digest(args.pop())
            except (AssertionError, ValueError):
                print(ranking_help)
                sys.exit(1)

            for pid in pids:
                ranking(pid)
            sys.exit(0)
        elif command == 'table':
            if args:
                print(table_help)
                sys.exit(1)

            table()
            sys.exit(0)

    print(generate_help)
    sys.exit(1)


def ranking(pid):
    pid = pid2str(pid)

    with open('settings.json') as f:
        settings = json.load(f)
        language = settings['language']

    if not os.path.exists('/'.join(pid)):
        print("{} - directory doesn't exist!\n".format(pid))
        return

    os.chdir('/'.join(pid))

    if not os.path.exists(pid + '.json'):
        print("{} - timing file (.json) doesn't exist!\n".format(pid))
        os.chdir('../../..')
        return

    with open(pid + '.json') as f:
        timing = json.load(f)

    timing = collections.OrderedDict(sorted(timing.items(),
                                     key=lambda x: timing2float(x[1])))
    min_t = timing2float(list(timing.items())[0][1])

    shutil.copyfile(pid + '.md', 'README.md')

    with open('README.md', 'a') as f:
        f.write('Language | Time | Relative\n--- | :---: | :---:\n')
        for ext, time in timing.items():
            t = timing2float(time)
            f.write('{} | {} | {}%\n'.format(language[ext],
                                             time,
                                             int(100 * t / min_t)))


def table():
    with open('settings.json') as f:
        settings = json.load(f)

    exts = settings['valid_extensions']
    language = settings['language']
    timings = {}
    pids = []

    for root, dirs, files in os.walk('.'):
        for f in files:
            if f.endswith('.json') and root != '.':
                with open(os.path.join(root, f)) as _:
                        timing = json.load(_)

                min_time = min(timing.values(), key=timing2float)
                timing = {key: timing2float(timing[key]) for key in timing}
                min_t = min(timing.values())
                timing = {key: int(100 * timing[key] / min_t)
                          for key in timing}

                pid = f.split('.')[0]
                timings[pid] = timing
                pids.append(pid)

    with open('README.md', 'w') as f:
        f.write('pid ')
        for ext in exts:
            f.write('| {} '.format(language[ext]))
        f.write('\n :---:' + ' | :---:' * len(exts) + '\n')
        for pid in pids:
            f.write(pid)
            for ext in exts:
                rel = timings[pid][ext]
                if rel == 100:
                    f.write(' | **{}**'.format(min_time))
                else:
                    f.write(' | {}%'.format(rel))
            f.write('\n')

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
    benchmark   benchmark problem using several programming languages
    fetch       fecth problem statement from ProjectEuler"""

benchmark_help = """usage: ./problem.py benchmark <pid|range>
benchmark problem using several programming languages"""

fetch_help = """usage: ./problem.py fetch <pid|range>
fecth problem statement from ProjectEuler"""

generate_help = """usage: ./generate.py <command> [<arg>]

available commands are:
    ranking     generate a ranking from a problem benchmark results
    table       aggregates all the benchmark results in a table"""

ranking_help = """usage: ./generate.py ranking <pid|range>
generate a ranking from a problem benchmark results"""

table_help = """usage: ./generate.py table
aggregates all the benchmark results in a table"""
