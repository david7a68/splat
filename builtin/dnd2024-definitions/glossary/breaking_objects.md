---
splat_schema: 2
splat_id: 35080ca0-59f9-53a4-999d-fc4b8976915a
splat_kind: rule
splat_key: dnd2024:glossary/breaking_objects
splat_name: Breaking Objects
splat_origin: dnd2024:reference/glossary/breaking_objects
splat_sources:
- SRD 5.2.1, pp. 177, 178, https://www.dndbeyond.com/srd
---

# Breaking Objects

## Source text

<!-- SRD page 177 -->

Objects can be harmed by attacks and by some spells, using the rules below. If an object is exceedingly fragile, the GM may allow a creature to break it automatically with the Attack or Utilize action.

Armor Class. The Object Armor Class table suggests ACs for various substances.

<!-- SRD page 178 -->

**Object Armor Class**

| AC | Substance | AC | Substance |
| --- | --- | --- | --- |
| 11 | Cloth, paper, rope | 19 | Iron, steel |
| 13 | Crystal, glass, ice | 21 | Mithral |
| 15 | Wood | 23 | Adamantine |
| 17 | Stone |  |  |

Hit Points. An object is destroyed when it has 0 Hit Points. The Object Hit Points table suggests Hit Points for fragile and resilient objects that are Large or smaller. To track Hit Points for a Huge or Gargantuan object, divide it into Large or smaller sections, and track each section’s Hit Points separately. The GM determines whether destroying part of an object causes the whole thing to collapse.

**Object Hit Points**

| Size | Fragile | Resilient |
| --- | --- | --- |
| Tiny (bottle, lock) | 2 (1d4) | 5 (2d4) |
| Small (chest, lute) | 3 (1d6) | 10 (3d6) |
| Medium (barrel, chandelier) | 4 (1d8) | 18 (4d8) |
| Large (cart, dining table) | 5 (1d10) | 27 (5d10) |

Damage Types and Objects. Objects have Immunity to Poison and Psychic damage. The GM might decide that some damage types are more or less effective against an object. For example, Bludgeoning damage works well for smashing things but not for cutting. Paper or cloth objects might have Vulnerability to Fire damage.

Damage Threshold. Big objects, such as castle walls, often have extra resilience represented by a damage threshold. See also “Damage Threshold.”

No Ability Scores. An object lacks ability scores unless a rule assigns scores to the object. Without ability scores, an object can’t make ability checks, and it fails all saving throws.

Original layout: [[../../srd-5.2.1/pages/177|page 177]], [[../../srd-5.2.1/pages/178|page 178]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/breaking_objects ` |
| ` implementation ` | ` dnd2024:glossary_breaking_objects ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_sources ` | ` ["attacks", "spells_that_affect_objects"] ` |
| ` destroyed_at_hit_points ` | ` 0 ` |
| ` damage_immunities ` | ` ["poison", "psychic"] ` |

#### Fragile automatic break

<!-- splat:yaml path=splat_rule/parameters/fragile_automatic_break -->

```yaml
eligibility: exceedingly_fragile
approval:
  chooser: gm
  options:
  - true
  - false
cost_options:
- attack_action
- utilize_action
```

#### Suggested ac

<!-- splat:values path=splat_rule/parameters/suggested_ac -->

| Field | Value |
| --- | --- |
| ` cloth ` | ` 11 ` |
| ` paper ` | ` 11 ` |
| ` rope ` | ` 11 ` |
| ` crystal ` | ` 13 ` |
| ` glass ` | ` 13 ` |
| ` ice ` | ` 13 ` |
| ` wood ` | ` 15 ` |
| ` stone ` | ` 17 ` |
| ` iron ` | ` 19 ` |
| ` steel ` | ` 19 ` |
| ` mithral ` | ` 21 ` |
| ` adamantine ` | ` 23 ` |

#### Suggested hit points

<!-- splat:yaml path=splat_rule/parameters/suggested_hit_points -->

```yaml
tiny:
  fragile:
    fixed: 2
    dice: 1d4
  resilient:
    fixed: 5
    dice: 2d4
small:
  fragile:
    fixed: 3
    dice: 1d6
  resilient:
    fixed: 10
    dice: 3d6
medium:
  fragile:
    fixed: 4
    dice: 1d8
  resilient:
    fixed: 18
    dice: 4d8
large:
  fragile:
    fixed: 5
    dice: 1d10
  resilient:
    fixed: 27
    dice: 5d10
```

#### Statistics selection

<!-- splat:values path=splat_rule/parameters/statistics_selection -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` suggested_values_binding ` | ` false ` |
| ` ac_type ` | ` integer ` |
| ` hit_points_type ` | ` integer ` |

#### Huge or gargantuan

<!-- splat:yaml path=splat_rule/parameters/huge_or_gargantuan -->

```yaml
partition_into_sizes:
- tiny
- small
- medium
- large
hit_points_per_section: true
whole_object_collapse_on_section_destruction:
  chooser: gm
  options:
  - true
  - false
```

#### Damage type effectiveness

<!-- splat:yaml path=splat_rule/parameters/damage_type_effectiveness -->

```yaml
chooser: gm
scope: object_and_damage_type
options:
- normal
- immune
- resistant
- vulnerable
example:
  materials:
  - paper
  - cloth
  damage_type: fire
  response: vulnerable
```

#### Damage threshold

<!-- splat:yaml path=splat_rule/parameters/damage_threshold -->

```yaml
allowed: true
typical: big_objects
rule:
  key: dnd2024:glossary/damage_threshold
```

#### Ability scores

<!-- splat:yaml path=splat_rule/parameters/ability_scores -->

```yaml
default: absent
exception: explicit_rule_assignment
without_scores:
  ability_checks_allowed: false
  all_saving_throws: automatic_failure
```
