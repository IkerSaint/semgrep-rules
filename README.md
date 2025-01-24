# Public Semgrep rules

This repository contains Semgrep rules developed by me and made available to the public. They are part of my personal experience performing security audits, code reviews, vulnerability reseach, and other projects. 

Visit [Testing Handbook](https://appsec.guide/docs/static-analysis/semgrep/) for Semgrep guidance.

## Using Semgrep

The easiest way to run the rules is to clone this repository, navigate to the root folder of your project, and run individual rules using the command below :

```shell
$ semgrep --config /path/to/semgrep-rules/semgreprule.yml
```

To run all rules from the cloned repository:

```shell
$ semgrep --config /path/to/semgrep-rules/ .
```

## Useful flags

Semgrep will run against all supported code files except for those in your `.gitignore` file. If you want to run the rules against all files and directories, including those in your `.gitignore`, add the `--no-git-ignore` flag.

```shell
$ semgrep --config /path/to/semgrep-rules/ . --no-git-ignore
```

You can also tell Semgrep to ignore files and directories that match any pattern. For instance, if you want to tell Semgrep to ignore all Go test files you can run the following:

```shell
$ semgrep --config /path/to/semgrep-rules/ . --exclude='*_test.go'
```

Use `-o` to output results to a file:

```shell
$ semgrep --config /path/to/semgrep-rules/hanging-goroutine.yml -o leaks.txt'
```

## Rules
