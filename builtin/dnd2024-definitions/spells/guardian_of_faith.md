---
splat_schema: 1
splat_id: 19de9a9b-8f93-58bd-9a7d-2b61d133bed4
splat_kind: spell
splat_key: dnd2024:spells/guardian_of_faith
splat_name: Guardian of Faith
splat_origin:
  key: dnd2024:reference/spell/guardian_of_faith
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: conjuration
  classes:
  - cleric
  components:
  - verbal
  concentration: false
  duration: 8 hours
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/guardian_of_faith
  implementation: dnd2024:spell_guardian_of_faith
  parameters:
    creation:
      size: large
      kind: spectral_guardian
      space:
        unoccupied: true
        visible: true
        within_spell_range: true
      hover: true
      occupies_space: true
      invulnerable: true
      duration: spell_duration
      appearance_constraint: appropriate_to_caster_deity_or_pantheon
    targets:
      relation: enemy
      distance_from_guardian_maximum_feet: 10
    triggers:
    - moves_into_distance_first_time_on_turn
    - starts_turn_in_distance
    save:
      ability: dexterity
      dc:
        binding: save_dc
      failure:
        damage:
          amount: 20
          type: radiant
      success:
        damage:
          amount: 10
          type: radiant
    vanish_when:
      total_damage_dealt_minimum: 60
splat_contracts:
- id: dnd2024:spell_guardian_of_faith
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
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - damage
  - participants
  events:
  - spell.cast_completed
  - turn.start
  - effect.ended
  - movement.completed
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - spatial_relationships
  - saving_throws
  - participant_lifecycle
splat_conversion:
  status: specified
  category: spell
  source_sha256: 58a49ddc4588083efed0150340263988cba5c920ff67cfaa3b2e9d39bdade6b2
  source_path: builtin/srd-5.2.1/spell/guardian_of_faith.md
  pages:
  - 137
  engine_gaps: &id001
  - dnd2024:spell_guardian_of_faith
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 21e512d5d07100c3ffd5e9321680ed6c2958f896eeda06db1d24f388b25520b9
splat_clauses:
- key: source_rules
  text: '# Guardian of Faith Level 4 Conjuration (Cleric) Casting Time: Action Range: 30 feet Components:
    V Duration: 8 hours A Large spectral guardian appears and hovers for the duration in an unoccupied
    space that you can see within range. The guardian occupies that space and is invulnerable, and it
    appears in a form appropriate for your deity or pantheon. Any enemy that moves to a space within 10
    feet of the guardian for the first time on a turn or starts its turn there makes a Dexterity saving
    throw, taking 20 Radiant damage on a failed save or half as much damage on a successful one. The guardian
    vanishes when it has dealt a total of 60 damage.'
  sha256: 1ee947951e2948812138fa308413e70b9584b6e6c71989e2a2584f1adcec329c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Guardian of Faith

<!-- SRD page 137 -->

Level 4 Conjuration (Cleric)\
Casting Time: Action\
Range: 30 feet\
Components: V\
Duration: 8 hours\
A Large spectral guardian appears and hovers for\
the duration in an unoccupied space that you can\
see within range. The guardian occupies that space\
and is invulnerable, and it appears in a form appro-\
priate for your deity or pantheon.\
Any enemy that moves to a space within 10 feet of\
the guardian for the first time on a turn or starts its\
turn there makes a Dexterity saving throw, taking\
20 Radiant damage on a failed save or half as much\
damage on a successful one. The guardian vanishes\
when it has dealt a total of 60 damage.\

Original layout: [[../../srd-5.2.1/pages/137|page 137]].
