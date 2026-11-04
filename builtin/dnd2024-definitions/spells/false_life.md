---
splat_schema: 2
splat_id: 039ce569-5324-5bfe-8b21-b1ae5f50ca85
splat_kind: spell
splat_key: dnd2024:spells/false_life
splat_name: False Life
splat_origin: dnd2024:reference/spell/false_life
splat_sources:
- SRD 5.2.1, pp. 129, https://www.dndbeyond.com/srd
---

# False Life


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Necromancy |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a drop of alcohol) |
| Duration | Instantaneous |


## Source text

<!-- SRD page 129 -->

Level 1 Necromancy (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a drop of alcohol)\
Duration: Instantaneous

You gain 2d4 + 4 Temporary Hit Points.

Using a Higher-Level Spell Slot. You gain 5 additional Temporary Hit Points for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/129|page 129]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/false_life ` |
| ` implementation ` | ` dnd2024:spell_false_life ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |

#### Temporary hp

<!-- splat:yaml path=splat_rule/parameters/temporary_hp -->

```yaml
dice:
  count: 2
  sides: 4
  add:
    op: add
    args:
    - 4
    - op: multiply
      args:
      - 5
      - op: subtract
        args:
        - binding: cast_level
        - 1
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

