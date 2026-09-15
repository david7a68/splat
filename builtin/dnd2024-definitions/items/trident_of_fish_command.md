---
splat_schema: 1
splat_id: ab80a986-fd3b-5196-a212-7af4d73565e0
splat_kind: item
splat_key: dnd2024:items/trident_of_fish_command
splat_name: Trident of Fish Command
splat_origin:
  key: dnd2024:reference/magic_item/trident_of_fish_command
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/trident_of_fish_command
  implementation: dnd2024:item_trident_of_fish_command
  parameters:
    equipment_state: carried
    charges:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          expression: 1d3
          scope: use
        cap: maximum
    spell_grants:
    - spell:
        key: dnd2024:spells/dominate_beast
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      save_dc: 15
      target_creature_types:
      - beast
      target_requires_swim_speed: true
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_trident_of_fish_command
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
  - spell.cast_pending
  - time.dawn
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - roll_resolution
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: c26ea0d618af9a2b43796d3e69102f2618e2962ea4acc8fab8c4f9e81c93093f
  source_path: builtin/srd-5.2.1/magic_item/trident_of_fish_command.md
  pages:
  - 250
  engine_gaps: &id001
  - dnd2024:item_trident_of_fish_command
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 1ee5d89594a85112853f2b6b3470cf7c23cc7a192e7fc8dc9a979ec1d2bd4e6f
splat_clauses:
- key: source_rules
  text: '# Trident of Fish Command Weapon (Trident), Uncommon (Requires Attunement) This magic weapon
    has 3 charges, and it regains 1d3 expended charges daily at dawn. While you carry it, you can expend
    1 charge to cast Dominate Beast (save DC 15) from it on a Beast that has a Swim Speed.'
  sha256: d881d718ad088ea18b99975967cdd1f0eafc3c0c2daf8f5e618382849ff3939d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Trident of Fish Command

<!-- SRD page 250 -->

Weapon (Trident), Uncommon (Requires Attunement)\
This magic weapon has 3 charges, and it regains\
1d3 expended charges daily at dawn. While you\
carry it, you can expend 1 charge to cast Dominate\
Beast (save DC 15) from it on a Beast that has a\
Swim Speed.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]].
