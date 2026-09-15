---
splat_schema: 1
splat_id: 7007a918-dc31-52e9-99b1-d6aa4f2bf858
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_7_elemental_fury
splat_name: 'Level 7: Elemental Fury'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
---

# Level 7: Elemental Fury

## Source text

<!-- SRD page 43 -->

The might of the elements flows through you. You\
gain one of the following options of your choice.\
Potent Spellcasting. Add your Wisdom modifier\
to the damage you deal with any Druid cantrip.\
Primal Strike. Once on each of your turns when\
you hit a creature with an attack roll using a\
weapon or a Beast form’s attack in Wild Shape, you\
can cause the target to take an extra 1d8 Cold, Fire,\
Lightning, or Thunder damage (choose when you\
hit).\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_7_elemental_fury ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_7_elemental_fury ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
potent_spellcasting:
  spell_list: druid
  spell_level: 0
  damage_bonus:
    var: actor.abilities.wisdom.modifier
primal_strike:
  trigger: attack.hits_creature
  attack_sources:
  - weapon
  - wild_shape.beast_attack
  limit:
    count: 1
    period: actor.turn
  extra_damage:
    dice:
      count: 1
      sides: 8
    type_choice:
    - cold
    - fire
    - lightning
    - thunder
    choice_time: attack.hit
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` minimum_level ` | ` 7 ` |
