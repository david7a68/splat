---
splat_schema: 1
splat_id: 662dd9da-c049-5604-8398-9c2809fde868
splat_kind: spell
splat_key: dnd2024:spells/command
splat_name: Command
splat_origin:
  key: dnd2024:reference/spell/command
splat_sources:
- title: SRD 5.2.1
  locator: pp. 116
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - bard
  - cleric
  - paladin
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/command
  implementation: dnd2024:spell_command
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: true
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        execute_command: next_target_turn
      success:
        effect: none
    choice:
      chooser: caster
      words: 1
      options:
      - approach
      - drop
      - flee
      - grovel
      - halt
    commands:
      approach:
        movement: toward_caster
        route: shortest_most_direct
        end_turn_within_feet: 5
      drop:
        drop: all_held_objects
        then: end_turn
      flee:
        whole_turn: move_away_from_caster
        means: fastest_available
      grovel:
        condition: prone
        then: end_turn
      halt:
        movement: false
        action: false
        bonus_action: false
    scaling:
      per_slot_above: 1
      additional_targets: 1
splat_contracts:
- id: dnd2024:spell_command
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - saves
  - next_turn_command
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - saving_throws
  - conditions
  - movement
  - inventory
  - choices
  - time
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 35fdb4cbb0e393cc1def0de68d4e1385220aaaeb5ad5e603085b68286b638ab6
  source_path: builtin/srd-5.2.1/spell/command.md
  pages:
  - 116
  engine_gaps: &id001
  - dnd2024:spell_command
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: c8ec4be138dde63d2a31b3790817da4c6ed464a1d4c9955646c7f0184c76e32b
splat_clauses:
- key: source_rules
  text: '# Command Level 1 Enchantment (Bard, Cleric, Paladin) Casting Time: Action Range: 60 feet Components:
    V Duration: Instantaneous You speak a one-word command to a creature you can see within range. The
    target must succeed on a Wisdom saving throw or follow the command on its next turn. Choose the command
    from these options: Approach. The target moves toward you by the shortest and most direct route, ending
    its turn if it moves within 5 feet of you. Drop. The target drops whatever it is holding and then
    ends its turn. Flee. The target spends its turn moving away from you by the fastest available means.
    Grovel. The target has the Prone condition and then ends its turn. Halt. On its turn, the target doesn’t
    move and takes no action or Bonus Action. Using a Higher-Level Spell Slot. You can affect one additional
    creature for each spell slot level above 1.'
  sha256: f5f67c0cfcbfa55f2d5e6788645c1cb96d0a1b53db09cf66562786738c7e3134
  status: contract_specified
splat_requires_mechanics: *id001
---

# Command

<!-- SRD page 116 -->

Level 1 Enchantment (Bard, Cleric, Paladin)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous\
You speak a one-word command to a creature you\
can see within range. The target must succeed on a\
Wisdom saving throw or follow the command on its\
next turn. Choose the command from these options:\
Approach. The target moves toward you by the\
shortest and most direct route, ending its turn if it\
moves within 5 feet of you.\
Drop. The target drops whatever it is holding and\
then ends its turn.\
Flee. The target spends its turn moving away from\
you by the fastest available means.\
Grovel. The target has the Prone condition and then\
ends its turn.\
Halt. On its turn, the target doesn’t move and takes\
no action or Bonus Action.\
Using a Higher-Level Spell Slot. You can affect\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/116|page 116]].
