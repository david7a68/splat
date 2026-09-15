---
splat_schema: 1
splat_id: 429ab81d-cc32-5595-aeeb-6901f5462992
splat_kind: spell
splat_key: dnd2024:spells/fear
splat_name: Fear
splat_origin:
  key: dnd2024:reference/spell/fear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 130
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: illusion
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a white feather
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/fear
  implementation: dnd2024:spell_fear
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    area:
      shape: cone
      length_feet: 30
      origin: caster
    targets:
      kind: creature
      selection: all_in_area
    save:
      ability: wisdom
      dc: spell_save_dc
      success: unaffected
    failure:
      drop: all_held_items
      condition: frightened
      source: caster
    while_frightened:
      timing: target_turn
      action: dash
      movement: away_from_caster
      route: safest
      exception: nowhere_to_move
      route_assumption: supplied_spatial_paths
    repeat_save:
      event: turn.end
      subject: target
      requires: no_line_of_sight_to_caster
      ability: wisdom
      dc: spell_save_dc
      success: end_on_target
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_fear
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    origin:
      type: location
      required: true
    safe_paths:
      type: resolution
      required: true
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
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - movement
  - behavior
  - conditions
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 1aab24a12291968bfd66880de429916d72e8b90499424a12950b2edf3bf779d3
  source_path: builtin/srd-5.2.1/spell/fear.md
  pages:
  - 130
  engine_gaps: &id001
  - dnd2024:spell_fear
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: b0c1696a91e0d84607e426222421887f366fac817909ef94e494f6da4fa9c2a0
splat_clauses:
- key: source_rules
  text: '# Fear Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range: Self Components:
    V, S, M (a white feather) Duration: Concentration, up to 1 minute Each creature in a 30-foot Cone
    must succeed on a Wisdom saving throw or drop whatever it is holding and have the Frightened condition
    for the duration. A Frightened creature takes the Dash action and moves away from you by the safest
    route on each of its turns unless there is nowhere to move. If the creature ends its turn in a space
    where it doesn’t have line of sight to you, the creature makes a Wisdom saving throw. On a successful
    save, the spell ends on that creature.'
  sha256: 291c65a310068b0626e424f30b2fa09797c4137d6c356c493382e10f7972d7ce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fear

<!-- SRD page 130 -->

Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a white feather)\
Duration: Concentration, up to 1 minute\
Each creature in a 30-foot Cone must succeed on a\
Wisdom saving throw or drop whatever it is holding\
and have the Frightened condition for the duration.\
A Frightened creature takes the Dash action and\
moves away from you by the safest route on each\
of its turns unless there is nowhere to move. If the\
creature ends its turn in a space where it doesn’t\
have line of sight to you, the creature makes a Wis-\
dom saving throw. On a successful save, the spell\
ends on that creature.\

Original layout: [[../../srd-5.2.1/pages/130|page 130]].
