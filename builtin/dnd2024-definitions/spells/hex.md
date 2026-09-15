---
splat_schema: 1
splat_id: bf2a6ef1-f17f-5c00-b9dd-6620049145bf
splat_kind: spell
splat_key: dnd2024:spells/hex
splat_name: Hex
splat_origin:
  key: dnd2024:reference/spell/hex
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
---

# Hex

## Source text

<!-- SRD page 140 -->

Level 1 Enchantment (Warlock)\
Casting Time: Bonus Action\
Range: 90 feet\
Components: V, S, M (the petrified eye of a newt)\
Duration: Concentration, up to 1 hour\
You place a curse on a creature that you can see\
within range. Until the spell ends, you deal an extra\
1d6 Necrotic damage to the target whenever you hit\
it with an attack roll. Also, choose one ability when\
you cast the spell. The target has Disadvantage on\
ability checks made with the chosen ability.\
If the target drops to 0 Hit Points before this spell\
ends, you can take a Bonus Action on a later turn to\
curse a new creature.\
Using a Higher-Level Spell Slot. Your Concentra-\
tion can last longer with a spell slot of level 2 (up to\
4 hours), 3–4 (up to 8 hours), or 5+ (24 hours).\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["warlock"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 90 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` the petrified eye of a newt ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/hex ` |
| ` implementation ` | ` dnd2024:spell_hex ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Curse

<!-- splat:yaml path=splat_rule/parameters/curse -->

```yaml
duration: spell_duration
additional_damage:
  event: caster_hits_target_with_attack_roll
  dice: 1d6
  type: necrotic
disadvantage:
  rolls: ability_checks
  ability: caster_choice_at_cast
```

#### Transfer

<!-- splat:values path=splat_rule/parameters/transfer -->

| Field | Value |
| --- | --- |
| ` requires ` | ` target_dropped_to_zero_hp_before_spell_ends ` |
| ` economy ` | ` bonus_action ` |
| ` timing ` | ` later_turn ` |
| ` target ` | ` new_creature ` |

#### Slot duration hours

<!-- splat:yaml path=splat_rule/parameters/slot_duration_hours -->

```yaml
'1': 1
'2': 4
'3': 8
'4': 8
'5': 24
'6': 24
'7': 24
'8': 24
'9': 24
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: hour
```
