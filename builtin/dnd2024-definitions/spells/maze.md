---
splat_schema: 1
splat_id: 64fc0893-46bb-5a96-a967-3e00f78b49c4
splat_kind: spell
splat_key: dnd2024:spells/maze
splat_name: Maze
splat_origin:
  key: dnd2024:reference/spell/maze
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: conjuration
  classes:
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/maze
  implementation: dnd2024:spell_maze
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 600
    target:
      kind: creature
      visible: true
      range_feet: 60
    banish:
      destination: labyrinthine_demiplane
      initial_save: false
    escape:
      economy: study_action
      check:
        ability: intelligence
        skill: investigation
        dc: 20
      success:
        escape: true
        end_spell: true
    on_spell_end:
      return: original_space
      occupied_fallback: nearest_unoccupied_space
splat_contracts:
- id: dnd2024:spell_maze
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target:
      type: participant
      required: true
  outputs:
  - effects
  - transports
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - planar_travel
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5560faae6c298e42ca6d70dfe4ef1a43ea0efa3043a022b890d52c70f6e3e7f0
  source_path: builtin/srd-5.2.1/spell/maze.md
  pages:
  - 148
  engine_gaps: &id001
  - dnd2024:spell_maze
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: b1686f4ed7abbf6cd459cb0b18598cce3277aacdb1a9c79e3302f68830b23094
splat_clauses:
- key: source_rules
  text: '# Maze Level 8 Conjuration (Wizard) Casting Time: Action Range: 60 feet Components: V, S Duration:
    Concentration, up to 10 minutes You banish a creature that you can see within range into a labyrinthine
    demiplane. The target remains there for the duration or until it escapes the maze. The target can
    take a Study action to try to escape. When it does so, it makes a DC 20 Intelligence (Investigation)
    check. If it succeeds, it escapes, and the spell ends. When the spell ends, the target reappears in
    the space it left or, if that space is occupied, in the nearest unoccupied space.'
  sha256: e871ac4be81993006da2ef841102d052e15f34957c9f2bd63593f84d6624b4f0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Maze

<!-- SRD page 148 -->

Level 8 Conjuration (Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You banish a creature that you can see within range\
into a labyrinthine demiplane. The target remains\
there for the duration or until it escapes the maze.\
The target can take a Study action to try to es-\
cape. When it does so, it makes a DC 20 Intelligence\
(Investigation) check. If it succeeds, it escapes, and\
the spell ends.\
When the spell ends, the target reappears in the\
space it left or, if that space is occupied, in the near-\
est unoccupied space.\

Original layout: [[../../srd-5.2.1/pages/148|page 148]].
