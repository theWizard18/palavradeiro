# Palavradeiro

Palavradeiro is a word generator for conlangs, configured in a yaml file.

## Configuration

The configuration file should be either in the current directory, in 
in the home directory or in `~/.config/palavradeiro/palavradeiro.yaml`.

the default configuration:

```yaml
phonemes:
  V: [a, e, i, o, u]
  C: [m, n, p, t, k, s, w, l, j]
  N: [m, n]
phonotactics:
- !Maybe C
- !One V
- !Maybe N
max_syllables: 3
word_quantity: 100
filters: []
separate_syllables: false
```

`phonemes` is a map where each key is a char and the values are lists of strings.

`phonotactics` is a list of values of the enum `Tactic` whose values are:

- `Maybe` which represents a possible phoneme of a group.
- `One` which is an obligatory phoneme of a group.
- `Multiple` which chooses one possible phoneme of a list of groups.

`max_syllables` is the max amount of syllables per word.

`word_quantity` how many words will be generated.

`filters` is a list of strings, filters out all generated words which contains any substring inside of it.

`separate_syllables` bool, determines if the words will have its syllables separated by hyphens.
