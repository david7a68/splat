---
splat_schema: 1
splat_id: a0d4116d-36c2-52de-af3b-ce5dfa01e7c1
splat_kind: item
splat_key: dnd2024:equipment/holy_water_25_gp
splat_name: Holy Water (25 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Holy Water (25 GP)

## Source text

<!-- SRD page 97 -->

When you take the Attack action, you can replace one of your attacks with throwing a flask of Holy Water. Target one creature you can see within 20 feet of yourself. The target must succeed on a Dexterity saving throw (DC 8 plus your Dexterity modifier and Proficiency Bonus) or take 2d8 Radiant damage if it is a Fiend or an Undead.

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 25 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/holy_water_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_holy_water_25_gp ` |

### Parameters

#### Use

<!-- splat:yaml path=splat_rule/parameters/use -->

```yaml
economy: replace_one_attack
action: attack
consume: 1
target:
  count: 1
  kinds:
  - creature
  visible: true
  range_ft: 20
save:
  ability: dexterity
  dc:
    op: add
    operands:
    - 8
    - ability_modifier: dexterity
    - stat: proficiency_bonus
  success: no_effect
  failure:
    damage:
      amount:
        dice:
          count: 2
          sides: 8
        bonus: 0
      type: radiant
    only_if_creature_type_in:
    - fiend
    - undead
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |
