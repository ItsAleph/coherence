# API

Initializing working space:

- [x] `chr init` - Initialize Coherence repository
- - [x] `--path <PathBuf>` - path to initialize repository in
- [ ] `chr clone <URL>` - Clone remote repository
- - [ ] `--path` - path to clone repository into

Managing tracking tree:

- [ ] `chr add paths=<PathBuf..>` - Set paths to be tracked by Coherence
- [ ] `chr tree` - Print tree of currently tracked paths
- - [ ] `--branch name=<String>` - the branch to show tree for
- [ ] `chr move` - Move paths eliminating possible ambiguity
- [ ] `chr remove` - Remove paths eliminating possible ambiguity
<!-- - [ ] `chr sync` - Syncronise working tree with proto -->

Managing working tree:

- [ ] `chr reset commit=<ID> files=<PathBuf..>` - Reset paths to their commit state

Working with commits:

- [ ] `chr apply ref=<URL|PathBuf> id=<ID>` - Apply foreign commit to current proto
- [ ] `chr commit` - Compare working tree with proto and record output changes as a commit
- - [ ] `-m <String>` - commit message (description)
- - [ ] `--allow-empty <bool>` - allow creating empty commits
- - [ ] `--allow-empty-message <bool>` - allow creating commits with empty messages
- [ ] `chr revert commit=<ID>` - Create a commit that does everything the specified commit did in reverse.
                                 Note that this is recursive, meaning that all dependencies between HEAD
                                 and the specified commit will be undone too.
- - [ ] `--no-commit` - do not create commit after performing actions.
                        Currently only usable with `--working`.
- - [ ] `--working <bool>` - perform actions on working tree rather than on proto.
                             Implies `--no-commit`.
- [ ] `chr history` - View commit history

Working with branches:

- [ ] `chr branch` - Shortcut for `chr branch list`
- [ ] `chr branch list` - Lists all local branches
- [ ] `chr branch create name=<String>`
- - [ ] `--from base=<String>`
- [ ] `chr branch rename old=<String> new=<String>`
- [ ] `chr branch delete name=<String>`

Utilities:

- [ ] `chr archive` - Create an archive from current tracking tree
- - [ ] `-o | --output path=<PathBuf>`
- - [ ] `--command cmd=<String>`
- [ ] `chr diff file1=<[branch=<String>:]PathBuf> file2=<[branch=<String>:]PathBuf>`

- [ ] `chr branch`
- [ ] `chr branch list`
- [ ] `chr branch create name=<String>`
- - [ ] `--from base=<String>`
- [ ] `chr branch rename old=<String> new=<String>`
- [ ] `chr branch delete name=<String>`

- [ ] `chr push`