---
splat_schema: 1
splat_id: c13c443b-6460-53bf-bc48-acdb876af0d9
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_1_divine_order
splat_name: 'Level 1: Divine Order'
splat_origin:
  key: dnd2024:reference/pages/037
splat_sources:
- title: SRD 5.2.1
  locator: pp. 37
  url: https://www.dndbeyond.com/srd
---

# Level 1: Divine Order

## Source text

<!-- SRD page 37 -->

You have dedicated yourself to one of the following\
sacred roles of your choice.\
Protector. Trained for battle, you gain proficiency\
with Martial weapons and training with Heavy\
armor.\
Thaumaturge. You know one extra cantrip from\
the Cleric spell list. In addition, your mystical con-\
nection to the divine gives you a bonus to your\
Intelligence (Arcana or Religion) checks. The bonus\
equals your Wisdom modifier (minimum of +1).\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_1_divine_order ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_1_divine_order ` |

### Parameters

#### Choice

<!-- splat:yaml path=splat_rule/parameters/choice -->

```yaml
count: 1
options:
- id: protector
  weapon_proficiencies:
  - martial
  armor_training:
  - heavy
- id: thaumaturge
  cantrip_choice:
    list: cleric
    count: 1
  check_bonus:
    ability: intelligence
    skills:
    - arcana
    - religion
    amount:
      op: max
      args:
      - 1
      - stat: abilities.wisdom.modifier
```
