---
splat_schema: 1
splat_id: e8dc8a9c-9430-5531-ae03-9cc6200909a3
splat_kind: item
splat_key: dnd2024:items/dust_of_sneezing_and_choking
splat_name: Dust of Sneezing and Choking
splat_origin:
  key: dnd2024:reference/magic_item/dust_of_sneezing_and_choking
splat_sources:
- title: SRD 5.2.1
  locator: pp. 219
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/dust_of_sneezing_and_choking
  implementation: dnd2024:item_dust_of_sneezing_and_choking
  parameters:
    uses: 1
    identify_false_result:
      key: dnd2024:items/dust_of_disappearance
    appearance_matches:
      key: dnd2024:items/dust_of_disappearance
    activation:
      action: utilize
      consume: true
      area:
        shape: emanation
        radius_ft: 30
        origin: user
        include_user: true
      save:
        ability: constitution
        dc: 15
        failure:
          condition: incapacitated
          suffocating: true
        success:
          effect: none
        automatic_success_types:
        - construct
        - elemental
        - ooze
        - plant
        - undead
    repeat_save:
      event: target_turn_end
      ability: constitution
      dc: 15
      success: end_effect
    end_on_spell_targeting:
    - key: dnd2024:spells/lesser_restoration
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_dust_of_sneezing_and_choking
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
  - action.utilize_pending
  - turn.ended
  - spell.cast_completed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - roll_resolution
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 593d8ba77d98c274e9c3dd794fee2c8545dca8ffe4c1f15549b9434b782bc37e
  source_path: builtin/srd-5.2.1/magic_item/dust_of_sneezing_and_choking.md
  pages:
  - 219
  engine_gaps: &id001
  - dnd2024:item_dust_of_sneezing_and_choking
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: e66de5abdec0d3b476eac82e392651bcc360acba79a8f26eb2c5620442e4b6a7
splat_clauses:
- key: source_rules
  text: '# Dust of Sneezing and Choking Wondrous Item, Uncommon Found in a small container, this powder
    resembles Dust of Disappearance, and Identify reveals it to be such. There is enough of it for one
    use. As a Utilize action, you can throw the dust into the air, forcing yourself and every creature
    in a 30-foot Emanation originating from you to make a DC 15 Constitution saving throw. Constructs,
    Elementals, Oozes, Plants, and Undead succeed on the save automatically. On a failed save, a creature
    begins sneezing uncontrollably; it has the Incapacitated condition and is suffocating. The creature
    repeats the save at the end of each of its turns, ending the effect on itself on a success. The effect
    also ends on any creature targeted by a Lesser Restoration spell.'
  sha256: b2b06fb07ee23ff977b35410dd6b3ea80f831a8341bd8fd0ee8ce67399e40569
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dust of Sneezing and Choking

<!-- SRD page 219 -->

Wondrous Item, Uncommon\
Found in a small container, this powder resembles\
Dust of Disappearance, and Identify reveals it to be\
such. There is enough of it for one use.\
As a Utilize action, you can throw the dust into the\
air, forcing yourself and every creature in a 30-foot\
Emanation originating from you to make a DC 15\
Constitution saving throw. Constructs, Elementals,\
Oozes, Plants, and Undead succeed on the save\
automatically.\
On a failed save, a creature begins sneezing un-\
controllably; it has the Incapacitated condition and\
is suffocating. The creature repeats the save at the\
end of each of its turns, ending the effect on itself on\
a success. The effect also ends on any creature tar-\
geted by a Lesser Restoration spell.\

Original layout: [[../../srd-5.2.1/pages/219|page 219]].
