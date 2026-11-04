---
splat_schema: 2
splat_id: c13c443b-6460-53bf-bc48-acdb876af0d9
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_1_divine_order
splat_name: 'Level 1: Divine Order'
splat_origin: dnd2024:reference/pages/037
splat_sources:
- SRD 5.2.1, pp. 37, https://www.dndbeyond.com/srd
---

# Level 1: Divine Order

## Source text

<!-- SRD page 37 -->

You have dedicated yourself to one of the following sacred roles of your choice.

Protector. Trained for battle, you gain proficiency with Martial weapons and training with Heavy armor.

Thaumaturge. You know one extra cantrip from the Cleric spell list. In addition, your mystical connection to the divine gives you a bonus to your Intelligence (Arcana or Religion) checks. The bonus equals your Wisdom modifier (minimum of +1).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_1_divine_order ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_1_divine_order ` |

### Parameters

#### Choice

<!-- splat:values path=splat_rule/parameters/choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/choice/options
key: Variant
order:
- protector
- thaumaturge
columns:
  Variant:
    path: id
    type: symbol
  Weapon proficiencies:
    path: weapon_proficiencies
    type: yaml
    optional: true
  Armor training:
    path: armor_training
    type: yaml
    optional: true
  Cantrip choice list:
    path: cantrip_choice/list
    type: symbol
    optional: true
  Cantrip choice count:
    path: cantrip_choice/count
    type: integer
    optional: true
  Check bonus ability:
    path: check_bonus/ability
    type: symbol
    optional: true
  Check bonus skills:
    path: check_bonus/skills
    type: yaml
    optional: true
  Check bonus amount op:
    path: check_bonus/amount/op
    type: symbol
    optional: true
  Details:
    type: section
    optional: true
-->

| Variant | Weapon proficiencies | Armor training | Cantrip choice list | Cantrip choice count | Check bonus ability | Check bonus skills | Check bonus amount op | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` Protector ` | ` ["martial"] ` | ` ["heavy"] ` |  |  |  |  |  |  |
| ` Thaumaturge ` |  |  | ` Cleric ` | ` 1 ` | ` Intelligence ` | ` ["arcana", "religion"] ` | ` Max ` | [Rules](#^splat-b0a614eb-thaumaturge "section:thaumaturge") |

###### Thaumaturge

^splat-b0a614eb-thaumaturge

###### Args

<!-- splat:yaml path=splat_rule/parameters/choice/options/@thaumaturge/check_bonus/amount/args -->

```yaml
- 1
- stat: abilities.wisdom.modifier
```
