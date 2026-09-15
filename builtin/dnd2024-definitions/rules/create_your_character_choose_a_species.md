---
splat_schema: 1
splat_id: cece6c90-0039-55f7-83d9-803e352b05c7
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_choose_a_species
splat_name: Choose a Species
splat_origin:
  key: dnd2024:reference/pages/020
splat_sources:
- title: SRD 5.2.1
  locator: pp. 20
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_choose_a_species
  parameters:
    choose:
      count: 1
      options:
      - key: dnd2024:species/dragonborn
      - key: dnd2024:species/dwarf
      - key: dnd2024:species/elf
      - key: dnd2024:species/gnome
      - key: dnd2024:species/goliath
      - key: dnd2024:species/halfling
      - key: dnd2024:species/human
      - key: dnd2024:species/orc
      - key: dnd2024:species/tiefling
    inherit:
    - traits
    - size
    - speed
  implementation: dnd2024:rule_create_your_character_choose_a_species
splat_contracts:
- id: dnd2024:rule_create_your_character_choose_a_species
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 20
  engine_gaps: &id001
  - dnd2024:rule_create_your_character_choose_a_species
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 488df08d499131486a42de6328a01707ddea870f8e78479f3976bf657d1ac43e
  parent_name: Create Your Character
  line_span:
    first: 1443
    count: 11
splat_clauses:
- key: source_rules
  text: '# Choose a Species Choose a species for your character. The following species options are detailed
    in “Character Origins”: Dragonborn, Dwarf, Elf, Gnome, Goliath, Halfling, Human, Orc, and Tiefling.
    Once you’ve chosen a species, write it on your character sheet. Then record your species’ traits.
    Your character’s size and Speed are determined by the character’s species; record these in the appropriate
    places on your character sheet as well (you may write just the first letter of your size).'
  sha256: 4a7d8035e5785cfdc26dab42a006fb4ee0b5ed138f02ea6852376da398635eb2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Choose a Species

<!-- SRD page 20 -->

Choose a species for your character. The following\
species options are detailed in “Character Origins”:\
Dragonborn, Dwarf, Elf, Gnome, Goliath, Halfling,\
Human, Orc, and Tiefling. Once you’ve chosen a spe-\
cies, write it on your character sheet. Then record\
your species’ traits.\
Your character’s size and Speed are determined\
by the character’s species; record these in the ap-\
propriate places on your character sheet as well\
(you may write just the first letter of your size).\
