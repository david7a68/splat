---
splat_schema: 1
splat_id: 0a96f42b-3b27-57e4-a112-4553a424129b
splat_kind: spell
splat_key: dnd2024:spells/scorching_ray
splat_name: Scorching Ray
splat_origin:
  key: dnd2024:reference/spell/scorching_ray
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/scorching_ray
  implementation: dnd2024:spell_scorching_ray
  parameters:
    rays:
      base_count: 3
      target_assignment: same_or_different_targets_in_range
      attack_per_ray: ranged_spell
      hit:
        damage:
          amount:
            dice:
              count: 2
              sides: 6
              add: 0
          type: fire
    scaling:
      per_slot_above: 2
      additional_rays: 1
splat_contracts:
- id: dnd2024:spell_scorching_ray
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spell_attack_modifier:
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
  - attack_results
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
splat_conversion:
  status: specified
  category: spell
  source_sha256: db0da91f991d184b9de36b5a7d0fc02c8e1f0571f35c5c42ab75065c7ca71eac
  source_path: builtin/srd-5.2.1/spell/scorching_ray.md
  pages:
  - 159
  engine_gaps: &id001
  - dnd2024:spell_scorching_ray
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 9c1ddb05a94812d620e32548600841c1a48e600829602655609b8014e57482d6
splat_clauses:
- key: source_rules
  text: '# Scorching Ray Level 2 Evocation (Sorcerer, Wizard) Casting Time: Action Range: 120 feet Components:
    V, S Duration: Instantaneous You hurl three fiery rays. You can hurl them at one target within range
    or at several. Make a ranged spell attack for each ray. On a hit, the target takes 2d6 Fire damage.
    Using a Higher-Level Spell Slot. You create one additional ray for each spell slot level above 2.'
  sha256: fca3cac851622c8ac1eafeb89b749352180fa712a6c9f7ef4e009823cbf004d9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scorching Ray

<!-- SRD page 159 -->

Level 2 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous\
You hurl three fiery rays. You can hurl them at one\
target within range or at several. Make a ranged\
spell attack for each ray. On a hit, the target takes\
2d6 Fire damage.\
Using a Higher-Level Spell Slot. You create one\
additional ray for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].
