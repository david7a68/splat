---
splat_schema: 1
splat_id: 7b7dd526-12b4-5cff-94fd-749cd6d6a42e
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_1_primal_order
splat_name: 'Level 1: Primal Order'
splat_origin:
  key: dnd2024:reference/pages/042
splat_sources:
- title: SRD 5.2.1
  locator: pp. 42
  url: https://www.dndbeyond.com/srd
---

# Level 1: Primal Order

## Source text

<!-- SRD page 42 -->

You have dedicated yourself to one of the following\
sacred roles of your choice.\
Magician. You know one extra cantrip from the\
Druid spell list. In addition, your mystical connec-\
tion to nature gives you a bonus to your Intelligence\
(Arcana or Nature) checks. The bonus equals your\
Wisdom modifier (minimum bonus of +1).\
Warden. Trained for battle, you gain proficiency\
with Martial weapons and training with Medium\
armor.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_1_primal_order ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_1_primal_order ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
magician:
  extra_cantrip:
    count: 1
    list: druid
  ability_check_bonus:
    ability: intelligence
    skills:
    - arcana
    - nature
    amount:
      op: max
      args:
      - 1
      - var: actor.abilities.wisdom.modifier
warden:
  weapon_proficiencies:
  - martial
  armor_training:
  - medium
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` minimum_level ` | ` 1 ` |
