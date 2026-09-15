---
splat_schema: 1
splat_id: fc71ebbf-7389-5ddc-9c61-c6ca3fce7978
splat_kind: spell
splat_key: dnd2024:spells/compulsion
splat_name: Compulsion
splat_origin:
  key: dnd2024:reference/spell/compulsion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 117
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: enchantment
  classes:
  - bard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/compulsion
  implementation: dnd2024:spell_compulsion
  parameters:
    target:
      kind: creature
      count: chosen_all
      visible_to_caster: true
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        condition: charmed
      success:
        effect: none
    command:
      action: bonus_action
      direction: caster_chosen_horizontal
      targets: charmed_targets
      timing: next_target_turn
      movement: as_much_as_possible
      route: safest
      after_movement_save:
        ability: wisdom
        success: end_spell_on_self
splat_contracts:
- id: dnd2024:spell_compulsion
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
  - charmed
  - movement_command
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - saving_throws
  - conditions
  - movement
  - choices
  - time
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5c232aa23c88bac0fdb38dd4323512631a30cb39a9e4fc15ef137478f716941b
  source_path: builtin/srd-5.2.1/spell/compulsion.md
  pages:
  - 117
  engine_gaps: &id001
  - dnd2024:spell_compulsion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 4396102a028f5649ba5780e9ebd035eb70fc03aa4d96cc305d9ee984c849febf
splat_clauses:
- key: source_rules
  text: '# Compulsion Level 4 Enchantment (Bard) Casting Time: Action Range: 30 feet Components: V, S
    Duration: Concentration, up to 1 minute Each creature of your choice that you can see within range
    must succeed on a Wisdom saving throw or have the Charmed condition until the spell ends. For the
    duration, you can take a Bonus Action to designate a direction that is horizontal to you. Each Charmed
    target must use as much of its movement as possible to move in that direction on its next turn, taking
    the safest route. After moving in this way, a target repeats the save, ending the spell on itself
    on a success.'
  sha256: 55384efbd2997ffc3c8ffc43ad63da253e87d924233cca58a6a98d4b8f7765f5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Compulsion

<!-- SRD page 117 -->

Level 4 Enchantment (Bard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
Each creature of your choice that you can see within\
range must succeed on a Wisdom saving throw or\
have the Charmed condition until the spell ends.\
For the duration, you can take a Bonus Action to\
designate a direction that is horizontal to you. Each\
Charmed target must use as much of its movement\
as possible to move in that direction on its next\
turn, taking the safest route. After moving in this\
way, a target repeats the save, ending the spell on\
itself on a success.\

Original layout: [[../../srd-5.2.1/pages/117|page 117]].
