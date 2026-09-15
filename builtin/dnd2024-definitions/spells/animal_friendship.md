---
splat_schema: 1
splat_id: d4c2380e-ac78-5579-8af5-fd603381dca7
splat_kind: spell
splat_key: dnd2024:spells/animal_friendship
splat_name: Animal Friendship
splat_origin:
  key: dnd2024:reference/spell/animal_friendship
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107, 108
  url: https://www.dndbeyond.com/srd
---

# Animal Friendship

## Source text

<!-- SRD page 107 -->

Level 1 Enchantment (Bard, Druid, Ranger)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a morsel of food)\
Duration: 24 hours\
Target a Beast that you can see within range. The\
target must succeed on a Wisdom saving throw or\
have the Charmed condition for the duration. If you\

<!-- SRD page 108 -->

or one of your allies deals damage to the target, the\
spells ends.\
Using a Higher-Level Spell Slot. You can target\
one additional Beast for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/107|page 107]], [[../../srd-5.2.1/pages/108|page 108]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 24 hours ` |
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
| ` description ` | ` a morsel of food ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/animal_friendship ` |
| ` implementation ` | ` dnd2024:spell_animal_friendship ` |

### Parameters

#### Targeting

<!-- splat:yaml path=splat_rule/parameters/targeting -->

```yaml
kind: creature
creature_types:
- beast
visible: true
count:
  base: 1
  per_slot_above: 1
  slot_base: 1
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
failure:
  condition: charmed
  source: caster
  duration: spell_duration
```

#### Termination

<!-- splat:values path=splat_rule/parameters/termination -->

| Field | Value |
| --- | --- |
| ` event ` | ` damage.dealt ` |
| ` dealer ` | ` caster_or_caster_ally ` |
| ` recipient ` | ` target ` |
| ` scope ` | ` spell ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 24
  unit: hour
```
