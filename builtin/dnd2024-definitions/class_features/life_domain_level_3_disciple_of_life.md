---
splat_schema: 1
splat_id: 15d20d39-9010-5ff8-bb78-716c6e07bef5
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_3_disciple_of_life
splat_name: 'Level 3: Disciple of Life'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
---

# Level 3: Disciple of Life

## Source text

<!-- SRD page 40 -->

When a spell you cast with a spell slot restores Hit Points to a creature, that creature regains additional Hit Points on the turn you cast the spell. The additional Hit Points equal 2 plus the spell slot’s level.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/life_domain_level_3_disciple_of_life ` |
| ` implementation ` | ` dnd2024:class_features_life_domain_level_3_disciple_of_life ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` spell.healing ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` caster ` | ` actor ` |
| ` expended_spell_slot ` | ` true ` |
| ` turn ` | ` spell.cast_turn ` |

#### Additional healing

<!-- splat:yaml path=splat_rule/parameters/additional_healing -->

```yaml
op: add
args:
- 2
- var: spell.slot_level
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` minimum_level ` | ` 3 ` |
