odin
====

Odin is a tool for launching web searches from your terminal.

configuration
-------------

Odin is configured with YAML, and comes preconfigured with the configuration in [odin.yaml](odin.yaml).

To add new templates and aliases, create your own config file in `~/.config/odin.yaml`.

### example

```yaml
# this option controls whether or not this config will inherit from
# the defualt config. everything in parent configs can be overridden
# by child configs.
root: false

# defines a single template named `rodarmor.com`. templates must
# be URLs, at the moment, and are passed all arguments in an array
# named `args`. we use the `join` filter to join them into a space-
# separated string
# 
# with this template definition `odin open rodarmor.com kaomoji` will
# point your browser at the url `https://rodarmor.com/kaomoji`
templates:
  rodarmor.com: https://rodarmor.com/{{args | join}}

# alises allow you to define alternate names for templates.
#
# this alias definition allows you to use `rc` as an alternate
# name for `rodarmor.com`, so `odin open rc blaster` and
# `odin open rodarmor.com blaster` will do the same thing
aliases:
  rc: rodarmor.com
```

subcommands
-----------

Odin supports the following subcommands:

- `odin open TEMPLATE ARGS...`: Render `TEMPLATE` with `ARGS` and launch browser
- `odin print TEMPLATE ARGS...`: Render `TEMPLATE` with `ARGS` and print
- `odin dump`: Print the current config file
- `odin help`: Get help

contributions
-------------

Your feature requests, pull request, suggestions, and additions to the default config are most appreciated!
