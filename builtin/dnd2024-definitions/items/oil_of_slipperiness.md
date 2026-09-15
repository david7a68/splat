---
splat_schema: 1
splat_id: 80a04f46-be96-5809-b662-9b2bdaeee518
splat_kind: item
splat_key: dnd2024:items/oil_of_slipperiness
splat_name: Oil of Slipperiness
splat_origin:
  key: dnd2024:reference/magic_item/oil_of_slipperiness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233, 234
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/oil_of_slipperiness
  implementation: dnd2024:item_oil_of_slipperiness
  parameters:
    consume: apply_or_pour
    apply:
      duration:
        amount: 10
        unit: minute
      target: creature_and_worn_carried_equipment
      vials_by_size:
        tiny: 1
        small: 1
        medium: 1
        large: 2
        huge: 3
        gargantuan: 4
      spell_effect:
        key: dnd2024:spells/freedom_of_movement
      duration_effect:
        amount: 8
        unit: hour
    pour:
      action: magic
      area:
        shape: square
        side_ft: 10
      spell_effect:
        key: dnd2024:spells/grease
      duration_effect:
        amount: 8
        unit: hour
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_potions
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_oil_of_slipperiness
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
  - item.applied
  - action.magic_pending
  - time.elapsed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 41f4d66d46d293bb2d658eda403f736cca9264ceccc90f75d733bcad7d465df1
  source_path: builtin/srd-5.2.1/magic_item/oil_of_slipperiness.md
  pages:
  - 233
  - 234
  engine_gaps: &id001
  - dnd2024:item_oil_of_slipperiness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: c91dd29e1511d974ae93910b88c167388e78e73bfab209bfc84a8da6f151c2d0
splat_clauses:
- key: source_rules
  text: '# Oil of Slipperiness Potion, Uncommon One vial of this oil can cover one Medium or smaller creature,
    along with the equipment it’s wearing and carrying (one additional vial is required for each size
    category above Medium). Applying the oil takes 10 minutes. The affected creature then gains the effect
    of the Freedom of Movement spell for 8 hours. Alternatively, the oil can be poured on the ground as
    a Magic action, where it covers a 10-foot square, duplicating the effect of the Grease spell in that
    area for 8 hours. This sticky, black unguent is thick and heavy, but it flows quickly when poured.'
  sha256: f0aa4edb94132ca2a31b4e81a9f04f3b28a1a4f15e367c472886af37a98608d9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Oil of Slipperiness

<!-- SRD page 233 -->

Potion, Uncommon\
One vial of this oil can cover one Medium or smaller\
creature, along with the equipment it’s wearing and\
carrying (one additional vial is required for each\
size category above Medium). Applying the oil takes\
10 minutes. The affected creature then gains the ef-\
fect of the Freedom of Movement spell for 8 hours.\
Alternatively, the oil can be poured on the ground\
as a Magic action, where it covers a 10-foot square,\

<!-- SRD page 234 -->

duplicating the effect of the Grease spell in that area\
for 8 hours.\
This sticky, black unguent is thick and heavy, but\
it flows quickly when poured.\

Original layout: [[../../srd-5.2.1/pages/233|page 233]], [[../../srd-5.2.1/pages/234|page 234]].
