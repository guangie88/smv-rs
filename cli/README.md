# `smv-cli`

CLI for SemVer manipulation.

If you want to transform extract / format SemVer values, such as for Docker
image tagging, then this is for you.

## Usages

### Extraction

Direct:

```bash
SEMVER=3.1.4
smv parse ${SEMVER} x.y # 3.1
smv parse -n ${SEMVER} x.y # 3.1 with newline at the end
```

From `stdin`:

```bash
SEMVER=3.1.4
echo ${SEMVER} | smv parse - x.y # 3.1
echo ${SEMVER} | smv parse -n - x.y # 3.1 with newline at the end
```

### Formatting

Simple with escape (escape char is `'\'`):

```bash
SEMVER=3.1.4
smv parse -n ${SEMVER} "\\x:x \\y:y \\z:z" # x:3 y:1 z:4
```

More textual:

```bash
SEMVER=3.1.4
smv parse -n ${SEMVER} "major: x, minor: y, patch: z"
# major: 3, minor: 1, patch: 4
```

For extra help, enter: `smv parse --help`.

## Limitations

Currently only works for values of format `x.y.z`.
