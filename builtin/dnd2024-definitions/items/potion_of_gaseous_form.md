---
splat_schema: 1
splat_id: 545e5e8e-ebf6-5af6-bf39-05c4a1ed43b2
splat_kind: item
splat_key: dnd2024:items/potion_of_gaseous_form
splat_name: Potion of Gaseous Form
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_gaseous_form
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_gaseous_form
  implementation: dnd2024:item_potion_of_gaseous_form
  parameters:
    consume: drink
    spell_effect:
      key: dnd2024:spells/gaseous_form
    duration:
      amount: 1
      unit: hour
    concentration: false
    end_early_action: bonus_action
    consume_action: bonus_action
    administer_action: bonus_action
    effect_target: drinker
    consumed_quantity: 1
    effect_starts: immediately_after_consumption
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_potions
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_potion_of_gaseous_form
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
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
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - item.consumed
  - action.bonus_pending
  - time.elapsed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 071691f0fe90d8b9f5fe3bee11a2f0c41c8fba9b047dd16d98c3de09715b91ac
  source_path: builtin/srd-5.2.1/magic_item/potion_of_gaseous_form.md
  pages:
  - 235
  engine_gaps: &id001
  - dnd2024:item_potion_of_gaseous_form
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 66d3ee7d620eb75187671e5b5e4167f85161f67e049623e88d37bb457412d2e6
splat_clauses:
- key: source_rules
  text: '# Potion of Gaseous Form Potion, Rare When you drink this potion, you gain the effect of the
    Gaseous Form spell for 1 hour (no Concentration required) or until you end the effect as a Bonus Action.
    This potion’s container seems to hold fog that moves and pours like water.'
  sha256: 23ada89f9af6c016cd2a6dce20418758c001b4ccd351678efdab44d01697318a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Gaseous Form

<!-- SRD page 235 -->

Potion, Rare\
When you drink this potion, you gain the effect of\
the Gaseous Form spell for 1 hour (no Concentration\
required) or until you end the effect as a Bonus\
Action.\
This potion’s container seems to hold fog that\
moves and pours like water.\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].
