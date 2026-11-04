---
splat_schema: 2
splat_id: e6dcdfeb-2dbf-5311-90d9-0de9ab82d5f8
splat_kind: spell
splat_key: dnd2024:spells/hold_monster
splat_name: Hold Monster
splat_origin: dnd2024:reference/spell/hold_monster
splat_sources:
- SRD 5.2.1, pp. 140, 141, https://www.dndbeyond.com/srd
---

# Hold Monster


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Enchantment |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 90 feet |
| Components | V, S, M (a straight piece of iron) |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 140 -->

Level 5 Enchantment (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a straight piece of iron)\
Duration: Concentration, up to 1 minute

<!-- SRD page 141 -->

Choose a creature that you can see within range. The target must succeed on a Wisdom saving throw or have the Paralyzed condition for the duration. At the end of each of its turns, the target repeats the save, ending the spell on itself on a success.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 5.

Original layout: [[../../srd-5.2.1/pages/140|page 140]], [[../../srd-5.2.1/pages/141|page 141]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/hold_monster ` |
| ` implementation ` | ` dnd2024:spell_hold_monster ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
visible: true
range_feet: 90
count:
  base: 1
  increment_per_slot_above: 1
  base_slot: 5
```

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` unaffected ` |
| ` failure_condition ` | ` paralyzed ` |

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.end ` |
| ` subject ` | ` target ` |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` end_on_target ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

