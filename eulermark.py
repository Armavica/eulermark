import os
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

directory_help = """usage: ./directory.py <command> [<arg>]

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
