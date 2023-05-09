# API

Initializing working space:

- [x] `chr init`
- - [x] `--path <PathBuf>`
- [ ] `chr clone <URL>`

Managing tracking tree:

- [ ] `chr add paths=<PathBuf..>`
- [ ] `chr tree`
- - [ ] `--branch name=<String>`
- [ ] `chr tree move`
- [ ] `chr tree remove`

Managing working tree:

- [ ] `chr reset commit=<ID> files=<PathBuf..>`

Working with commits:

- [ ] `chr apply ref=<URL|PathBuf> id=<ID>`
- [ ] `chr commit`
- - [ ] `-m <String>`
- - [ ] `--allow-empty <bool>`
- - [ ] `--allow-empty-message <bool>`
- [ ] `chr revert commit=<ID>`
- - [ ] `--silent <bool>`
- [ ] `chr history`

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