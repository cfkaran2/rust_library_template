# Contributing

We welcome any and all contributions to this project!  Use this document as a
guide to the processes we follow for all contributions, including our own
internal ones. While we may look at contributions that don't follow these
guidelines, it is likely that we'll push those contributions further down the
priority queue of what we review.  In the end, we may require you to follow
these guidelines for us to consider your contribution, so following them is
good practice regardless.

Note that we have a [code of conduct](./CODE_OF_CONDUCT.md), please follow it in
all your interactions with everyone involved in this project.

# Community and Communications

At the current time, all community communications are via the issue tracker.  As
the project grows, and if it appears that the need is there, we can explore
alternative methods of communicating (open an issue if you think this is
important).

# Getting Started

Unsure where to begin contributing to the project? You can start by looking
through the issues for beginner and help wanted issues.

- **Beginner issues** - issues which should only require a few lines of code,
  and a test or two.  If it's a new feature, then it may require additional
  documentation.
- **Help wanted issues** - These tend to be more involved than beginner issues,
  and may require collaboration with the project's maintainers to successfully
  complete.

That said, bugs are a perennial headache, and all help in tracking them down is
appreciated.  It can also help you learn the code behind the project.  So, run
the tests, and see what breaks.  Alternatively, if you find that all of the
tests pass but there is still a bug, then write a new test for the bug and
contribute that to the project.

If you've looked through the issues and nothing grabs you, maybe some of these
will be of interest to you:

- Code and code review.
- Documentation.
- Developing new tests, and writing new bug reports.
- Feature requests that are within the scope of the project (see the
  [`README.md`](./README.md)).
- Ideas for other improvements.

Basically, anything and everything that is within the scope of the project is
fair game for a contribution!

## If You've Never Contributed To An Open Source Project Before, Read This

Working on your first pull request? You can learn how from this free series,
[Finding ways to contribute to open source on GitHub][l1].

Want something a little more advanced, and not just specific to this project?
Try [First Timers Only][l2], which helps you find issues to work on.

## If You Are Contributing Code

This project's goal is to produce a highly polished **product.** Because of
this, we have some fairly stringent requirements on what you need do to get your
contribution into this project.  The sections below are broken out in a way that
(I hope) makes it easy to quickly learn what you need to do.  Please feel free
to reach out to us if there are any confusing areas; one contribution that we
always need is better documentation, and bug reports that tell us how to write
better docs are part of that!

## Definition of Done

This project uses the [Scrum Guide][l3] to define what "done" means.  For this
project, an increment of code is "done" when all of the following are true:

