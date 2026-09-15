---
splat_schema: 1
splat_id: 27ea59e2-4ed9-586f-a6ce-133cf8776339
splat_kind: item
splat_key: dnd2024:items/oil_of_etherealness
splat_name: Oil of Etherealness
splat_origin:
  key: dnd2024:reference/magic_item/oil_of_etherealness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/oil_of_etherealness
  implementation: dnd2024:item_oil_of_etherealness
  parameters:
    application_duration:
      amount: 10
      unit: minutes
    vials_by_target_size:
      tiny: 1
      small: 1
      medium: 1
      large: 2
      huge: 3
      gargantuan: 4
    covers:
    - creature
    - worn_equipment
    - carried_equipment
    consume_applied_vials: true
    effect:
      spell:
        key: dnd2024:spells/etherealness
      duration:
        amount: 1
        unit: hours
    attunement_required: false
splat_contracts:
- id: dnd2024:item_oil_of_etherealness
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
  - item.application_completed
  capabilities:
  - item_effects
  - item_spellcasting
  - effect_duration
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: b608dbd9b46cb79fb284eeb7068acd55c0684f4a3acd17e097fc1f4df9f2d59f
  source_path: builtin/srd-5.2.1/magic_item/oil_of_etherealness.md
  pages:
  - 233
  engine_gaps: &id001
  - dnd2024:item_oil_of_etherealness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 9598958c22e11317a0ad89983fa0b11a3e916e7ccc55f4379b9506d898cfc2ff
splat_clauses:
- key: source_rules
  text: '# Oil of Etherealness Potion, Rare One vial of this oil can cover one Medium or smaller creature,
    along with the equipment it’s wearing and carrying (one additional vial is required for each size
    category above Medium). Applying the oil takes 10 minutes. The affected creature then gains the effect
    of the Etherealness spell for 1 hour. Beads of this cloudy, gray oil form on the outside of its container
    and quickly evaporate.'
  sha256: 3069d1ff199f14bd64697b24fc17f0dbe98400944f8daec98746cc1743fc10ba
  status: contract_specified
splat_requires_mechanics: *id001
---

# Oil of Etherealness

<!-- SRD page 233 -->

Potion, Rare\
One vial of this oil can cover one Medium or smaller\
creature, along with the equipment it’s wearing and\
carrying (one additional vial is required for each\
size category above Medium). Applying the oil takes\
10 minutes. The affected creature then gains the\
effect of the Etherealness spell for 1 hour.\
Beads of this cloudy, gray oil form on the outside\
of its container and quickly evaporate.\

Original layout: [[../../srd-5.2.1/pages/233|page 233]].
