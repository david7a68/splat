---
splat_schema: 2
splat_id: 066b3f45-a0d3-5d4a-95e8-a3c07f1070e9
splat_kind: spell
splat_key: dnd2024:spells/bane
splat_name: Bane
splat_origin: dnd2024:reference/spell/bane
splat_sources:
- SRD 5.2.1, pp. 112, https://www.dndbeyond.com/srd
---

# Bane


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Enchantment |
| Classes | Bard, Cleric, Warlock |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S, M (a drop of blood) |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 112 -->

Level 1 Enchantment (Bard, Cleric, Warlock)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a drop of blood)\
Duration: Concentration, up to 1 minute

Up to three creatures of your choice that you can see within range must each make a Charisma saving throw. Whenever a target that fails this save makes an attack roll or a saving throw before the spell ends, the target must subtract 1d4 from the attack roll or save.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/112|page 112]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/bane ` |
| ` implementation ` | ` dnd2024:spell_bane ` |

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
visible: true
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: charisma
dc:
  binding: save_dc
failure:
  roll_modifier:
    rolls:
    - attack
    - save
    amount:
      dice:
        count: 1
        sides: 4
        add: 0
    operation: subtract
    duration: spell_duration
success:
  effect: none
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

