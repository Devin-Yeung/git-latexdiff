# TODO List

- Add `--no-capture` args, which indicate that the log of pdflatex
will be shown on `stdout`
- Add `--log-level` args
- Add `--abort-if-error` args, which indicate that the program will abort
if errors in the compilation stage
- Allow user to pass in extra args to `latexdiff`, `pdflatex` ...
- Add windows support, this because `skim` use `tuikit` which does not
support windows. One possible solution is to use `fzf` instead of skim on Windows target,
a new arg `--use-fzf` will also be added
(`zoxide` impl a fzf wrapper, maybe can get some ideas from its impl)
- Use regex to match the compile error message,
See [LaTeX-Workshop's impl](https://github.com/James-Yu/LaTeX-Workshop/blob/f65d9e4e437a1fe206842f0ae9245e3181b11ad8/src/components/parser/latexlog.ts)
- Current index can be compared