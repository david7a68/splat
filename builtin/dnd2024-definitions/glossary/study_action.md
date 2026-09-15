---
splat_schema: 1
splat_id: 062ec5a9-e058-554f-8e75-ac6cac62c9c5
splat_kind: rule
splat_key: dnd2024:glossary/study_action
splat_name: Study [Action]
splat_origin:
  key: dnd2024:reference/glossary/study_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 189
  url: https://www.dndbeyond.com/srd
---

# Study [Action]

## Source text

<!-- SRD page 189 -->

When you take the Study action, you make an Intel-\
ligence check to study your memory, a book, a clue,\
or another source of knowledge and call to mind an\
important piece of information about it.\
The Areas of Knowledge table suggests which\
skills are applicable to various areas of knowledge.\
Areas of Knowledge\
Skill Areas\
Arcana Spells, magic items, eldritch symbols,\
magical traditions, planes of existence,\
and certain creatures (Aberrations,\
Constructs, Elementals, Fey, and\
Monstrosities)\
History Historic events and people, ancient\
civilizations, wars, and certain creatures\
(Giants and Humanoids)\
Investigation Traps, ciphers, riddles, and gadgetry\
Nature Terrain, flora, weather, and certain\
creatures (Beasts, Dragons, Oozes, and\
Plants)\
Religion Deities, religious hierarchies and rites,\
holy symbols, cults, and certain crea-\
tures (Celestials, Fiends, and Undead)\

Original layout: [[../../srd-5.2.1/pages/189|page 189]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/study_action ` |
| ` implementation ` | ` dnd2024:glossary_study_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` ability ` | ` intelligence ` |
| ` sources ` | ` ["memory", "book", "clue", "other_knowledge_source"] ` |
| ` result ` | ` recall_important_information ` |

#### Suggested skills

<!-- splat:yaml path=splat_rule/parameters/suggested_skills -->

```yaml
arcana:
  topics:
  - spells
  - magic_items
  - eldritch_symbols
  - magical_traditions
  - planes
  creature_types:
  - aberration
  - construct
  - elemental
  - fey
  - monstrosity
history:
  topics:
  - historic_events
  - historic_people
  - ancient_civilizations
  - wars
  creature_types:
  - giant
  - humanoid
investigation:
  topics:
  - traps
  - ciphers
  - riddles
  - gadgetry
nature:
  topics:
  - terrain
  - flora
  - weather
  creature_types:
  - beast
  - dragon
  - ooze
  - plant
religion:
  topics:
  - deities
  - religious_hierarchies
  - religious_rites
  - holy_symbols
  - cults
  creature_types:
  - celestial
  - fiend
  - undead
```