- You've followed all of the requirements in the [Legal](#legal) section.
- If you're contributing code (including test code, examples, or benchmarks),
  you've followed all of the guidance in the
  [Code Contributions](#code-contributions) and [Documentation](#documentation)
  sections.
- If you're contributing documentation, you've followed the guidance in the
  [Documentation](#documentation) section.
- **Every** commit you made followed the project requirements! See the
  [Commits](#commits) section for more information on this.

If you're contributing bug reports or feature requests, follow the guidance in
the [Bug Report Or Feature Request Contributions](#bug-report-or-feature-request-contributions)
section.

# Legal

All contributions are accepted under the terms in the
[`LICENSE.txt`](./LICENSE.txt) file **only!** When you make a contribution, you
are certifying that you have the necessary rights to make the contribution under
the terms of the license, and that you are licensing all of your contributions
to the project and any users of the project under the terms of the license in
the [`LICENSE.txt`](./LICENSE.txt) file that governs this project.

In particular, you agree to the following, copied from
https://developercertificate.org/, which is henceforth called the Developer
Certificate of Origin (DCO):

```text
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.
1 Letterman Drive
Suite D4700
San Francisco, CA, 94129

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.


Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

# Code Contributions

## Code Contribution Checklist

Use the following as a checklist for what you need to do to get your code into
this project.

1.  Read the project's [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md), and the
    [`CONTRIBUTING.md`](./CONTRIBUTING.md) file (this file) completely.
    Following both increase the chances that a developer or maintainer will
    respond to your pull request.
2.  Create issues for any major changes and enhancements that you wish to make.
    Discuss your planned changes transparently and completely with the
    community first, and get community feedback.
3.  Create your own fork of the project.
4.  Make your changes in your fork.
    - Follow the guidance in the [Commits](#commits) section, **especially** the
      [Signing Your Commits](#signing-your-commits) section!  We're working to
      implement automated checks that will reject your contribution if any of
      your commits aren't properly signed!
5.  Ensure that all of your code meets all requirements in the
    [rust API guidelines checklist][l4].
6.  Ensure that all documentation is up to date per the requirements in the
    [Documentation](#documentation) section of this file.
7.  Write unit tests for all of your code using the guidance in the
    [Testing](#testing) section for what is expected.
8.  Ensure cross-platform compatibility for every change that's accepted; at a
    minimum that means the latest versions of Windows, Mac, Debian & Ubuntu
    Linux.  That said, a major goal of the project is to support all
    [Tier 1][l5] Rust platforms, so if you can verify that your project works on
    all of the current [Tier 1][l5] platforms under the current stable version
    of Rust, that would be great.
    - If you don't have access to some of these systems, you can skip this step;
      our test framework will execute tests on all of these systems anyways, but
      testing on your own first can be a little faster than waiting for the CI
      system to build and test it for you.
9.  If applicable, add examples of how to use your code to the `examples`
    directory, and benchmarks to the `benchmarks` directory.
10. Increase the version numbers in any files to the new version that this pull
    request would represent *as is appropriate*. Some documentation describes
    the version in which a given change occurred, and therefore must not be
    modified. In short, **do not use scripts to bulk update the version
    numbers!** This invariably causes issues somewhere along the way, even with
    reviewers checking each change. The versioning scheme we use is
    [SemVer][l6].
11. If the change is user-facing and sufficiently important, update the
    [`README.md`](./README.md). This includes new environment variables, exposed
    ports, useful file locations, etc. Be aware that security conscious users
    will use this information when updating their firewall and other security
    settings, so if you forget to update this information it will cause
    complaints and confusion.  **DO NOT MANUALLY MODIFY THE CHANGELOG.md
    FILE!** Our automated build process uses [`git-cliff`][l7] to update the
    `CHANGELOG.md` file automatically.  If you manually update it, then your
    update will be overwritten the next time the build system runs. Instead,
    make sure that you are following the [Conventional Commits][l8] standard,
    and using the types and scopes described in the
    [How to Write Commit Messages](#how-to-write-commit-messages) section for
    all of your commits.
12. If you like the change and think the project could use it:
    - Run `cargo +nightly fmt` on your fork.
      - The project already has a `.rustfmt.toml` file at its root, which will
        ensure that you are following the formatting guidelines.
    - Run `cargo +nightly clippy`, and where the suggestions make sense, make
      changes.  **NOTE:** not all of clippy's suggestions make sense!  If you
      need to ignore a clippy lint, go ahead and do so, *but let us know that
      you're doing that in the PR!*  Clippy sometimes makes mistakes, so
      sometimes it's necessary to bypass the lints it has.
    - Run `cargo clean` and then `cargo +nightly test --workspace --release`, as
      well as `cargo test --workspace --release`.  We actually test under the
      `stable`, `beta`, and `nightly` channels as we sometimes catch more bugs
      that way, but for most development `stable` and `nightly` are good
      enough.  Please clean your `target` directory between the tests to ensure
      that you don't 'get lucky' when building, and end up linking old build
      products.
    - Also ensure that all examples and benchmarks continue to run correctly.
      This means *all* of the examples and benchmarks, not the just the ones
      that you've added!  There have been cases where new features or fixes
      break old examples or benchmarks, so we need to test to make sure that
      won't happen.
13. Ensure any install or build products are removed before you make your pull
    request.
14. Make your pull request, and get someone to review it and concur that it
    should be merged in.

If you think a pull request has been sitting in the queue for too long without
getting a response, please ping us (literally, all you have to do is type the
word *ping* in the comment section of the pull request, and it'll pop up
another email for someone to read).  We might have to put off your pull request
for a bit while we deal with other priorities, but we'll let you know that, and
when we think we'll get back to your pull request.

## Testing

There are at least two purposes for testing:

- To find bugs while you are writing the code.
- To isolate bugs while you are in the field.

This section only describes the former, because while the kinds of testing
described in this section may help with the latter, it's usually up to the
engineer in the field to develop an ad-hoc test to try to debug a system on the
fly; unit, integration, etc., tests aren't really designed for that purpose. If
it is possible to develop tools for field testing, then that will be another
crate within this project.

Given that, we're going to describe the types of tests you'll likely be writing
in the order that you'll probably be writing them.  In most projects you design
the project from the top down, moving from the solution as a whole to each
part, then each subpart etc.  Once you're down to a small enough part that
you're ready to code it up, you're also at the level where you can write unit
tests to verify that chunk works.  How to write unit tests for this project is
briefly described in the [Unit Testing](#unit-testing) section.

As you combine subparts into parts, and then into the solution as a whole, you
will start to transition away from [unit testing](#unit-testing) to higher-level
[Integration Testing](#integration-testing).  In rust, the difference is that
unit tests are written within the same module as what is being tested; this
gives unit tests access to private information within the given module.
[Integration testing](#integration-testing) simulates what your user will
actually see; they won't have access to private members of the crate.  This
makes them ideal for ['dogfooding'][l9] your code.  Writing examples and
benchmarks are further examples of ['dogfooding'][l9] your code.

How you should do each of these types of tests are described in the sections
below.

### Unit Testing

Unit testing is the lowest-level form of testing.  All types *must* implement
the [`Semiarbitrary`][l10] trait. This allows us to dog food our internal fuzzer
implementation, as well as test your type.

All unit tests will need to use the crate's internal fuzzer implementation. This
is required for two reasons; first, it helps to test this crate's fuzzer, and
second, the internal fuzzer's properties are well-understood by the maintainers
of this crate.  This lets us find bugs within the crate more quickly than might
otherwise be possible.

### Integration Testing

Integration tests follow the Rust standard and are installed in the `tests`
directory of the workspace/crate they are a part of.  These tests check the
public APIs.  At the crate level, they should test crate functionality only.
Functionality that crosses crates should be placed in the workspace-level
`tests` directory.

# Documentation

Documentation is prose that helps the reader understand how to use your code,
and why they should use it in that way.  The most important principle is the
following one:

> Is the language clear?  Can a reader really understand what you're trying to
> say?

The rest of the instructions in here exist to expand upon that principle.  While
you should follow the guidelines here, there are always corner cases where it
won't make sense to do so.  In general, err on the side of more documentation;
it rarely hurts, and often helps.

## How to Write Documentation

This section is partially adapted from the [Rust API guidelines][l4].  The
particular flavor of markdown used is described in the [rustdoc book][l11].

Documentation covers a great deal of ground, starting from the overall design
and moving all the way down to the individual functions, methods, and
constants. The purpose of documentation is to convey all of the following:

- What is the issue that is being solved by this level of documentation?
- How is the covered chunk of code solving the issue?
    - This includes what is expected of the user when using this chunk of code.
    - If applicable, what is out of scope for this chunk, and if it isn't
      obvious, *why* that stuff is out of scope.
- Examples of how to use the code.
- Known issues that need to be solved at a future date.
    - If something is incomplete, you *must* use a documentation comment as
      close to the location as possible with the exact phrase `TODO:` followed
      by a single space, and then as much plain text as you need to explain what
      is incomplete.  Where it makes sense, use the `todo!()` macro with an
      explanation of what is missing.
    - If something is broken, you *must* use a documentation comment as close to
      the location as possible with the exact phrase `FIXME:` followed by a
      single space, and then as much plain text as you need to explain what is
      broken.  Where it makes sense, use may wish to use `panic!()`, `abort!()`,
      or even `compile_error!()` to cause a compilation to fail.

The reason for the phrases `TODO:` and `FIXME:` are simple: when you compile the
code, the compiler will alert you that there things to fix.  This can be easier
to deal with than creating an issue in a bug tracker for it (although you really
*should* create an issue in the bug tracker for it, that's what it's there for).

The following subsections explain how this is applied within the code's
hierarchy of concerns.  I use terminology that is specific to Rust, but should
be easy to understand even if you aren't familiar with Rust.

### Workspace Level

At the workspace level, the documentation explains the problems the project is
trying to solve, what the objectives are, and what the definition of done is.
It also has a high-level plan of what will be done, and what each crate is
individually responsible for.  This document has enough detail that anyone
reading it knows what to expect from the project as a whole, but the fine
detail of how each individual crate operates is reserved for that crate's own
documentation. There is also an overall explanation of the general architecture
and of concerns that cross crate boundaries.

At the workspace level, supply complete examples of how to use the project's
code.  These must be placed in the `examples` directory.  The examples must be
sufficiently well documented that they can be copied verbatim into a user's
crate, and then modified to work with their code.  Ideally, the examples can
double as templates that others can adapt to their own uses, so they should be
defined with that in mind.

### Crate Level

Crates are concerned with well-separated areas of functionality.  The
documentation for those crates should try to remain focused on the contents of
the crate, and not on the contents of the workspace or other crates. That said,
crate level documentation may reference documentation outside of the crate, but
not repeat it ([DRY principle][l12].

At the crate level, we see code examples within the documentation itself.  These
are generally small, self-contained chunks of code that illustrate what problems
the crate is solving.  They shouldn't reference code outside of the crate, but
that is a guideline, not a fast and hard rule.  Any code should be able to pass
standard doc tests; if they fail, it is a sign that the documentation needs to
be updated.

If a crate could be useful by itself, then you should supply stand-alone
examples of how to use it in the `examples` directory in that crate.  Treat
examples as potential templates and document what users can replace to make
their own code work.

### Module Level and Below

At the module level and below, every module, trait, struct, enum, method, macro,
type definition, and constant must be fully documented, which includes a
complete code example of how to use the item.  The reason for this level of
documentation is for doctests; rust will automatically run the code within every
documentation comment as if it were a test of the code, flagging any that fail.
Doctests that fail indicate that the documentation itself is likely out of date,
which makes doctests a surrogate for verifying that the documentation is
current.

Functions, methods and macros require documentation on error, panic, and where
needed, safety conditions.  For example:

```rust
/// One line summary
///
/// Multi-paragraph discussion of the function.  This includes the following
/// information:
///
/// - What problem the function solves.
/// - What preconditions (if any) the function expects.
/// - What postconditions (if any) the function guarantees on completion.
///
/// ## Example
///
/// Within this section, provide an example of how to use the function.  This
/// will be a doctest that can be run.  You may choose to give examples that
/// cannot be run as well, but those are optional.
///
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
///       foo(12u8)?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// ## Safety (optional)
///
/// This section is optional, but *must* be in place if this function can lead
/// to unsafety.  All conditions that can lead to unsafety *must* be described.
/// How to avoid unsafe behavior when calling the function *must* also be
/// described.
///
/// ## Panics (optional)
///
/// If this function can panic via the 'panic!()' or 'abort!()' macros, the
/// causes *must* be described here, along with how to avoid causing a panic or
/// abort.  Note that certain types of panics and aborts are outside of the
/// control of the function and do not need to be described here.  As an
/// example, it is impossible to know if a process will run out of memory
/// during an allocation because that requires knowledge of what other
/// processes are planning on doing concurrent to this process.  As a rule of
/// thumb, if this function calls `panic!()` or `abort!()` directly, it must be
/// documented. Otherwise, use your best judgement.
///
/// ## Parameters
///
/// If a function accepts arguments, then they *must* be described here.  In
/// general, this should be in the form of a list, similar to the following:
///
/// - `arg` - A description of `arg`.  You do not need to detail what the type
///   is as the compiler will fill that in for you, but you must describe what
///   it is for.  Most importantly, if there are values that can be passed in
///   that are not allowed, you **must** describe them here.  For example, if
///   `arg` must be in the range `[0, 10]` or a panic can occur, then you
///   **must** provide a warning about that here, as well as provide the warning
///   in the `Panics` section above.
/// - `next_arg` - The next argument in the list
/// - `and_so_forth`
///
/// ## Returns
///
/// Provide a complete description of what is returned.  The type information
/// will be picked up by the compiler, but how and why a particular instance is
/// returned is not always obvious from the return type, which is why
/// you **must** explain it here.  In particular, if you return a `Result`,
/// always detail what types of errors may be returned by this function.
///
/// The only case where you don't need the `Returns` section is if nothing (AKA,
/// the `unit` type, `()`) is returned.
///
/// For the function below, we might have documentation similar to the
/// following:
///    If this function operates correctly, it will return the additive inverse
///    of `arg` in the `Ok()` variant.  The additive inverse is the value that
///    makes `(arg + x) % 256 == 0` true (where `x` is the additive inverse).
///
///    This function makes a remote procedure call (RPC) under the hood to a
///    service that may or may not be available at the time of the call.  If
///    the service isn't available (or it has some other kind of error), then
///    the appropriate [`FooError`] type will be returned in the
///    `Err()` variant. Read the documentation for [`FooError`] for more
///    information on what kinds of errors you might need to deal with.
pub fn foo(arg: u8) -> Result<u8, FooError> {
    todo!()
}
```

Methods are similar to the above, except that 'surprising' behavior with the
first parameter must also be documented.  In particular, if the first parameter
is '&self', then the expectation is it is immutable.  If you are using interior
mutability, then this **must** be documented.  You *should* document when
mutations can occur when passing in '&mut self'.  You do not need to document
mutation behavior if the first parameter is 'Self' as the calling code will be
giving up ownership in that case.

Macros should be documented in a manner similar to functions.

There is no need for `examples` or `benchmarks` directories below the crate
level.

# Commits

## What to Commit

In general, if it took human effort to create it, it should be committed.
However, if it is automatically generated by a build process, and will be
recreated the next time the crate is built, then don't commit it.

Note that if you know that something is going to be large and rarely changed,
like binary files, use [`git-lfs`][l13] to store it.

If you're not sure if it should go in the repository, please contact the
maintainers for guidance; we really don't need multi-gigabyte commits in the
repository, it just makes things hard for everyone.

## Signing Your Commits

Contributors must sign-off each commit by adding a `Signed-off-by: ...` line to
commit messages to certify that they have the right to submit the code they are
contributing to the project according to the [DCO][l14], and that they are doing
so under this project's license, which is in the [`LICENSE.txt`] file.

To simplify this process, you may choose to do the following to configure your
git options:

```bash
git config --global trailer.sign.key "Signed-off-by: "
git config --global trailer.sign.ifmissing add
git config --global trailer.sign.ifexists addIfDifferent
git config --global trailer.sign.command 'echo "$(git config user.name) <$(git config user.email)>"'
```

Or, if you only want to sign off on this project, then change directories to the
root of this project and execute:

```bash
git config trailer.sign.key "Signed-off-by: "
git config trailer.sign.ifmissing add
git config trailer.sign.ifexists addIfDifferent
git config trailer.sign.command 'echo "$(git config user.name) <$(git config user.email)>"'
```

You can check that this worked via the following command:

```bash
git interpret-trailers <<EOF
```

At the `>`, type `EOF` and press enter.  The output should be similar to the
following:

```bash
Signed-off-by: John Smith <john.smith@example.com>
```

When you commit, use the `-s` or `--signoff` flag to the command, like so:

```bash
git commit -s
```

The commit message should automatically have the trailer listed above appended
to it.

## How to Write Commit Messages

Commit messages are themselves documentation, and as such we have requirements
for what you put into a commit, and how you write your commit messages.  Most
importantly:

> There **must** be **at most** one new feature (or bug fix) per commit!

It's tempting to roll up a bunch of small fixes into a single commit, but the
problem with this approach is when we're trying to figure out why something
broke; `git bisect` doesn't work as well as it would if there were many smaller
commits that we could work our way through.  Doing so also breaks our
documentation requirements:

> **All** commit messages **must** follow the [Conventional Commits][l8]
> standard!

We limit the types and scopes used in the header lines of the commit messages to
the following:

- `chore` - Used for cleaning up code in preparation for a release.
- `doc` - Used when you've improved or added documentation. Not used for fixes
          to documentation (see the `fix` type for that).
- `feat` - Used when you add a new *complete* code feature.  Basically, `feat`
           is an announcement that you think the feature you've added is now
           ready to be used.
- `fix` - Used when you fix a bug.  This is used for **all** bug fixes, from
          code, to documentation, to build system, etc.  However, it is not used
          when you're creating a new feature unrelated to existing code or
          documentation.  When you use this type, you should use a scope.  You
          may choose any scope you want, but the following are commonly used:
          - `chore` - Used when you were doing a `chore`, broke something, and
            then fixed it.
          - `doc` - Used when you found some broken/bad/unclear documentation
            and fixed it.
          - `test` - Used when you found a broken test and fixed it.
          - Code fixes generally aren't scoped as they are just ordinary fixes.
            As a result, we don't generally use `feat`, `perf`, `refactor`,
            `style`, or `WIP` as scopes because they really don't make sense
            here.
- `perf` - Used when you make a change that improves the performance of the
           code. This shouldn't add any new features, nor is it fixing broken
           code; the changes you made were purely for making the current code
           faster.
- `refactor` - Used when you've moved code or other artifacts around to organize
               it better.
- `style` - Used when you run something like `rustfmt` across the code, or
            otherwise make changes that have no semantic or syntactic meaning,
            and are done just to make the code look pretty.
- `test` - Used when you add a new test.  Note that if you fix a broken test,
           then you should use the `fix` type instead.
- `WIP` - Short for "Work In Progress".  If you make a commit that is incomplete
          for some reason, use this type.  It's most useful for all
          the 'in-between' commits that we need to make as we're working on
          things, as well as for 'backup' commits that we make when we want to
          make sure that we won't lose work for some reason.  If you want to
          use a scope similar to the ones used by the `fix` type, you may, but
          because `WIP` is such an open-ended type you don't need include a
          scope, nor do you need to limit yourself to the scopes in the `fix`
          type.  Whatever you choose to do, please make sure it makes sense to
          others.

If you think other types should be added, please open up an issue to discuss
them first.  Our automated build process uses [`git-cliff`][l7] to update the
`CHANGELOG.md` file, and since it it automated, it can become confused if we
don't follow the proper conventions when creating new types.

# Bug Report Or Feature Request Contributions

1. First, search the issue database to see if the bug or feature was already
   reported.
   - If it was was already reported, see if you can add any new information to
     the existing report.  In general, the more information we have, the more we
     can do to fix the issue.
2. If it wasn't already reported, make a minimal, complete test case that
   illustrates the bug so that we can use it to track down the bug.  Submit it
   with your bug report.
   - The easiest way to submit complete test cases is either as pull requests,
     or by creating a new project whose URL you reference in the bug
     report.
   - **All code must follow the guidelines in this document!** The code you
     write to illustrate your issue may be added to this project's set of test
     cases, which means it needs to follow all of the guidelines here just like
     any other code contribution.
3. Please use the issue templates where possible.  They will help guide you when
   creating a new report.

**This project is not yet security hardened!** As a result, no bug is a security
bug, they are all just bugs.  There are no special mechanisms for reporting
security bugs, they can all be put into the issue tracker.

## Further reading
- https://github.com/Bluegg/bug-reporting-guide
- https://github.com/chase-seibert/blog/blob/master/_posts/2016-02-26-QA-101-How-to-write-a-bug-report.md

[l1]: https://docs.github.com/en/get-started/exploring-projects-on-github/finding-ways-to-contribute-to-open-source-on-github
[l2]: https://www.firsttimersonly.com/
[l3]: https://scrumguides.org/docs/scrumguide/v2020/2020-Scrum-Guide-US.pdf#zoom=100
[l4]: https://rust-lang.github.io/api-guidelines/checklist.html
[l5]: https://doc.rust-lang.org/rustc/platform-support.html
[l6]: http://semver.org/
[l7]: https://crates.io/crates/git-cliff
[l8]: https://www.conventionalcommits.org/
[l9]: https://en.wikipedia.org/wiki/Dogfooding
[l10]: crate::semiarbitrary::Semiarbitrary
[l11]: https://doc.rust-lang.org/rustdoc/index.html
[l12]: https://en.wikipedia.org/wiki/Don%27t_repeat_yourself
[l13]: https://git-lfs.github.com/
[l14]: https://developercertificate.org/
