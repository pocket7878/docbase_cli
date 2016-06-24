# docbase_cli

Docbase CLI written in Rust.

This software is still *ALPHA* quality.

## Installation

`git clone` this repo and run `cargo install`.

Now you can use `docbase-cli`.

## Envvar

Please set `DOCBASE_TOKEN` to your docbase api token.

## Usage

- Browse your docbase (Normal usage).

```
$ docbase-cli
```

- Specify your team domain

```
$ docbase-cli -t foobar
```

or

```
$ docbase-cli --team foobar
```

- Specify your team and group

```
$ docbase-cli -t foobar -g baz
```

or

```
$ docbase-cli --team foobar --group baz
```

- Specify pager program


```
$ docbase-cli -p "lv"
```

or

```
$ docbase-cli --pager "lv"
```

Default pager is `less`

