---
splat_schema: 1
splat_id: b69fb1ef-c538-59b7-a5a5-64ba0ec3e3a2
splat_kind: item
splat_key: dnd2024:items/ring_of_animal_influence
splat_name: Ring of Animal Influence
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_animal_influence
splat_sources:
- title: SRD 5.2.1
  locator: pp. 237
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/ring_of_animal_influence
  implementation: dnd2024:item_ring_of_animal_influence
  parameters:
    equipment_state: worn_on_finger_or_similar_digit
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
        key: dnd2024:spells/animal_friendship
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      save_dc: 13
    - spell:
        key: dnd2024:spells/fear
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      save_dc: 13
      target_creature_types:
      - beast
    - spell:
        key: dnd2024:spells/speak_with_animals
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      save_dc: 13
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rings
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_ring_of_animal_influence
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
  source_sha256: 2c50f5c993d9826836a56dae77537a6588cf645986c8ac6b7e28d3440764d384
  source_path: builtin/srd-5.2.1/magic_item/ring_of_animal_influence.md
  pages:
  - 237
  engine_gaps: &id001
  - dnd2024:item_ring_of_animal_influence
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 4e31cd7409c1986a4cf6dd09f1f0310b0bf36256f971c2a47facb6afd5ea8c38
splat_clauses:
- key: source_rules
  text: '# Ring of Animal Influence Ring, Rare This ring has 3 charges, and it regains 1d3 expended charges
    daily at dawn. While wearing the ring, you can expend 1 charge to cast one of the following spells
    (save DC 13) from it: • Animal Friendship • Fear (affects Beasts only) • Speak with Animals'
  sha256: f85aefa21de94950d43fd8b9f5a139c3f179657bfe2e396df4ecb8de62dcfa35
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Animal Influence

<!-- SRD page 237 -->

Ring, Rare\
This ring has 3 charges, and it regains 1d3 ex-\
pended charges daily at dawn. While wearing the\
ring, you can expend 1 charge to cast one of the fol-\
lowing spells (save DC 13) from it:\
•  Animal Friendship\
•  Fear (affects Beasts only)\
•  Speak with Animals\

Original layout: [[../../srd-5.2.1/pages/237|page 237]].
