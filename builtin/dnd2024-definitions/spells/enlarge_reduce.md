---
splat_schema: 1
splat_id: 382204c4-9092-5f17-a08d-029db06492d8
splat_kind: spell
splat_key: dnd2024:spells/enlarge_reduce
splat_name: Enlarge/Reduce
splat_origin:
  key: dnd2024:reference/spell/enlarge_reduce
splat_sources:
- title: SRD 5.2.1
  locator: pp. 127, 128
  url: https://www.dndbeyond.com/srd
---

# Enlarge/Reduce

## Source text

<!-- SRD page 127 -->

Level 2 Transmutation (Bard, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a pinch of powdered iron)\
Duration: Concentration, up to 1 minute\
For the duration, the spell enlarges or reduces a\
creature or an object you can see within range (see\

<!-- SRD page 128 -->

the chosen effect below). A targeted object must\
be neither worn nor carried. If the target is an un-\
willing creature, it can make a Constitution saving\
throw. On a successful save, the spell has no effect.\
Everything that a targeted creature is wearing\
and carrying changes size with it. Any item it drops\
returns to normal size at once. A thrown weapon or\
piece of ammunition returns to normal size imme-\
diately after it hits or misses a target.\
Enlarge. The target’s size increases by one cat-\
egory—from Medium to Large, for example. The\
target also has Advantage on Strength checks and\
Strength saving throws. The target’s attacks with\
its enlarged weapons or Unarmed Strikes deal an\
extra 1d4 damage on a hit.\
Reduce. The target’s size decreases by one cate-\
gory—from Medium to Small, for example. The tar-\
get also has Disadvantage on Strength checks and\
Strength saving throws. The target’s attacks with\
its reduced weapons or Unarmed Strikes deal 1d4\
less damage on a hit (this can’t reduce the damage\
below 1).\

Original layout: [[../../srd-5.2.1/pages/127|page 127]], [[../../srd-5.2.1/pages/128|page 128]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["bard", "druid", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a pinch of powdered iron ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/enlarge_reduce ` |
| ` implementation ` | ` dnd2024:spell_enlarge_reduce ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["enlarge", "reduce"] ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |
| ` object_not_worn_or_carried ` | ` true ` |

#### Unwilling creature

<!-- splat:values path=splat_rule/parameters/unwilling_creature -->

| Field | Value |
| --- | --- |
| ` may_save ` | ` true ` |
| ` ability ` | ` constitution ` |
| ` success ` | ` no_effect ` |

#### Equipment

<!-- splat:values path=splat_rule/parameters/equipment -->

| Field | Value |
| --- | --- |
| ` resize_worn_and_carried ` | ` true ` |
| ` drop ` | ` revert_immediately ` |
| ` thrown_weapon_or_ammunition ` | ` revert_after_hit_or_miss ` |

#### Effects

<!-- splat:yaml path=splat_rule/parameters/effects -->

```yaml
enlarge:
  size_category_delta: 1
  strength_checks: advantage
  strength_saves: advantage
  weapon_and_unarmed_hit_damage_bonus:
    dice:
      count: 1
      sides: 4
      add: 0
reduce:
  size_category_delta: -1
  strength_checks: disadvantage
  strength_saves: disadvantage
  weapon_and_unarmed_hit_damage_subtract:
    dice:
      count: 1
      sides: 4
      add: 0
  minimum_damage: 1
```
