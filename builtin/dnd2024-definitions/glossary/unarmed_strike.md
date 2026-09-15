---
splat_schema: 1
splat_id: 0a5955ef-690d-5d69-89d2-f2ee734745b3
splat_kind: rule
splat_key: dnd2024:glossary/unarmed_strike
splat_name: Unarmed Strike
splat_origin:
  key: dnd2024:reference/glossary/unarmed_strike
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
---

# Unarmed Strike

## Source text

<!-- SRD page 190 -->

Instead of using a weapon to make a melee attack,\
you can use a punch, kick, headbutt, or similar\
forceful blow. In game terms, this is an Unarmed\
Strike—a melee attack that involves you using your\
body to damage, grapple, or shove a target within 5\
feet of you.\
Whenever you use your Unarmed Strike, choose\
one of the following options for its effect.\
Damage. You make an attack roll against the\
target. Your bonus to the roll equals your Strength\
modifier plus your Proficiency Bonus. On a hit, the\
target takes Bludgeoning damage equal to 1 plus\
your Strength modifier.\
Grapple. The target must succeed on a Strength\
or Dexterity saving throw (it chooses which), or it\
has the Grappled condition. The DC for the saving\
throw and any escape attempts equals 8 plus your\
Strength modifier and Proficiency Bonus. This grap-\
ple is possible only if the target is no more than one\
size larger than you and if you have a hand free to\
grab it. See also “Grappling.”\
Shove. The target must succeed on a Strength or\
Dexterity saving throw (it chooses which), or you ei-\
ther push it 5 feet away or cause it to have the Prone\
condition. The DC for the saving throw equals 8 plus\
your Strength modifier and Proficiency Bonus. This\
shove is possible only if the target is no more than\
one size larger than you.\

Original layout: [[../../srd-5.2.1/pages/190|page 190]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/unarmed_strike ` |
| ` implementation ` | ` dnd2024:glossary_unarmed_strike ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_type ` | ` melee ` |
| ` weapon_used ` | ` false ` |
| ` body_strike_examples ` | ` ["punch", "kick", "headbutt", "similar_forceful_blow"] ` |
| ` target_range_feet ` | ` 5 ` |

#### Effect choice

<!-- splat:values path=splat_rule/parameters/effect_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` subject ` |
| ` options ` | ` ["damage", "grapple", "shove"] ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
attack_bonus:
  op: add
  args:
  - var: strength_modifier
  - var: proficiency_bonus
hit_damage:
  type: bludgeoning
  amount:
    op: add
    args:
    - 1
    - var: strength_modifier
```

#### Grapple

<!-- splat:yaml path=splat_rule/parameters/grapple -->

```yaml
target_maximum_size_categories_larger: 1
free_hand_required: true
save_ability_choice:
  chooser: target
  options:
  - strength
  - dexterity
save_dc:
  op: add
  args:
  - 8
  - var: strength_modifier
  - var: proficiency_bonus
escape_dc: same_as_save_dc
failure_condition: grappled
```

#### Shove

<!-- splat:yaml path=splat_rule/parameters/shove -->

```yaml
target_maximum_size_categories_larger: 1
save_ability_choice:
  chooser: target
  options:
  - strength
  - dexterity
save_dc:
  op: add
  args:
  - 8
  - var: strength_modifier
  - var: proficiency_bonus
failure_choice:
  chooser: subject
  options:
  - push_feet: 5
    direction: away_from_subject
  - condition: prone
```
