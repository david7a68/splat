---
splat_schema: 1
splat_id: f90e1217-fb74-504e-981f-d809e6574934
splat_kind: spell
splat_key: dnd2024:spells/feather_fall
splat_name: Feather Fall
splat_origin:
  key: dnd2024:reference/spell/feather_fall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 130
  url: https://www.dndbeyond.com/srd
---

# Feather Fall

## Source text

<!-- SRD page 130 -->

Level 1 Transmutation (Bard, Sorcerer, Wizard)\
Casting Time: Reaction, which you take when you or a creature you can see within 60 feet of you falls\
Range: 60 feet\
Components: V, M (a small feather or piece of down)\
Duration: 1 minute

Choose up to five falling creatures within range. A falling creature’s rate of descent slows to 60 feet per round until the spell ends. If a creature lands before the spell ends, the creature takes no damage from the fall, and the spell ends for that creature.

Original layout: [[../../srd-5.2.1/pages/130|page 130]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 minute ` |
| ` casting_time ` | ` Reaction, which you take when you or a creature you can see within 60 feet of you falls ` |
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
| ` description ` | ` a small feather or piece of down ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/feather_fall ` |
| ` implementation ` | ` dnd2024:spell_feather_fall ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` descent_feet_per_round ` | ` 60 ` |

#### Target

<!-- splat:yaml path=splat_rule/parameters/target -->

```yaml
kind: creature
count:
  maximum: 5
visible_to_caster: false
falling: true
```

#### Landing

<!-- splat:values path=splat_rule/parameters/landing -->

| Field | Value |
| --- | --- |
| ` before_spell_end ` | ` true ` |
| ` fall_damage ` | ` 0 ` |
| ` end_spell_on_landed_target ` | ` true ` |
