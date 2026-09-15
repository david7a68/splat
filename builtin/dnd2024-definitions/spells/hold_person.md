---
splat_schema: 1
splat_id: ca888eaa-780b-5c8c-95ae-229ca74c404a
splat_kind: spell
splat_key: dnd2024:spells/hold_person
splat_name: Hold Person
splat_origin:
  key: dnd2024:reference/spell/hold_person
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141
  url: https://www.dndbeyond.com/srd
---

# Hold Person

## Source text

<!-- SRD page 141 -->

Level 2 Enchantment (Bard, Cleric, Druid, Sorcerer,\
Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a straight piece of iron)\
Duration: Concentration, up to 1 minute\
Choose a Humanoid that you can see within range.\
The target must succeed on a Wisdom saving throw\
or have the Paralyzed condition for the duration. At\
the end of each of its turns, the target repeats the\
save, ending the spell on itself on a success.\
Using a Higher-Level Spell Slot. You can target\
one additional Humanoid for each spell slot level\
above 2.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "cleric", "druid", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a straight piece of iron ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/hold_person ` |
| ` implementation ` | ` dnd2024:spell_hold_person ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` humanoid ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  condition: paralyzed
  duration: spell
success:
  effect: none
```

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.end ` |
| ` ability ` | ` wisdom ` |
| ` success ` | ` end_spell_on_self ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 2 ` |
| ` additional_targets ` | ` 1 ` |
