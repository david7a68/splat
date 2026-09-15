---
splat_schema: 1
splat_id: 94d50f9b-0274-56c2-9eb4-672376348dd2
splat_kind: rule
splat_key: dnd2024:rules/proficiency_skill_proficiencies
splat_name: Skill Proficiencies
splat_origin:
  key: dnd2024:reference/pages/008
splat_sources:
- title: SRD 5.2.1
  locator: pp. 8, 9
  url: https://www.dndbeyond.com/srd
---

# Skill Proficiencies

## Source text

<!-- SRD page 8 -->

Most ability checks involve using a skill, which rep-\
resents a category of things creatures try to do with\
an ability check. The descriptions of the actions\
you take (see “Actions” later in “Playing the Game”)\
specify which skill applies if you make an ability\
check for that action, and many other rules note\
when a skill is relevant. The GM has the ultimate say\
on whether a skill is relevant in a situation.\
If a creature is proficient in a skill, the creature\
applies its Proficiency Bonus to ability checks in-\
volving that skill. Without proficiency in a skill, a\
creature can still make ability checks involving\

<!-- SRD page 9 -->

Skills\
Skill Ability Example Uses\
Acrobatics Dexterity Stay on your feet in a tricky situation, or perform an acrobatic stunt.\
Animal Handling Wisdom Calm or train an animal, or get an animal to behave in a certain way.\
Arcana Intelligence Recall lore about spells, magic items, and the planes of existence.\
Athletics Strength Jump farther than normal, stay afloat in rough water, or break something.\
Deception Charisma Tell a convincing lie, or wear a disguise convincingly.\
History Intelligence Recall lore about historical events, people, nations, and cultures.\
Insight Wisdom Discern a person’s mood and intentions.\
Intimidation Charisma Awe or threaten someone into doing what you want.\
Investigation Intelligence Find obscure information in books, or deduce how something works.\
Medicine Wisdom Diagnose an illness, or determine what killed the recently slain.\
Nature Intelligence Recall lore about terrain, plants, animals, and weather.\
Perception Wisdom Using a combination of senses, notice something that’s easy to miss.\
Performance Charisma Act, tell a story, perform music, or dance.\
Persuasion Charisma Honestly and graciously convince someone of something.\
Religion Intelligence Recall lore about gods, religious rituals, and holy symbols.\
Sleight of Hand Dexterity Pick a pocket, conceal a handheld object, or perform legerdemain.\
Stealth Dexterity Escape notice by moving quietly and hiding behind things.\
Survival Wisdom Follow tracks, forage, find a trail, or avoid natural hazards.\
that skill but doesn’t add its Proficiency Bonus. For\
example, if a character tries to climb a cliff, the GM\
might ask for a Strength (Athletics) check. If the\
character has Athletics proficiency, the character\
adds their Proficiency Bonus to the Strength check.\
If the character lacks that proficiency, they make\
the check without adding their Proficiency Bonus.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/proficiency_skill_proficiencies ` |
| ` implementation ` | ` dnd2024:rule_proficiency_skill_proficiencies ` |

### Parameters

#### Skill abilities

<!-- splat:values path=splat_rule/parameters/skill_abilities -->

| Field | Value |
| --- | --- |
| ` acrobatics ` | ` dexterity ` |
| ` animal_handling ` | ` wisdom ` |
| ` arcana ` | ` intelligence ` |
| ` athletics ` | ` strength ` |
| ` deception ` | ` charisma ` |
| ` history ` | ` intelligence ` |
| ` insight ` | ` wisdom ` |
| ` intimidation ` | ` charisma ` |
| ` investigation ` | ` intelligence ` |
| ` medicine ` | ` wisdom ` |
| ` nature ` | ` intelligence ` |
| ` perception ` | ` wisdom ` |
| ` performance ` | ` charisma ` |
| ` persuasion ` | ` charisma ` |
| ` religion ` | ` intelligence ` |
| ` sleight_of_hand ` | ` dexterity ` |
| ` stealth ` | ` dexterity ` |
| ` survival ` | ` wisdom ` |

#### Relevant skill

<!-- splat:values path=splat_rule/parameters/relevant_skill -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` rules_suggest ` | ` true ` |

#### Proficient

<!-- splat:values path=splat_rule/parameters/proficient -->

| Field | Value |
| --- | --- |
| ` add ` | ` proficiency_bonus ` |

#### Unproficient

<!-- splat:values path=splat_rule/parameters/unproficient -->

| Field | Value |
| --- | --- |
| ` check_permitted ` | ` true ` |
| ` proficiency_bonus ` | ` 0 ` |
