import os
import sys
import shlex
from pathlib import Path


def get_common_root(paths):
    if not paths:
        return None
    common_path = Path(os.path.commonpath(paths))
    return str(common_path)


def read_file_content(file_path):
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            return file.read()
    except Exception as e:
        return f"Error reading file: {str(e)}"


def process_files(file_paths):
    root_path = get_common_root(file_paths)
    result = []

    for path in file_paths:
        content = read_file_content(path)
        relative_path = os.path.relpath(path, root_path)
        result.append(f"```{relative_path}\n{content}\n```")

    return root_path, "\n\n".join(result)


def main(input_string):
    file_paths = shlex.split(input_string)

    root_path, formatted_content = process_files(file_paths)

    print(f"Root path: {root_path}\n")
    print(formatted_content)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py \"path1 path2 path3 ...\"")
        sys.exit(1)

    main(sys.argv[1])