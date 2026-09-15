---
splat_schema: 1
splat_id: c53c534a-c02f-5da4-a876-46a1d9d6b4b7
splat_kind: spell
splat_key: dnd2024:spells/mirror_image
splat_name: Mirror Image
splat_origin:
  key: dnd2024:reference/spell/mirror_image
splat_sources:
- title: SRD 5.2.1
  locator: pp. 150
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: illusion
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/mirror_image
  implementation: dnd2024:spell_mirror_image
  parameters:
    duplicates:
      initial_count: 3
      same_space: true
      follow_and_mimic: true
      indistinguishable: true
    trigger:
      event: attack.hit
      target: caster
      roll:
        dice_per_remaining_duplicate: 1
        sides: 6
      if_any_die_gte: 3
      replace_hit_target: one_duplicate
      destroy_hit_duplicate: true
    duplicates_ignore: all_other_damage_and_effects
    end:
      remaining_duplicates: 0
    attacker_bypass:
      any:
      - blinded
      - blindsight
      - truesight
splat_contracts:
- id: dnd2024:spell_mirror_image
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spell_attack_modifier:
      required: false
      type: integer
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
  - duplicate_pool
  - redirected_hit
  events:
  - spell.cast_completed
  - attack.hit
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - conditions
splat_conversion:
  status: specified
  category: spell
  source_sha256: 595c87eb9779604567054b62607d85e986ada9de63976fb28aa6f37d11417a40
  source_path: builtin/srd-5.2.1/spell/mirror_image.md
  pages:
  - 150
  engine_gaps: &id001
  - dnd2024:spell_mirror_image
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: c01366ca5aa9e8078cfe0c7691c8ebc74455493f5ae4ad4e4954a3a4a7b45d6c
splat_clauses:
- key: source_rules
  text: '# Mirror Image Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    Self Components: V, S Duration: 1 minute Three illusory duplicates of yourself appear in your space.
    Until the spell ends, the duplicates move with you and mimic your actions, shifting position so it’s
    impossible to track which image is real. Each time a creature hits you with an attack roll during
    the spell’s duration, roll a d6 for each of your remaining duplicates. If any of the d6s rolls a 3
    or higher, one of the duplicates is hit instead of you, and the duplicate is destroyed. The duplicates
    otherwise ignore all other damage and effects. The spell ends when all three duplicates are destroyed.
    A creature is unaffected by this spell if it has the Blinded condition, Blindsight, or Truesight.'
  sha256: 51c6e5d6f9a8d937df6a90b5b573929bcb11ca5dc27add0dc4bf8238f344cc97
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mirror Image

<!-- SRD page 150 -->

Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: 1 minute\
Three illusory duplicates of yourself appear in your\
space. Until the spell ends, the duplicates move with\
you and mimic your actions, shifting position so it’s\
impossible to track which image is real.\
Each time a creature hits you with an attack roll\
during the spell’s duration, roll a d6 for each of\
your remaining duplicates. If any of the d6s rolls\
a 3 or higher, one of the duplicates is hit instead of\
you, and the duplicate is destroyed. The duplicates\
otherwise ignore all other damage and effects. The\
spell ends when all three duplicates are destroyed.\
A creature is unaffected by this spell if it has the\
Blinded condition, Blindsight, or Truesight.\

Original layout: [[../../srd-5.2.1/pages/150|page 150]].
