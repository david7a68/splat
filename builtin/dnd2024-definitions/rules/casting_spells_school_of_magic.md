---
splat_schema: 1
splat_id: 62f14d6b-ec6c-53bf-83be-deeb4deb3b49
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_school_of_magic
splat_name: School of Magic
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_school_of_magic
  parameters:
    classification_only: true
    schools:
    - abjuration
    - conjuration
    - divination
    - enchantment
    - evocation
    - illusion
    - necromancy
    - transmutation
    standalone_school_effects: false
    other_rules_can_match_school: true
  implementation: dnd2024:rules_casting_spells_school_of_magic
splat_contracts:
- id: dnd2024:rules_casting_spells_school_of_magic
  inputs:
    spell:
      type: definition
      required: true
  outputs:
  - school
  events: []
  capabilities:
  - spell_metadata
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 105
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_school_of_magic
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: c8368af4bd1882087aa7cd6093576206b6653ceffc6d1600a60930e3349f02a1
  parent_name: Casting Spells
  line_span:
    first: 9127
    count: 16
splat_clauses:
- key: source_rules
  text: '# School of Magic Each spell belongs to a school of magic. The schools are listed in the Schools
    of Magic table. These categories help describe spells but have no rules of their own, although some
    other rules refer to them. Schools of Magic School Typical Effects Abjuration Prevents or reverses
    harmful effects Conjuration Transports creatures or objects Divination Reveals information Enchantment
    Influences minds Evocation Channels energy to create effects that are often destructive Illusion Deceives
    the mind or senses Necromancy Manipulates life and death Transmutation Transforms creatures or objects'
  sha256: fa91f3a42b57b99055ca56895eebf8a76c6b85aa70f3e6f3c27e56f1ea6821e4
  status: contract_specified
splat_requires_mechanics: *id001
---

# School of Magic

<!-- SRD page 105 -->

Each spell belongs to a school of magic. The schools\
are listed in the Schools of Magic table. These cate-\
gories help describe spells but have no rules of their\
own, although some other rules refer to them.\
Schools of Magic\
School Typical Effects\
Abjuration Prevents or reverses harmful effects\
Conjuration Transports creatures or objects\
Divination Reveals information\
Enchantment Influences minds\
Evocation Channels energy to create effects that\
are often destructive\
Illusion Deceives the mind or senses\
Necromancy Manipulates life and death\
Transmutation Transforms creatures or objects\
