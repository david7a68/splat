---
splat_schema: 1
splat_id: 71a40ee4-b75b-5f4e-918f-cd0094b13923
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_1_sneak_attack
splat_name: 'Level 1: Sneak Attack'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61, 62
  url: https://www.dndbeyond.com/srd
---

# Level 1: Sneak Attack

## Source text

<!-- SRD page 61 -->

You know how to strike subtly and exploit a foe’s\
distraction. Once per turn, you can deal an extra\
1d6 damage to one creature you hit with an attack\

<!-- SRD page 62 -->

Rogue Features\
Proficiency\
Level Bonus Class Features\
1 +2 Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery\
2 +2 Cunning Action\
3 +2 Rogue Subclass, Steady Aim\
4 +2 Ability Score Improvement\
5 +3 Cunning Strike, Uncanny Dodge\
6 +3 Expertise\
7 +3 Evasion, Reliable Talent\
8 +3 Ability Score Improvement\
9 +4 Subclass feature\
10 +4 Ability Score Improvement\
11 +4 Improved Cunning Strike\
12 +4 Ability Score Improvement\
13 +5 Subclass feature\
14 +5 Devious Strikes\
15 +5 Slippery Mind\
16 +5 Ability Score Improvement\
17 +6 Subclass feature\
18 +6 Elusive\
19 +6 Epic Boon\
20 +6 Stroke of Luck\
roll if you have Advantage on the roll and the attack\
uses a Finesse or a Ranged weapon. The extra dam-\
age’s type is the same as the weapon’s type.\
You don’t need Advantage on the attack roll if at\
least one of your allies is within 5 feet of the target,\
the ally doesn’t have the Incapacitated condition,\
and you don’t have Disadvantage on the attack roll.\
The extra damage increases as you gain Rogue\
levels, as shown in the Sneak Attack column of the\
Rogue Features table.\

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

<!-- splat:yaml path=splat_rule/parameters/requires -->

```yaml
weapon_any:
- finesse_property
- ranged_weapon
any:
- attack_advantage: true
- attack_disadvantage: false
  ally_within_target_feet: 5
  ally_incapacitated: false
```

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
