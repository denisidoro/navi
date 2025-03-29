# Developers documentation

This documentation entry is destined to contributors of the navi project
and open-source repository.

## Versioning Scheme

| Type  | Description                                                                                      |
|-------|--------------------------------------------------------------------------------------------------|
| Major | Anything which introduces a major breaking change. The bash to rust rewrite was such an example. |
| Minor | Almost everything.                                                                               |
| Fix   | A fix, just like its name. It should be micro releases with minimal changes.                     |

## Deprecation of features

Once you introduce a feature, you need to have a clear view of when we're
going to remove its support within navi.

In order to offer stability to the users, we prefer having 10 minor versions
between the deprecation notice and the removal of its support.

````txt
Version where the feature is being deprecated: 0.10.0
Version where the support is dropped: 0.20.0
````

> [!NOTE]
> This rule is not absolute and each feature deprecation needs to be handled
> carefully given its own circumstances, but try to stick as close as possible
> to this rule.
