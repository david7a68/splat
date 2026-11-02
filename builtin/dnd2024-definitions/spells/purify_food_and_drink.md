---
splat_schema: 1
splat_id: eb1d3055-55bf-59f6-838b-15c1b6a3fe18
splat_kind: spell
splat_key: dnd2024:spells/purify_food_and_drink
splat_name: Purify Food and Drink
splat_origin:
  key: dnd2024:reference/spell/purify_food_and_drink
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
---

# Purify Food and Drink

## Source text

<!-- SRD page 157 -->

Level 1 Transmutation (Cleric, Druid, Paladin)\
Casting Time: Action or Ritual\
Range: 10 feet\
Components: V, S\
Duration: Instantaneous

You remove poison and rot from nonmagical food and drink in a 5-foot-radius Sphere centered on a point within range.

Original layout: [[../../srd-5.2.1/pages/157|page 157]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["cleric", "druid", "paladin"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 10 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/purify_food_and_drink ` |
| ` implementation ` | ` dnd2024:spell_purify_food_and_drink ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` remove ` | ` ["poison", "rot"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 5 ` |

#### Eligible

<!-- splat:values path=splat_rule/parameters/eligible -->

| Field | Value |
| --- | --- |
| ` kinds ` | ` ["food", "drink"] ` |
| ` nonmagical ` | ` true ` |
