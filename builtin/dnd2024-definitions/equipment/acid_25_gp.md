---
splat_schema: 1
splat_id: 3c0faf22-8ca2-5423-a190-b444a5dfc8b9
splat_kind: item
splat_key: dnd2024:equipment/acid_25_gp
splat_name: Acid (25 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Acid (25 GP)

## Source text

<!-- SRD page 94 -->

When you take the Attack action, you can replace\
one of your attacks with throwing a vial of Acid.\
Target one creature or object you can see within\
20 feet of yourself. The target must succeed on a\
Dexterity saving throw (DC 8 plus your Dexterity\
modifier and Proficiency Bonus) or take 2d6 Acid\
damage.\

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
| ` mechanic ` | ` dnd2024:equipment/acid_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_acid_25_gp ` |

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
  - object
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
          sides: 6
        bonus: 0
      type: acid
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |
