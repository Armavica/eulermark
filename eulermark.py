# Copyright (C) 2013 Jorge Aparicio

BS4_NOT_FOUND = False
try:
    import bs4
except:
    BS4_NOT_FOUND = True
import collections
import json
import math
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


def tim2float(tim):
    num, prefix = tim.split()
    return float(num) * {'s': 1, 'ms': 1e-3, 'us': 1e-6}[prefix]


def mem2float(mem):
    return float(mem.split(' ')[0])


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


def count_lines(filename, comment):
    count = 0

    with open(filename) as f:
        for line in f:
            if line.strip() and not line.startswith(comment):
                count += 1

    return count


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
        time = settings['time']
        timeout = settings['timeout']
        compiler = settings["compiler"]
        input_needs_extension = settings["input_needs_extension"]
        interpreter = settings["interpreter"]
        interpreter_extension = settings["interpreter_extension"]
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

    tims = {}
    mems = {}
    for ext in valid_extensions:
        if pid + ext in contents:
            print('Found {} ({})'.format(language[ext], ext))

            # compile
            if ext in compiler:
                if shutil.which(compiler[ext][0]):
                    print('  Compiling...', end=' ')
                    inp = pid + (ext if ext in input_needs_extension else '')
                    o_flag = ['-o', pid] if ext in ['.c', '.cpp'] else []
                    if sub.call(compiler[ext] + o_flag + [inp],
                                stdout=sub.DEVNULL):
                        print('FAIL!\n  Skipping\n')
                        continue
                else:
                    print('  Compiler not found!\n  Skipping\n')
                    continue
                print('DONE')

            # validate
            print('  Validating...', end=' ')
            out = './' + pid + interpreter_extension.get(ext, '')
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
                exe = interpreter.get(ext, []) + [out]
                stmt = 'sub.call({}, stdout=sub.DEVNULL)'.format(exe)
                timeit = sub.check_output(['python3.3',
                                           '-m', 'timeit',
                                           '-s', setup, stmt])

                timeit = timeit.decode('utf-8').rstrip('\n')
                print(timeit, end=', ')

                num, prefix = timeit.split(':')[1].split()[:2]
                tims[ext] = ' '.join([num, prefix.rstrip('ec')])

                mem = 0
                for i in range(3):
                    p = sub.Popen([time, '-f', '%M'] + exe,
                                  stderr=sub.PIPE, stdout=sub.DEVNULL)
                    p.wait()
                    s = p.stderr.read().decode('utf-8').rstrip('\n')
                    if int(s) > mem:
                        mem = int(s)
                mem /= 1024
                n = 2 - int(math.log10(mem))
                mem = ('{:.' + str(n) + 'f} MB').format(mem)
                mems[ext] = mem
                print('max of 3:', mem)

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

    if tims:
        tims = collections.OrderedDict(sorted(tims.items(),
                                       key=lambda x: tim2float(x[1])))
        mems = collections.OrderedDict(sorted(mems.items(),
                                       key=lambda x: mem2float(x[1])))

        with open(pid + '.json', 'w') as f:
            json.dump({'tim': tims, 'mem': mems}, f, indent=2)

        print('Summary')
        for key, value in tims.items():
            print('{}\t\t{}\t\t{}'.format(key, value, mems[key]))

    print()
    os.chdir('../../..')


def fetch(pid):
    problem = "https://projecteuler.net/problem={}".format(pid)
    pid = pid2str(pid)
    pdir = '/'.join(pid)

    os.makedirs(pdir, exist_ok=True)

    print(pid + ' - Fetching...', end=' ')
    sys.stdout.flush()

    with urllib.request.urlopen(problem) as url:
        soup = bs4.BeautifulSoup(url)

    title = '{} - {}'.format(pid, soup.h2.text)
    soup = soup.find('div', role='problem')

    if soup:
        p, ps = soupwalker(soup, '', [])
        assert(not p)

        os.chdir(pdir)

        with open(pid + '.md', 'w') as f:
            f.write(title + '\n')
            f.write('-' * len(title) + '\n\n')

            for p in ps:
                f.write('\n'.join(textwrap.wrap(p, width=79)) + '\n\n')

        os.chdir('../../..')
        print('DONE')
    else:
        print('NOT FOUND!')


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
        comment = settings['comment']

    if not os.path.exists('/'.join(pid)):
        print("{} - directory doesn't exist!\n".format(pid))
        return

    os.chdir('/'.join(pid))

    if not os.path.exists(pid + '.json'):
        print("{} - timing file (.json) doesn't exist!\n".format(pid))
        os.chdir('../../..')
        return

    with open(pid + '.json') as f:
        results = json.load(f)

    tims = results['tim']
    mems = results['mem']

    tims = collections.OrderedDict(sorted(tims.items(),
                                   key=lambda x: tim2float(x[1])))
    mems = collections.OrderedDict(sorted(mems.items(),
                                   key=lambda x: mem2float(x[1])))
    min_t_ext = min(tims, key=lambda x: tim2float(tims[x]))
    min_m_ext = min(mems, key=lambda x: mem2float(mems[x]))
    min_t = tim2float(tims[min_t_ext])
    min_m = mem2float(mems[min_m_ext])

    shutil.copyfile(pid + '.md', 'README.md')

    with open('README.md', 'a') as f:
        f.write('Language | Time | rTime | Mem | rMem | LoC\n')
        f.write('--- | :---: | :---: | :---: | :---: | :---:\n')
        for ext, tim in tims.items():
            mem = mems[ext]
            t = tim2float(tim)
            m = mem2float(mem)
            f.write('{} | {} | {}% | {} | {}% | {}\n'.
                    format(language[ext],
                           tim if ext != min_t_ext else '**' + tim + '**',
                           round(100 * t / min_t),
                           mem if ext != min_m_ext else '**' + mem + '**',
                           round(100 * m / min_m),
                           count_lines(pid + ext, comment[ext])))


def table():
    with open('settings.json') as f:
        settings = json.load(f)

    exts = settings['valid_extensions']
    language = settings['language']
    min_tims = {}
    min_t_exts = {}
    timss = {}
    pids = []

    for root, dirs, files in os.walk('.'):
        for f in files:
            if f.endswith('.json') and root != '.':
                with open(os.path.join(root, f)) as _:
                        results = json.load(_)

                tims = results['tim']
                min_t_ext = min(tims, key=lambda x: tim2float(tims[x]))
                min_tim = tims[min_t_ext]
                min_t = tim2float(min_tim)
                tims = {key: round(100 * tim2float(tims[key]) / min_t)
                        for key in tims}

                pid = f.split('.')[0]
                timss[pid] = tims
                min_tims[pid] = min_tim
                min_t_exts[pid] = min_t_ext
                pids.append(pid)

    with open('README.md', 'w') as f:
        f.write('pid ')
        for ext in exts:
            f.write('| {} '.format(language[ext]))
        f.write('\n :---:' + ' | :---:' * len(exts) + '\n')
        for pid in sorted(pids):
            f.write('[{}]({})'.format(pid, '/'.join(pid)))
            for ext in exts:
                rel = timss[pid].get(ext)
                if ext == min_t_exts[pid]:
                    f.write(' | **{}**'.format(min_tims[pid]))
                elif rel:
                    f.write(' | {}%'.format(rel))
                else:
                    f.write(' | -')
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
