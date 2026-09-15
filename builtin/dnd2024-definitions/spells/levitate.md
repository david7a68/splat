---
splat_schema: 1
splat_id: fa97a2c5-10f1-5117-95cd-2cdbb3d2e4a8
splat_kind: spell
splat_key: dnd2024:spells/levitate
splat_name: Levitate
splat_origin:
  key: dnd2024:reference/spell/levitate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 144
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a metal spring
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/levitate
  implementation: dnd2024:spell_levitate
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 600
    target:
      choices:
      - creature
      - loose_object
      visible: true
      range_feet: 60
      object_weight_pounds_max: 500
    resistance:
      only_if: unwilling_creature
      save:
        ability: constitution
        dc: spell_save_dc
      success: unaffected
    initial_motion:
      direction: up
      distance_feet_max: 20
      suspended_after: true
    target_movement:
      requires: fixed_object_or_surface_in_reach
      method: push_or_pull
      movement_rule: climbing
    caster_altitude_change:
      timing: caster_turn
      distance_feet_max: 20
      directions:
      - up
      - down
      self_target:
        economy: part_of_movement
      other_target:
        economy: magic_action
        target_must_remain_within_range_feet: 60
    spell_end_aloft:
      movement: float_gently_to_ground
    target_binding:
      exactly_one:
      - target_creature
      - target_object
      must_match_selected_mode: true
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_levitate
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    movement_choice:
      required: false
      type: choice
    target_creature:
      required: false
      type: participant
    target_object:
      required: false
      type: object
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - movement
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 0509baa0a14e62bba4fde8b271dd14cf96da481d4d6e92bd2cf4b01443d8aaad
  source_path: builtin/srd-5.2.1/spell/levitate.md
  pages:
  - 144
  engine_gaps: &id001
  - dnd2024:spell_levitate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: d37159b19946fd83ba22b77b1a1c174f8bce71e818e79bbf669756b8bc868e60
splat_clauses:
- key: source_rules
  text: '# Levitate Level 2 Transmutation (Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S, M (a metal spring) Duration: Concentration, up to 10 minutes One creature or loose object of
    your choice that you can see within range rises vertically up to 20 feet and remains suspended there
    for the duration. The spell can levitate an object that weighs up to 500 pounds. An unwilling creature
    that succeeds on a Constitution saving throw is unaffected. The target can move only by pushing or
    pulling against a fixed object or surface within reach (such as a wall or a ceiling), which allows
    it to move as if it were climbing. You can change the target’s altitude by up to 20 feet in either
    direction on your turn. If you are the target, you can move up or down as part of your move. Otherwise,
    you can take a Magic action to move the target, which must remain within the spell’s range. When the
    spell ends, the target floats gently to the ground if it is still aloft.'
  sha256: 7f6a1bc8576be1a7b4d7f3c119f15ff7668017a0af87c1f20b6be60c0d85b79d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Levitate

<!-- SRD page 144 -->

Level 2 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a metal spring)\
Duration: Concentration, up to 10 minutes\
One creature or loose object of your choice that you\
can see within range rises vertically up to 20 feet\
and remains suspended there for the duration. The\
spell can levitate an object that weighs up to 500\
pounds. An unwilling creature that succeeds on a\
Constitution saving throw is unaffected.\
The target can move only by pushing or pulling\
against a fixed object or surface within reach (such\
as a wall or a ceiling), which allows it to move as if it\
were climbing. You can change the target’s altitude\
by up to 20 feet in either direction on your turn. If\
you are the target, you can move up or down as part\
of your move. Otherwise, you can take a Magic ac-\
tion to move the target, which must remain within\
the spell’s range.\
When the spell ends, the target floats gently to the\
ground if it is still aloft.\

Original layout: [[../../srd-5.2.1/pages/144|page 144]].
