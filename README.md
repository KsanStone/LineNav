# LineNav

Command Synopsis

```bash
lnav [options] [path1] [path2]...
```

If no paths are provided to the command, it counts lines in the current directory.

## Available options

### --verbose -v

In verbose mode, the Line Counter command provides a detailed tree view, showcasing every counted file in a hierarchical
structure along with the corresponding final line count result.

### --very-verbose -vv

Additionally, to the file tree shown by [verbose mode](#--verbose--v) this flag also shows the auto-detected file
encoding and file processing time.

### --all-files -a

The default behavior for files with 0 lines and files which could not be decoded it to hide them. This flag prevents
that, showing every file, while highlighting the corrupt and 0 loc ones. Be aware that files not included in the count,
such as files not having the right [file extensions](#--file-extensions--f) or not selected by the filter will still not be
shown.

### --exclude -x

The `exclude` parameter takes multiple names of files or directories to exclude from the search.

### --encoding -e

The encoding parameter specifies the file encoding to use while reading the files. Files which fail to decode using the
specified encoding will not be counted.

### --file-extensions -f

Limits the line counter to only counting files with the specified file extensions. The option takes in a comma separated
list of them.
For example:

```bash
lnav --file-extensions java,cpp,py
```

Will only search through `*.{java,cpp,py}`.

### --simple \[-s]

When in combination with verbose mode, it simplifies its output by not printing the file tree. Instead, each file is
printed along with its file path and line count.

```text
/path/to/file1.txt : 800
/file2.txt : 10
Total: 810
```

### --summary \[limit] \[-m]

Summarize the line count based on the file extension.
The optional limit will limit the result to the top n positions.