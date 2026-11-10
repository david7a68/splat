---
splat_schema: 2
splat_id: 3a6f1cf5-22be-59da-82ea-0c3bdcb50da5
splat_kind: spell
splat_key: dnd2024:spells/aid
splat_name: Aid
splat_origin: dnd2024:reference/spell/aid
splat_sources:
- SRD 5.2.1, pp. 107, https://www.dndbeyond.com/srd
---

# Aid


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Abjuration |
| Classes | Bard, Cleric, Druid, Paladin, Ranger |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S, M (a strip of white cloth) |
| Duration | 8 hours |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 107 -->

Level 2 Abjuration (Bard, Cleric, Druid, Paladin, Ranger)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a strip of white cloth)\
Duration: 8 hours

Choose up to three creatures within range. Each target’s Hit Point maximum and current Hit Points increase by 5 for the duration.

Using a Higher-Level Spell Slot. Each target’s Hit Points increase by 5 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/107|page 107]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/aid ` |
| ` implementation ` | ` dnd2024:spell_aid ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 3 ` |
| ` within_spell_range ` | ` true ` |

#### Hit points

<!-- splat:yaml path=splat_rule/parameters/hit_points -->

```yaml
current_increase:
  op: add
  args:
  - 5
  - op: multiply
    args:
    - 5
    - op: subtract
      args:
      - binding: cast_level
      - 2
maximum_increase:
  op: add
  args:
  - 5
  - op: multiply
    args:
    - 5
    - op: subtract
      args:
      - binding: cast_level
      - 2
duration: spell_duration
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

