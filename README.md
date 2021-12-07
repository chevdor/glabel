# glabel

## Intro

Utility to import/export Github labels.

## Usage

### Help

    USAGE:
        glabel [FLAGS] <SUBCOMMAND>

    FLAGS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

    SUBCOMMANDS:
        get     Get/download labels
        help    Print this message or the help of the given subcommand(s)

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
