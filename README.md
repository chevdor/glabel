# glabel

## Intro

A utility to import (=`get`) and export (=`apply`) Github labels.

## Install

    cargo install --git https://github.com/chevdor/glabel --locked
    glabel --help

## Usage

### Help

    USAGE:
        glabel [FLAGS] <SUBCOMMAND>

    FLAGS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

    SUBCOMMANDS:
        apply    Apply a label set
        get      Get/download labels
        help     Print this message or the help of the given subcommand(s)

### Get

    USAGE:
        glabel get [FLAGS] [OPTIONS] <REPOSITORY>

    ARGS:
        <REPOSITORY>    The repo string for now in the form owner/repo such as chevdor/foobar

    FLAGS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

    OPTIONS:
        -o, --output <OUTPUT>    The output filename

Without passing the `--output|-o` flags, the labels will be shown in your terminal as:

     - bug                      [d73a4a]: Something isn't working
     - documentation            [0075ca]: Improvements or additions to documentation
     - duplicate                [cfd3d7]: This issue or pull request already exists
     - enhancement              [a2eeef]: New feature or request
     - good first issue         [7057ff]: Good for newcomers
     - help wanted              [008672]: Extra attention is needed
     - invalid                  [e4e669]: This doesn't seem right
     - question                 [d876e3]: Further information is requested
     - wontfix                  [ffffff]: This will not be worked on

However, if you provide an output file, the yaml will be stored as:

    ---
    name: chevdor/glabel
    description: Import from chevdor/glabel
    labels:
      - name: bug
        description: "Something isn't working"
        color: d73a4a
      - name: documentation
        description: Improvements or additions to documentation
        color: 0075ca
      - name: duplicate
        description: This issue or pull request already exists
        color: cfd3d7
      - name: enhancement
        description: New feature or request
        color: a2eeef
      - name: good first issue
        description: Good for newcomers
        color: 7057ff
      - name: help wanted
        description: Extra attention is needed
        color: "008672"
      - name: invalid
        description: "This doesn't seem right"
        color: e4e669
      - name: question
        description: Further information is requested
        color: d876e3
      - name: wontfix
        description: This will not be worked on
        color: ffffff

### Apply

    USAGE:
        glabel apply [FLAGS] --token <TOKEN> <REPOSITORY> <INPUT>

    ARGS:
        <REPOSITORY>    The repo string for now in the form owner/repo such as chevdor/foobar
        <INPUT>         The filename where your set is stored

    FLAGS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

    OPTIONS:
        -t, --token <TOKEN>    If you follow good security practices, your GITHUB_TOKEN should not have
                               write access to your repos. Here since we need to write labels, we use
                               another variable for the token with write access. It is highly
                               recommended to pass this as an Environment variable [env: TOKEN=<your
                               admin token>]

### Documentation

If you feel fancy (and lazy), you may even generate a documentation from your tags using [tera-cli](https://github.com/chevdor/tera-cli).

Using the [template](templates/doc.md.tera) in this repo and the following command:

    tera --template templates/doc.md.tera doc/sample_yaml.yaml

Will generate the following output:

    This is the documentation for your set named `chevdor/glabel`.

    It contains 9 labels:

    - `bug`: *Something isn't working*
    - `documentation`: *Improvements or additions to documentation*
    - `duplicate`: *This issue or pull request already exists*
    - `enhancement`: *New feature or request*
    - `good first issue`: *Good for newcomers*
    - `help wanted`: *Extra attention is needed*
    - `invalid`: *This doesn't seem right*
    - `question`: *Further information is requested*
    - `wontfix`: *This will not be worked on*
