---
splat_schema: 1
splat_id: 578f7a44-f954-5ae1-8faa-a95da7eb24c9
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_3_deflect_attacks
splat_name: 'Level 3: Deflect Attacks'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
---

# Level 3: Deflect Attacks

## Source text

<!-- SRD page 51 -->

When an attack roll hits you and its damage includes Bludgeoning, Piercing, or Slashing damage, you can take a Reaction to reduce the attack’s total damage against you. The reduction equals 1d10 plus your Dexterity modifier and Monk level.

If you reduce the damage to 0, you can expend 1 Focus Point to redirect some of the attack’s force. If you do so, choose a creature you can see within 5 feet of yourself if the attack was a melee attack or a creature you can see within 60 feet of yourself that isn’t behind Total Cover if the attack was a ranged attack. That creature must succeed on a Dexterity saving throw or take damage equal to two rolls of your Martial Arts die plus your Dexterity modifier. The damage is the same type dealt by the attack.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_3_deflect_attacks ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_3_deflect_attacks ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` reaction ` |
| ` trigger ` | ` attack.hit_actor ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` damage_types_include_any ` | ` ["bludgeoning", "piercing", "slashing"] ` |

#### Reduce total damage

<!-- splat:values path=splat_rule/parameters/reduce_total_damage -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/reduce_total_damage/args
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Dice count:
    path: dice/count
    type: integer
    optional: true
  Dice sides:
    path: dice/sides
    type: integer
    optional: true
  Var:
    path: var
    type: string
    optional: true
-->

| Key | Dice count | Dice sides | Var |
| --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 10 ` |  |
| ` row_2 ` |  |  | ` actor.abilities.dexterity.modifier ` |
| ` row_3 ` |  |  | ` actor.class_levels.monk ` |

#### If damage reduced to zero

<!-- splat:values path=splat_rule/parameters/if_damage_reduced_to_zero -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |

##### Cost

<!-- splat:values path=splat_rule/parameters/if_damage_reduced_to_zero/cost -->

| Field | Value |
| --- | --- |
| ` resource ` | ` monk.focus_points ` |
| ` amount ` | ` 1 ` |

##### Target

<!-- splat:yaml path=splat_rule/parameters/if_damage_reduced_to_zero/target -->

```yaml
visible: true
type: creature
if_melee_attack:
  range_feet: 5
if_ranged_attack:
  range_feet: 60
  total_cover: false
```

##### Save

<!-- splat:values path=splat_rule/parameters/if_damage_reduced_to_zero/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` success_damage ` | ` 0 ` |

###### Dc

<!-- splat:yaml path=splat_rule/parameters/if_damage_reduced_to_zero/save/dc -->

```yaml
op: add
args:
- 8
- var: actor.abilities.wisdom.modifier
- var: actor.proficiency_bonus
```

###### Failure damage

<!-- splat:values path=splat_rule/parameters/if_damage_reduced_to_zero/save/failure_damage -->

| Field | Value |
| --- | --- |
| ` type ` | ` trigger_attack.damage_type ` |

###### Amount

<!-- splat:values path=splat_rule/parameters/if_damage_reduced_to_zero/save/failure_damage/amount -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

###### Args

<!-- splat:table
path: splat_rule/parameters/if_damage_reduced_to_zero/save/failure_damage/amount/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Dice count:
    path: dice/count
    type: integer
    optional: true
  Var:
    path: var
    type: string
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Dice count | Var | Details |
| --- | --- | --- | --- |
| ` row_1 ` | ` 2 ` |  | [Rules](#^splat-5d551cdb-row_1 "section:row_1") |
| ` row_2 ` |  | ` actor.abilities.dexterity.modifier ` |  |

###### Row 1

^splat-5d551cdb-row_1

###### Sides

<!-- splat:yaml path=splat_rule/parameters/if_damage_reduced_to_zero/save/failure_damage/amount/args/@row_1/dice/sides -->

```yaml
martial_arts_die_sides:
  table:
    key: dnd2024:tables/monk_progression
  row:
    var: actor.class_levels.monk
  column: martial_arts
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 3 ` |
