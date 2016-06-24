# docbase_cli

Docbase CLI written in Rust.

This software is still *ALPHA* quality.

## Installation

`git clone` this repo and run `cargo install`.

Now you can use `docbase-cli`.

## Envvar

Please set `DOCBASE_TOKEN` to your docbase api token or use `token` option.

## Usage

- Browse your docbase (Normal usage).

```
$ docbase-cli
```

- Specify your docbase token with option.

```
$ docbase-cli -t "your docbase token"
```

or

```
$ docbase-cli --token "your docbase token"
```

- Specify your team domain

```
$ docbase-cli -d foobar
```

or

```
$ docbase-cli --domain foobar
```

- Specify your team and group

```
$ docbase-cli -d foobar -g baz
```

or

```
$ docbase-cli --domain foobar --group baz
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

