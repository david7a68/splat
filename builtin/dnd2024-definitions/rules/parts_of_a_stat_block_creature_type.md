---
splat_schema: 1
splat_id: 8f415528-ba40-55fd-8d90-a277e4accdf5
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_creature_type
splat_name: Creature Type
splat_origin:
  key: dnd2024:reference/pages/254
splat_sources:
- title: SRD 5.2.1
  locator: pp. 254
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_creature_type
  parameters:
    creature_type_enum:
    - aberration
    - beast
    - celestial
    - construct
    - dragon
    - elemental
    - fey
    - fiend
    - giant
    - humanoid
    - monstrosity
    - ooze
    - plant
    - undead
    type_inherent_rules: false
    type_filters_other_effects: true
  implementation: dnd2024:rules_parts_of_a_stat_block_creature_type
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_creature_type
  inputs:
    definition:
      type: definition
      required: true
  outputs:
  - typed_attributes
  events: []
  capabilities:
  - definition_catalogs
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 254
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_creature_type
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 57681723a579a6647b842e8b4046cbcef16957b034fff45765e93b735506b03a
  parent_name: Parts of a Stat Block
  line_span:
    first: 10894
    count: 36
splat_clauses:
- key: source_rules
  text: '# Creature Type Each monster has a tag that identifies the type of creature it is. Certain spells,
    magic items, class features, and other effects in the game interact in special ways with creatures
    of a particular type. The game includes the following creature types, which have no rules of their
    own: Aberrations are utterly alien beings, such as aboleths and cloakers. Beasts are non-Humanoid
    natural creatures, like horses and wolves, as well as most giant animals. Celestials are magical creatures,
    such as angels and pegasi, with ties to the Upper Planes. Constructs are magically created creatures
    such as homunculi and shield guardians. Dragons are scaly beings of ancient origin, such as red dragons
    and wyverns. Elementals are beings from the Elemental Planes, such as efreet and water elementals.
    Fey are creatures tied to the Feywild or the forces of nature, such as dryads and goblins. Fiends
    are creatures tied to terrifying Lower Planes, such as balors and hell hounds. Giants are towering
    beings with humanlike shapes, like fire giants and trolls. Humanoids are people defined by their roles
    and professions, such as mages, pirates, and warriors. They include members of varied species. Monstrosities
    are unnatural creatures with strange origins, such as mimics and owlbears. Oozes are gelatinous creatures,
    including black puddings and gelatinous cubes. Plants are sentient vegetation and fungal monsters,
    such as shambling mounds and treants. Undead are spirits and the reanimated dead, such as ghosts,
    vampires, and zombies.'
  sha256: 88620dafe85482293f64510de1b71a2e73313876bf36591987201046563189b9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Creature Type

<!-- SRD page 254 -->

Each monster has a tag that identifies the type of\
creature it is. Certain spells, magic items, class fea-\
tures, and other effects in the game interact in spe-\
cial ways with creatures of a particular type.\
The game includes the following creature types,\
which have no rules of their own:\
Aberrations are utterly alien beings, such as abo-\
leths and cloakers.\
Beasts are non-Humanoid natural creatures, like\
horses and wolves, as well as most giant animals.\
Celestials are magical creatures, such as angels and\
pegasi, with ties to the Upper Planes.\
Constructs are magically created creatures such as\
homunculi and shield guardians.\
Dragons are scaly beings of ancient origin, such as\
red dragons and wyverns.\
Elementals are beings from the Elemental Planes,\
such as efreet and water elementals.\
Fey are creatures tied to the Feywild or the forces\
of nature, such as dryads and goblins.\
Fiends are creatures tied to terrifying Lower\
Planes, such as balors and hell hounds.\
Giants are towering beings with humanlike shapes,\
like fire giants and trolls.\
Humanoids are people defined by their roles and\
professions, such as mages, pirates, and warriors.\
They include members of varied species.\
Monstrosities are unnatural creatures with\
strange origins, such as mimics and owlbears.\
Oozes are gelatinous creatures, including black\
puddings and gelatinous cubes.\
Plants are sentient vegetation and fungal monsters,\
such as shambling mounds and treants.\
Undead are spirits and the reanimated dead, such\
as ghosts, vampires, and zombies.\
