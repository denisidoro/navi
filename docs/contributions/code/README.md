# Contribute code to Navi

Navi is written in Rust, the widgets may be written in any language given it can be integrated with Navi.

If you take the example of the most common widgets for Navi they are written in their shell scripting language
because they intend to integrate Navi with the shell in question (Fish, Zsh, NuShell, PowerShell, etc.).

We separate Navi into two categories:

- `Navi Core` which refers to Navi's code in Rust
- `Navi Widgets` which refers to code that intends to integrate Navi with a 3rd-party software

## Contribute to Navi Core

If you want to contribute to Navi Core there are certain steps you need to follow for
your changes to be accepted.

1. First, open an issue if no opened issues are related to the change you want to contribute.
2. [Optional] Wait to have an opinion from the maintainers, developers or contributors from Navi.

   > This step is marked as *Optional* as you can open a Merge Request (MR)/Pull Request (PR)  
   > without having to open an issue beforehand, although it is recommended to not do so.

   We ask you to wait before working on a PR as the way you see a feature and its implementation
   might not be similar on how a maintainer of Navi sees it.

   This will save you and the maintainers time.

3. Fork the repository and iterate over your changes.
4. Update Navi documentation

    If you implement a new feature, you will need to create a new entry in the project's
    documentation for users to know what has changed.

    No significant modification in Navi's behaviour should be merged without being documented.\
    For more details I recommend you to see [contributions/documentation/](../documentation/README.md).

5. Open a PR on [denisidoro/navi](https://github.com/denisidoro/navi/pulls) and request a review
6. [Optional] Your PR needs revisions and changes before it can be merged

    It's not rare that your PR will need changes before it can be accepted by the maintainers
    and then merged into the main branch.

7. Your PR has been merged    

    Congratulations! Your PR has been reviewed and merged, you should be proud of it,
    and we thank you for your contribution.

    The next release cycle will package all contributions into a new release and users
    throughout the world will be able to use your new feature(s).
