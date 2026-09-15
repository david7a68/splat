---
splat_schema: 1
splat_id: c1fe9cc5-f0f5-582c-afee-9066245175eb
splat_kind: spell
splat_key: dnd2024:spells/time_stop
splat_name: Time Stop
splat_origin:
  key: dnd2024:reference/spell/time_stop
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: transmutation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/time_stop
  implementation: dnd2024:spell_time_stop
  parameters:
    time:
      other_creatures_elapsed_time: 0
      caster_consecutive_turns:
        dice:
          count: 1
          sides: 4
          add: 1
      caster_actions_and_movement: normal
    end_when:
      caster_action_or_created_effect_affects_any:
      - other_creature
      - object_worn_or_carried_by_other_creature
      caster_distance_from_cast_location_feet_greater_than: 1000
splat_contracts:
- id: dnd2024:spell_time_stop
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast_completed
  - effect.ended
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - dice
  - spatial_relationships
  - turn_scheduling
splat_conversion:
  status: specified
  category: spell
  source_sha256: 830520630a1b2e32539222aa38217784b56d709617e279985f2060aa70b0f4fb
  source_path: builtin/srd-5.2.1/spell/time_stop.md
  pages:
  - 169
  engine_gaps: &id001
  - dnd2024:spell_time_stop
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 2ba42a608f9355ff0902146d969bc14998c823942ac55485d66209761f43f8a8
splat_clauses:
- key: source_rules
  text: '# Time Stop Level 9 Transmutation (Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V Duration: Instantaneous You briefly stop the flow of time for everyone but yourself. No time passes
    for other creatures, while you take 1d4 + 1 turns in a row, during which you can use actions and move
    as normal. This spell ends if one of the actions you use during this period, or any effects that you
    create during it, affects a creature other than you or an object being worn or carried by someone
    other than you. In addition, the spell ends if you move to a place more than 1,000 feet from the location
    where you cast it.'
  sha256: 069593c83d7325c9da8b30a2902931828239bcc00bd959c7a3c47308ac3fc09d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Time Stop

<!-- SRD page 169 -->

Level 9 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Instantaneous\
You briefly stop the flow of time for everyone but\
yourself. No time passes for other creatures, while\
you take 1d4 + 1 turns in a row, during which you\
can use actions and move as normal.\
This spell ends if one of the actions you use during\
this period, or any effects that you create during it,\
affects a creature other than you or an object being\
worn or carried by someone other than you. In addi-\
tion, the spell ends if you move to a place more than\
1,000 feet from the location where you cast it.\

Original layout: [[../../srd-5.2.1/pages/169|page 169]].
