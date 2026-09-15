---
splat_schema: 1
splat_id: 1d9f348b-1d10-5840-b28b-6160733877bb
splat_kind: spell
splat_key: dnd2024:spells/invisibility
splat_name: Invisibility
splat_origin:
  key: dnd2024:reference/spell/invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
---

# Invisibility

## Source text

<!-- SRD page 143 -->

Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (an eyelash in gum arabic)\
Duration: Concentration, up to 1 hour\
A creature you touch has the Invisible condition un-\
til the spell ends. The spell ends early immediately\
after the target makes an attack roll, deals damage,\
or casts a spell.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 2.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` an eyelash in gum arabic ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/invisibility ` |
| ` implementation ` | ` dnd2024:spell_invisibility ` |

### Parameters

#### Targeting

<!-- splat:yaml path=splat_rule/parameters/targeting -->

```yaml
kind: creatures
touch: true
count:
  base: 1
  per_slot_above: 1
  slot_base: 2
```

#### Condition

<!-- splat:values path=splat_rule/parameters/condition -->

| Field | Value |
| --- | --- |
| ` name ` | ` invisible ` |
| ` duration ` | ` spell_duration ` |

#### Early end

<!-- splat:values path=splat_rule/parameters/early_end -->

| Field | Value |
| --- | --- |
| ` timing ` | ` immediately_after ` |
| ` target_events ` | ` ["attack_roll_made", "damage_dealt", "spell_cast"] ` |
| ` scope ` | ` spell ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: hour
```
