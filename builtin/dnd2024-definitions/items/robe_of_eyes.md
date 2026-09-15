---
splat_schema: 1
splat_id: 1c8ebf98-c2c6-5e3d-acaf-be0f0be519a4
splat_kind: item
splat_key: dnd2024:items/robe_of_eyes
splat_name: Robe of Eyes
splat_origin:
  key: dnd2024:reference/magic_item/robe_of_eyes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 240
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/robe_of_eyes
  implementation: dnd2024:item_robe_of_eyes
  parameters:
    equipped: worn
    checks:
    - ability: wisdom
      skill: perception
      requires: sight
      mode: advantage
    senses:
      darkvision_ft: 120
      truesight_ft: 120
    drawbacks:
    - trigger:
        spell:
          key: dnd2024:spells/light
        target: robe
      effect:
        condition: blinded
        duration:
          amount: 1
          unit: minutes
      repeat_save:
        ability: constitution
        dc: 11
        failure:
          continue: true
        success:
          end_effect: true
        repeat: target.turn_end
    - trigger:
        spell:
          key: dnd2024:spells/daylight
        distance_from_robe_ft_max: 5
      effect:
        condition: blinded
        duration:
          amount: 1
          unit: minutes
      repeat_save:
        ability: constitution
        dc: 15
        failure:
          continue: true
        success:
          end_effect: true
        repeat: target.turn_end
    attunement_required: true
splat_contracts:
- id: dnd2024:item_robe_of_eyes
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - spell.cast_completed
  - turn.ended
  - ability_check.pending
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
  - effect_duration
  - conditions
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: ca78af26470d78bdadfc5c0ea59e8b08e4fc2cab7bb05f60aea757bd4789f512
  source_path: builtin/srd-5.2.1/magic_item/robe_of_eyes.md
  pages:
  - 240
  engine_gaps: &id001
  - dnd2024:item_robe_of_eyes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 7020fef321275d971419d2fbb09f5b6d4cb72d036eb7b27b774d064cb3f54cf9
splat_clauses:
- key: source_rules
  text: '# Robe of Eyes Wondrous Item, Rare (Requires Attunement) This robe is adorned with eyelike patterns.
    While you wear the robe, you gain the following benefits: All-Around Vision. The robe gives you Advantage
    on Wisdom (Perception) checks that rely on sight. Special Senses. You have Darkvision and Truesight,
    both with a range of 120 feet. Drawbacks. A Light spell cast on the robe or a Daylight spell cast
    within 5 feet of the robe gives you the Blinded condition for 1 minute. At the end of each of your
    turns, you make a Constitution saving throw (DC 11 for Light or DC 15 for Daylight), ending the condition
    on yourself on a success.'
  sha256: 8c39ea4bd24d3fc271e6e6a46816c5bbe3b2014f7fda8be41cda3c410ccb8b7b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Robe of Eyes

<!-- SRD page 240 -->

Wondrous Item, Rare (Requires Attunement)\
This robe is adorned with eyelike patterns. While\
you wear the robe, you gain the following benefits:\
All-Around Vision. The robe gives you Advantage\
on Wisdom (Perception) checks that rely on sight.\
Special Senses. You have Darkvision and Truesight,\
both with a range of 120 feet.\
Drawbacks. A Light spell cast on the robe or a\
Daylight spell cast within 5 feet of the robe gives\
you the Blinded condition for 1 minute. At the end of\
each of your turns, you make a Constitution saving\
throw (DC 11 for Light or DC 15 for Daylight), ending\
the condition on yourself on a success.\

Original layout: [[../../srd-5.2.1/pages/240|page 240]].
