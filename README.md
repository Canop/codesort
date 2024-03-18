## Code-Sort

[![AGPL][s2]][l2] [![Latest Version][s1]][l1] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/code-sort.svg
[l1]: https://crates.io/crates/code-sort

[s2]: https://img.shields.io/badge/license-AGPL-blue.svg
[l2]: LICENSE

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3490?code-sort


Sometimes, an enum or a struct can become big enough that it's easier to keep track of its use, for example `match` arms, when the code is alphabetically sorted.

Of course you can't just sort lines if you want the code to keep working and the comments (and annotations, attributes, etc.) to follow the code.

Sorting code manually is a tedious and boring task.

**code-sort** can sort struct fields, struct variants, enum variants, type declarations, match arms of any kind, function declarations, etc.

## Example


## Install code-sort

## Usage

## Integrate code-sort in a code editor

By default, **code-sort** takes the code to sort from stdin and writes the sorted code to stdout.

It can sort either the whole input, a given range (with `--range`), or select the best range around a given line (using `--around`).

You can also change the input and output to be files, with `--src` and `--dst`.

Those options make it easy to integrate code-sort in any editor. See below for vim.

## Integrate code-sort in vim

#### Sort the selection

When you don't want to sort the whole range (for example because there's a specific entry that you want to keep at the beginning), you can specify the range.

Visually select the selection, then type `:`, then `!code-sort`.
This changes the input to

```
:'<,'>!code-sort
```

Press the `enter` key: the selection is sorted with code-sort.

#### Add a binding to sort around the current line

You don't usually have to select the zone to sort.
You can ask code-sort to automatically select the zone to sort around your current line.

With the following binding typing the leader key then 'cs' will automatically select the set of blocs around the current line and sort it. The buffer won't be saved so you can undo or save yourself.

```
" sort the optimal range around the current line
nnoremap <Leader>cs ma<ESC>:execute ":%!code-sort --around ".line('.')<CR>`a
```
Explanation of the command:

* the current position is saved in the `a` register with `ma`
* the command including the line number is built then executed with `:execute`
* the previous position is then restored

## Supported Code kinds

3 code analyzers are available now:

* Rust, which should work for C too
* Java
* JavaScript

Contributions for other languages would be welcome.

## Licence

**code-sort** is licenced under [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html).

You're free to use the **code-sort** program to sort the code of your choice, even commercial.
