---
splat_schema: 2
splat_id: 20599c22-b59a-5f0f-935c-d7703c8a4d69
splat_kind: spell
splat_key: dnd2024:spells/bless
splat_name: Bless
splat_origin: dnd2024:reference/spell/bless
splat_sources:
- SRD 5.2.1, pp. 113, https://www.dndbeyond.com/srd
---

# Bless


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Enchantment |
| Classes | Cleric, Paladin |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S, M (a Holy Symbol worth 5+ GP) |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 113 -->

Level 1 Enchantment (Cleric, Paladin)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a Holy Symbol worth 5+ GP)\
Duration: Concentration, up to 1 minute

You bless up to three creatures within range. Whenever a target makes an attack roll or a saving throw before the spell ends, the target adds 1d4 to the attack roll or save.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/113|page 113]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/bless ` |
| ` implementation ` | ` dnd2024:spell_bless ` |

### Parameters

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
maximum:
  op: add
  args:
  - 3
  - op: multiply
    args:
    - 1
    - op: subtract
      args:
      - binding: cast_level
      - 1
within_spell_range: true
```

#### Roll modifier

<!-- splat:yaml path=splat_rule/parameters/roll_modifier -->

```yaml
rolls:
- attack
- save
amount:
  dice:
    count: 1
    sides: 4
    add: 0
operation: add
duration: spell_duration
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

