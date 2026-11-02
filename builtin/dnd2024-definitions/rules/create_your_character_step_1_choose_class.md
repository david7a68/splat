---
splat_schema: 1
splat_id: 480440e0-e1f4-55de-8648-7cc15ece079c
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_step_1_choose_class
splat_name: 'Step 1: Choose Class'
splat_origin:
  key: dnd2024:reference/pages/019
splat_sources:
- title: SRD 5.2.1
  locator: pp. 19
  url: https://www.dndbeyond.com/srd
---

# Step 1: Choose Class

## Source text

<!-- SRD page 19 -->

Choose a class, and write it on your character sheet. The Class Overview table summarizes the classes. See “Classes” for the classes’ details.

**Class Overview**

| Class | Likes ... | Primary Ability | Complexity |
| --- | --- | --- | --- |
| Barbarian | Battle | Strength | Average |
| Bard | Performing | Charisma | High |
| Cleric | Gods | Wisdom | Average |
| Druid | Nature | Wisdom | High |
| Fighter | Weapons | Strength or Dexterity | Low |
| Monk | Unarmed combat | Dexterity and Wisdom | High |
| Paladin | Defense | Strength and Charisma | Average |
| Ranger | Survival | Dexterity and Wisdom | Average |

| Class | Likes ... | Primary Ability | Complexity |
| --- | --- | --- | --- |
| Rogue | Stealth | Dexterity | Low |
| Sorcerer | Power | Charisma | High |
| Warlock | Occult lore | Charisma | High |
| Wizard | Spellbooks | Intelligence | Average |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_step_1_choose_class ` |
| ` implementation ` | ` dnd2024:rule_create_your_character_step_1_choose_class ` |

### Parameters

#### Choose

<!-- splat:yaml path=splat_rule/parameters/choose -->

```yaml
count: 1
options:
- key: dnd2024:classes/barbarian
- key: dnd2024:classes/bard
- key: dnd2024:classes/cleric
- key: dnd2024:classes/druid
- key: dnd2024:classes/fighter
- key: dnd2024:classes/monk
- key: dnd2024:classes/paladin
- key: dnd2024:classes/ranger
- key: dnd2024:classes/rogue
- key: dnd2024:classes/sorcerer
- key: dnd2024:classes/warlock
- key: dnd2024:classes/wizard
```

#### Class primary abilities

<!-- splat:yaml path=splat_rule/parameters/class_primary_abilities -->

```yaml
barbarian:
- strength
bard:
- charisma
cleric:
- wisdom
druid:
- wisdom
fighter:
  any:
  - strength
  - dexterity
monk:
- dexterity
- wisdom
paladin:
- strength
- charisma
ranger:
- dexterity
- wisdom
rogue:
- dexterity
sorcerer:
- charisma
warlock:
- charisma
wizard:
- intelligence
```
