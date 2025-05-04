# Contribute documentation to Navi

If you don't want or can't code in Rust, we welcome all contributions,
even more so if it's related to documentation.

The documentation of Navi is currently made in Markdown.

## Markdown documentation

The documentation source files are located in the `docs/` folder and are mainly grouped by features.
The current documentation follows a structure where one folder equals one topic.

Here is a quick representation of the folder structure this documentation currently follows:

```txt
.
+-- docs
|   +-- examples
|   |   +-- <topic-examples>
|   +-- src
|   |   +-- <topic-source-files>
|   |   |   +-- <sorted-by-type>
|   +-- <topic>
|   |   +-- README.md
```

You can see that we have separated the `src` and `examples` folder from the topic with the intent to make it
easier to find each type of documentation.

> [!NOTE]
> It is recommended to not go deeper than 3 levels in the documentation.

