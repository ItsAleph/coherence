def parse_file(rules):
    header = Header(
        username = rules.next(),
        email = rules.next(),
        date = rules.next(),
        message = rules.next(),
    )

    return File(
        header = header,
        patches = parse_patches(rules),
    )

def parse_patches(rules):
    patches = []

    while patch := rules.next():
        match patch.as_rule():
            case CreatePatch | DeletePatch:
                f = parse_simple_patch
            case UpdatePatch:
                f = parse_update_patch
        patches.append(f(patch))

    return patches

def parse_simple_patch(patch):
    rules = patch.into_inner()

    header = SimpleHeader(
        path = rules.next(),
    )

    additions = []

    while True:
        line = rules.next()

        if line.as_rule() is not Rest:
            break

        additions.append(line.as_str())

    return SimplePatch(
        header = header,
        additions = additions,
    )

def parse_update_patch(patch):
    rules = patch.into()

    header = SimpleHeader(
        path = rules.next(),
    )

    additions = []
