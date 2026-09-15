---
splat_schema: 1
splat_id: 9cda0de3-9384-5323-978c-32a334b7ab48
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_assign_ability_scores
splat_name: Assign Ability Scores
splat_origin:
  key: dnd2024:reference/pages/021
splat_sources:
- title: SRD 5.2.1
  locator: pp. 21
  url: https://www.dndbeyond.com/srd
---

# Assign Ability Scores

## Source text

<!-- SRD page 21 -->

Once you’ve generated six scores, assign them to\
Strength, Dexterity, Constitution, Intelligence, Wis-\
dom, and Charisma, keeping in mind your class’s\
primary ability. Fill in the ability modifiers as well.\
If you’re using the Standard Array option, consult\
the Standard Array by Class table for suggestions\
on where to assign scores for your character’s class.\
The table puts the highest scores in a class’s main\
abilities. If you used a different method to gener-\
ate the scores, you may still use this table to guide\
where you place your highest and lowest scores.\
Standard Array by Class\
Class Str. Dex. Con. Int. Wis. Cha.\
Barbarian 15 13 14 10 12 8\
Bard 8 14 12 13 10 15\
Cleric 14 8 13 10 15 12\
Druid 8 12 14 13 15 10\
Fighter 15 14 13 8 10 12\
Monk 12 15 13 10 14 8\
Paladin 15 10 13 8 12 14\
Ranger 12 15 13 8 14 10\
Rogue 12 15 13 14 10 8\
Sorcerer 10 13 14 8 12 15\
Warlock 8 14 13 12 10 15\
Wizard 8 12 13 15 14 10\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_assign_ability_scores ` |
| ` implementation ` | ` dnd2024:rules_create_your_character_assign_ability_scores ` |

### Parameters

#### Assign generated scores

<!-- splat:values path=splat_rule/parameters/assign_generated_scores -->

| Field | Value |
| --- | --- |
| ` bijection ` | ` true ` |
| ` abilities ` | ` ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"] ` |

#### Ability modifier

<!-- splat:yaml path=splat_rule/parameters/ability_modifier -->

```yaml
op: floor
args:
- op: divide
  args:
  - op: subtract
    args:
    - var: ability_score
    - 10
  - 2
```

#### Suggested standard array assignments

<!-- splat:yaml path=splat_rule/parameters/suggested_standard_array_assignments -->

```yaml
barbarian:
  strength: 15
  dexterity: 13
  constitution: 14
  intelligence: 10
  wisdom: 12
  charisma: 8
bard:
  strength: 8
  dexterity: 14
  constitution: 12
  intelligence: 13
  wisdom: 10
  charisma: 15
cleric:
  strength: 14
  dexterity: 8
  constitution: 13
  intelligence: 10
  wisdom: 15
  charisma: 12
druid:
  strength: 8
  dexterity: 12
  constitution: 14
  intelligence: 13
  wisdom: 15
  charisma: 10
fighter:
  strength: 15
  dexterity: 14
  constitution: 13
  intelligence: 8
  wisdom: 10
  charisma: 12
monk:
  strength: 12
  dexterity: 15
  constitution: 13
  intelligence: 10
  wisdom: 14
  charisma: 8
paladin:
  strength: 15
  dexterity: 10
  constitution: 13
  intelligence: 8
  wisdom: 12
  charisma: 14
ranger:
  strength: 12
  dexterity: 15
  constitution: 13
  intelligence: 8
  wisdom: 14
  charisma: 10
rogue:
  strength: 12
  dexterity: 15
  constitution: 13
  intelligence: 14
  wisdom: 10
  charisma: 8
sorcerer:
  strength: 10
  dexterity: 13
  constitution: 14
  intelligence: 8
  wisdom: 12
  charisma: 15
warlock:
  strength: 8
  dexterity: 14
  constitution: 13
  intelligence: 12
  wisdom: 10
  charisma: 15
wizard:
  strength: 8
  dexterity: 12
  constitution: 13
  intelligence: 15
  wisdom: 14
  charisma: 10
```
