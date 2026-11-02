---
splat_schema: 1
splat_id: 1146923b-6200-5244-a2d0-54a361b76f2d
splat_kind: item
splat_key: dnd2024:equipment/net_1_gp
splat_name: Net (1 GP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
---

# Net (1 GP)

## Source text

<!-- SRD page 98 -->

When you take the Attack action, you can replace one of your attacks with throwing a Net. Target a creature you can see within 15 feet of yourself. The target must succeed on a Dexterity saving throw (DC 8 plus your Dexterity modifier and Proficiency Bonus) or have the Restrained condition until it escapes. The target succeeds automatically if it is Huge or larger.

To escape, the target or a creature within 5 feet of it must take an action to make a DC 10 Strength (Athletics) check, freeing the Restrained creature on a success. Destroying the Net (AC 10; 5 HP; Immunity to Bludgeoning, Poison, and Psychic damage) also frees the target, ending the effect.

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
| ` value ` | ` 1 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/net_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_net_1_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` on_destroy ` | ` free_target ` |

#### Throw

<!-- splat:yaml path=splat_rule/parameters/throw -->

```yaml
economy: replace_one_attack
action: attack
target:
  kind: creature
  visible: true
  range_ft: 15
save:
  ability: dexterity
  dc:
    op: add
    operands:
    - 8
    - ability_modifier: dexterity
    - stat: proficiency_bonus
  automatic_success_sizes:
  - huge
  - gargantuan
  success: no_effect
  failure:
    condition: restrained
    until: escape_or_net_destroyed
```

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
economy: action
actor: target_or_creature_within_5_ft
check:
  ability: strength
  skill: athletics
  dc: 10
success: free_target
```

#### Object stats

<!-- splat:values path=splat_rule/parameters/object_stats -->

| Field | Value |
| --- | --- |
| ` armor_class ` | ` 10 ` |
| ` hit_points ` | ` 5 ` |
| ` damage_immunities ` | ` ["bludgeoning", "poison", "psychic"] ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3.0 ` |
| ` unit ` | ` lb ` |
