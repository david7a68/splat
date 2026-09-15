---
splat_schema: 1
splat_id: a3ca0881-5df9-56b7-866e-7a7ac7f348f9
splat_kind: spell
splat_key: dnd2024:spells/arcane_eye
splat_name: Arcane Eye
splat_origin:
  key: dnd2024:reference/spell/arcane_eye
splat_sources:
- title: SRD 5.2.1
  locator: pp. 110
  url: https://www.dndbeyond.com/srd
---

# Arcane Eye

## Source text

<!-- SRD page 110 -->

Level 4 Divination (Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a bit of bat fur)\
Duration: Concentration, up to 1 hour\
You create an Invisible, invulnerable eye within\
range that hovers for the duration. You mentally\
receive visual information from the eye, which can\
see in every direction. It also has Darkvision with a\
range of 30 feet.\
As a Bonus Action, you can move the eye up to 30\
feet in any direction. A solid barrier blocks the eye’s\
movement, but the eye can pass through an opening\
as small as 1 inch in diameter.\

Original layout: [[../../srd-5.2.1/pages/110|page 110]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
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
| ` description ` | ` a bit of bat fur ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/arcane_eye ` |
| ` implementation ` | ` dnd2024:spell_arcane_eye ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` location ` |
| ` within_range ` | ` true ` |

#### Sensor

<!-- splat:values path=splat_rule/parameters/sensor -->

| Field | Value |
| --- | --- |
| ` invisible ` | ` true ` |
| ` invulnerable ` | ` true ` |
| ` hovers ` | ` true ` |
| ` perception ` | ` visual ` |
| ` transmission ` | ` mental_to_caster ` |
| ` directions ` | ` all ` |
| ` darkvision_feet ` | ` 30 ` |
| ` duration ` | ` spell_duration ` |

#### Move

<!-- splat:values path=splat_rule/parameters/move -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` max_feet ` | ` 30 ` |
| ` direction ` | ` any ` |
| ` solid_barrier_blocks ` | ` true ` |
| ` minimum_opening_diameter_inches ` | ` 1 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: hour
```
