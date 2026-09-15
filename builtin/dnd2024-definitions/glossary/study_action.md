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
splat_rule:
  mechanic: dnd2024:glossary/study_action
  parameters:
    cost: action
    ability: intelligence
    sources:
    - memory
    - book
    - clue
    - other_knowledge_source
    result: recall_important_information
    suggested_skills:
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
  implementation: dnd2024:glossary_study_action
splat_contracts:
- id: dnd2024:glossary_study_action
  inputs:
    subject:
      type: participant
      required: true
    topic:
      type: choice
      required: true
    knowledge_source:
      type: object
      required: true
    dc:
      type: integer
      required: true
  outputs:
  - check
  - recalled_information
  events:
  - action.requested
  capabilities:
  - action_economy
  - checks
  - knowledge
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 50f67bf972884e47dc49d480a478e92a3a3c718f789e3e3a0121fb1512ba35f9
  source_path: builtin/srd-5.2.1/glossary/study_action.md
  pages:
  - 189
  engine_gaps: &id001
  - dnd2024:glossary_study_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: a6a411759fc888d0e82d0c09faad568faf8ccf47374d2f7e89339f3bc7e2c294
splat_clauses:
- key: source_rules
  text: '# Study [Action] When you take the Study action, you make an Intelligence check to study your
    memory, a book, a clue, or another source of knowledge and call to mind an important piece of information
    about it. The Areas of Knowledge table suggests which skills are applicable to various areas of knowledge.
    Areas of Knowledge Skill Areas Arcana Spells, magic items, eldritch symbols, magical traditions, planes
    of existence, and certain creatures (Aberrations, Constructs, Elementals, Fey, and Monstrosities)
    History Historic events and people, ancient civilizations, wars, and certain creatures (Giants and
    Humanoids) Investigation Traps, ciphers, riddles, and gadgetry Nature Terrain, flora, weather, and
    certain creatures (Beasts, Dragons, Oozes, and Plants) Religion Deities, religious hierarchies and
    rites, holy symbols, cults, and certain creatures (Celestials, Fiends, and Undead)'
  sha256: de1aeb79d6f2b644d47cab79decb67ecaddd601f84ae52d59683c88c8bcebf7e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Study [Action]

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
