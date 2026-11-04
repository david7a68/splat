---
splat_schema: 2
splat_id: 71a40ee4-b75b-5f4e-918f-cd0094b13923
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_1_sneak_attack
splat_name: 'Level 1: Sneak Attack'
splat_origin: dnd2024:reference/pages/061
splat_sources:
- SRD 5.2.1, pp. 61, 62, https://www.dndbeyond.com/srd
---

# Level 1: Sneak Attack

## Source text

<!-- SRD page 61 -->

You know how to strike subtly and exploit a foe’s distraction. Once per turn, you can deal an extra 1d6 damage to one creature you hit with an attack roll if you have Advantage on the roll and the attack uses a Finesse or a Ranged weapon. The extra damage’s type is the same as the weapon’s type.

**Rogue Features**

| Level | Proficiency Bonus | Class Features | Sneak Attack |
| --- | --- | --- | --- |
| 1 | +2 | Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery | 1d6 |
| 2 | +2 | Cunning Action | 1d6 |
| 3 | +2 | Rogue Subclass, Steady Aim | 2d6 |
| 4 | +2 | Ability Score Improvement | 2d6 |
| 5 | +3 | Cunning Strike, Uncanny Dodge | 3d6 |
| 6 | +3 | Expertise | 3d6 |
| 7 | +3 | Evasion, Reliable Talent | 4d6 |
| 8 | +3 | Ability Score Improvement | 4d6 |
| 9 | +4 | Subclass feature | 5d6 |
| 10 | +4 | Ability Score Improvement | 5d6 |
| 11 | +4 | Improved Cunning Strike | 6d6 |
| 12 | +4 | Ability Score Improvement | 6d6 |
| 13 | +5 | Subclass feature | 7d6 |
| 14 | +5 | Devious Strikes | 7d6 |
| 15 | +5 | Slippery Mind | 8d6 |
| 16 | +5 | Ability Score Improvement | 8d6 |
| 17 | +6 | Subclass feature | 9d6 |
| 18 | +6 | Elusive | 9d6 |
| 19 | +6 | Epic Boon | 10d6 |
| 20 | +6 | Stroke of Luck | 10d6 |

<!-- SRD page 62 -->

You don’t need Advantage on the attack roll if at least one of your allies is within 5 feet of the target, the ally doesn’t have the Incapacitated condition, and you don’t have Disadvantage on the attack roll.

The extra damage increases as you gain Rogue levels, as shown in the Sneak Attack column of the Rogue Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_1_sneak_attack ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_1_sneak_attack ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.attack_hits_creature ` |

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` period ` | ` turn ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` weapon_any ` | ` ["finesse_property", "ranged_weapon"] ` |

##### Any

<!-- splat:table
path: splat_rule/parameters/requires/any
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Attack advantage:
    path: attack_advantage
    type: boolean
    optional: true
  Attack disadvantage:
    path: attack_disadvantage
    type: boolean
    optional: true
  Ally within target feet:
    path: ally_within_target_feet
    type: integer
    optional: true
  Ally incapacitated:
    path: ally_incapacitated
    type: boolean
    optional: true
-->

| Key | Attack advantage | Attack disadvantage | Ally within target feet | Ally incapacitated |
| --- | --- | --- | --- | --- |
| ` row_1 ` | true |  |  |  |
| ` row_2 ` |  | false | ` 5 ` | false |

#### Extra damage

<!-- splat:yaml path=splat_rule/parameters/extra_damage -->

```yaml
dice:
  count:
    op: ceil
    args:
    - op: divide
      args:
      - var: actor.class_levels.rogue
      - 2
  sides: 6
type: weapon.damage_type
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 1 ` |
