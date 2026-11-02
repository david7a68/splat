---
splat_schema: 1
splat_id: b9c3b1bf-e82e-5824-a369-f3aaa80cda67
splat_kind: item
splat_key: dnd2024:equipment/alchemist_s_fire_50_gp
splat_name: Alchemist’s Fire (50 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Alchemist’s Fire (50 GP)

## Source text

<!-- SRD page 94 -->

When you take the Attack action, you can replace one of your attacks with throwing a flask of Alchemist’s Fire. Target one creature or object you can see within 20 feet of yourself. The target must succeed on a Dexterity saving throw (DC 8 plus your Dexterity modifier and Proficiency Bonus) or take 1d4 Fire damage and start burning (see “Rules Glossary”).

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
| ` value ` | ` 50 ` |
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
| ` mechanic ` | ` dnd2024:equipment/alchemist_s_fire_50_gp ` |
| ` implementation ` | ` dnd2024:equipment_alchemist_s_fire_50_gp ` |

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
          count: 1
          sides: 4
        bonus: 0
      type: fire
    apply_rule:
      key: dnd2024:glossary/burning_hazard
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |
