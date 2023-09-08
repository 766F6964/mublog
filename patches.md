# Module system
## Overview
To allow adding additional functionality to μblog, a module system is available.
Each module is store in modules/tags and contains all files (html, js, css) needed to run the module.
Additionally, each module can contain a `config.json` file which contains meta information about the module.
This includes various operations that need to be performed on the templates, such 
as inserting data before/after a keyword into the template and copying module 
files to the output directory.

## Operations
- `insert_before` / `insert_after` inserts data before/after a keyword in the template. The data can be loaded from a file or 
directly passed, similar to a variable:

```yaml
operations:
  - operation: "insert_before"
    source_type: "file/string"
    source_data: "file.txt"
    pattern: "keyword"
```
- `replace` replaces a keyword in the template with data from a file or directly passed data:

```yaml
operations:
  - operation: "replace"
    source_type: "file/string"
    source_data: "file.txt"
    pattern: "keyword"
```

## Env Variables
Before modules are processed, some environment variables are set, which can be used in the config file.
The following variables are available to be used in a module config file:
- pages
- posts

## TODO:
- Find alternative name for modules, e.g. patches, plugins, extensions, addons, etc.
- Switch from json to yaml to allow comments in config files