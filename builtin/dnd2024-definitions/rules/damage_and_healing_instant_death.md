---
splat_schema: 2
splat_id: ce429d70-8e73-5bfa-8c4b-cb8c874d9b9a
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_instant_death
splat_name: Instant Death
splat_origin: dnd2024:reference/pages/017
splat_sources:
- SRD 5.2.1, pp. 17, https://www.dndbeyond.com/srd
---

# Instant Death

## Source text

<!-- SRD page 17 -->

Here are the main ways a creature can die instantly.

Monster Death. A monster dies the instant it drops to 0 Hit Points, although a Game Master can ignore this rule for an individual monster and treat it like a character.

Hit Point Maximum of 0. A creature dies if its Hit Point maximum reaches 0. Certain effects drain life energy, reducing a creature’s Hit Point maximum.

Massive Damage. When damage reduces a character to 0 Hit Points and damage remains, the character dies if the remainder equals or exceeds their Hit Point maximum. For example, if your character has a Hit Point maximum of 12, currently has 6 Hit Points, and takes 18 damage, the character drops to 0 Hit Points, but 12 damage remains. The character then dies, since 12 equals their Hit Point maximum.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_instant_death ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_instant_death ` |

### Parameters

#### Death conditions

<!-- splat:table
path: splat_rule/parameters/death_conditions
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Subject:
    path: subject
    type: symbol
    optional: true
  Hp:
    path: hp
    type: integer
    optional: true
  Exception authority:
    path: exception/authority
    type: symbol
    optional: true
  Exception choice:
    path: exception/choice
    type: symbol
    optional: true
  Hp maximum:
    path: hp_maximum
    type: integer
    optional: true
  Hp reduced to:
    path: hp_reduced_to
    type: integer
    optional: true
  Remaining damage gte:
    path: remaining_damage_gte
    type: symbol
    optional: true
-->

| Key | Subject | Hp | Exception authority | Exception choice | Hp maximum | Hp reduced to | Remaining damage gte |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Monster ` | ` 0 ` | ` Gm ` | ` Use character death rules ` |  |  |  |
| ` row_2 ` |  |  |  |  | ` 0 ` |  |  |
| ` row_3 ` | ` Character ` |  |  |  |  | ` 0 ` | ` Hp maximum ` |

#### Death rules

<!-- splat:values path=splat_rule/parameters/death_rules -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/damage_and_healing_death_saving_throws ` |
