---
splat_schema: 1
splat_id: 324d5c1b-b439-56a5-8b13-dd48e1ea576b
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_choose_languages
splat_name: Choose Languages
splat_origin:
  key: dnd2024:reference/pages/020
splat_sources:
- title: SRD 5.2.1
  locator: pp. 20
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_choose_languages
  parameters:
    granted_languages:
    - common
    additional_standard_languages:
      count: 2
      selection:
      - choose
      - roll
      dice:
        count: 1
        sides: 12
      table:
      - min: 1
        max: 1
        language: common_sign_language
      - min: 2
        max: 2
        language: draconic
      - min: 3
        max: 4
        language: dwarvish
      - min: 5
        max: 6
        language: elvish
      - min: 7
        max: 7
        language: giant
      - min: 8
        max: 8
        language: gnomish
      - min: 9
        max: 9
        language: goblin
      - min: 10
        max: 11
        language: halfling
      - min: 12
        max: 12
        language: orc
    knowledge:
    - communicate
    - read
    - write
    additional_grants_source: features
    rare_requires_feature: true
    rare_languages:
    - abyssal
    - celestial
    - deep_speech
    - druidic
    - infernal
    - primordial
    - sylvan
    - thieves_cant
    - undercommon
    primordial:
      dialects:
      - aquan
      - auran
      - ignan
      - terran
      mutually_intelligible: true
  implementation: dnd2024:rules_create_your_character_choose_languages
splat_contracts:
- id: dnd2024:rules_create_your_character_choose_languages
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - languages
  events: []
  capabilities:
  - character_creation
  - languages
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 20
  engine_gaps: &id001
  - dnd2024:rules_create_your_character_choose_languages
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: b38e1f59ccb8fee5051bfb53a2aa1dad5879f5119e5af2d25c6121c6f26c490f
  parent_name: Create Your Character
  line_span:
    first: 1467
    count: 40
splat_clauses:
- key: source_rules
  text: '# Choose Languages Your character knows at least three languages: Common plus two languages you
    roll or choose from the Standard Languages table. Knowledge of a language means your character can
    communicate in it, read it, and write it. Your class and other features might also give you languages.
    The Standard Languages table lists languages that are widespread in the setting. Every player character
    knows Common. The other standard languages originated with the first members of the most prominent
    species in the setting and have since spread widely. Standard Languages 1d12 Language — Common 1 Common
    Sign Language 2 Draconic 3–4 Dwarvish 5–6 Elvish 7 Giant 8 Gnomish 9 Goblin 10–11 Halfling 12 Orc
    The Rare Languages table lists languages that are either secret or derived from other planes of existence
    and thus less widespread in the worlds of the Material Plane. Some features let a character learn
    a rare language. Rare Languages Language Language Abyssal Primordial* Celestial Sylvan Deep Speech
    Thieves’ Cant Druidic Undercommon Infernal *Primordial includes the Aquan, Auran, Ignan, and Terran
    dialects. Creatures that know one of these dialects can communicate with those that know a different
    one.'
  sha256: 8f036381f1f4dd9b4e005e72aa9f0c5dfe6132c2fd9d3d83f560dfbe7944fb1c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Choose Languages

<!-- SRD page 20 -->

Your character knows at least three languages:\
Common plus two languages you roll or choose\
from the Standard Languages table. Knowledge of a\
language means your character can communicate in\
it, read it, and write it. Your class and other features\
might also give you languages.\
The Standard Languages table lists languages that\
are widespread in the setting. Every player charac-\
ter knows Common. The other standard languages\
originated with the first members of the most\
prominent species in the setting and have since\
spread widely.\
Standard Languages\
1d12 Language\
— Common\
1 Common Sign Language\
2 Draconic\
3–4 Dwarvish\
5–6 Elvish\
7 Giant\
8 Gnomish\
9 Goblin\
10–11 Halfling\
12 Orc\
The Rare Languages table lists languages that are\
either secret or derived from other planes of exis-\
tence and thus less widespread in the worlds of the\
Material Plane. Some features let a character learn a\
rare language.\
Rare Languages\
Language Language\
Abyssal Primordial*\
Celestial Sylvan\
Deep Speech Thieves’ Cant\
Druidic Undercommon\
Infernal\
*Primordial includes the Aquan, Auran, Ignan, and Terran dialects.\
Creatures that know one of these dialects can communicate with\
those that know a different one.\
