---
splat_schema: 1
splat_id: 1c31d842-028f-537a-8aa5-09a959077099
splat_kind: item
splat_key: dnd2024:items/ring_of_evasion
splat_name: Ring of Evasion
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_evasion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_evasion
  implementation: dnd2024:item_ring_of_evasion
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
    reaction:
      trigger: failed_dexterity_saving_throw
      charge_cost: 1
      replace_outcome: success
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rings
splat_contracts:
- id: dnd2024:item_ring_of_evasion
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
  - saving_throw.failed
  - time.dawn
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - resources_and_time
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 64c6c74d8b84e8d5e66258577397e52637ac79c0762b9db032987e23a23ea874
  source_path: builtin/srd-5.2.1/magic_item/ring_of_evasion.md
  pages:
  - 238
  engine_gaps: &id001
  - dnd2024:item_ring_of_evasion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 3b64a2bfb7679040dbce9c7334bd40b2d8c93d6f0dee8283ee55cde275cae76f
splat_clauses:
- key: source_rules
  text: '# Ring of Evasion Ring, Rare (Requires Attunement) This ring has 3 charges, and it regains 1d3
    expended charges daily at dawn. When you fail a Dexterity saving throw while wearing the ring, you
    can take a Reaction to expend 1 charge to succeed on that save instead.'
  sha256: 6d79a2d4579e4e7e31a295db90e1f8e25c2a0762370aebeea993383d8a2ad661
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Evasion

<!-- SRD page 238 -->

Ring, Rare (Requires Attunement)\
This ring has 3 charges, and it regains 1d3 ex-\
pended charges daily at dawn. When you fail a Dex-\
terity saving throw while wearing the ring, you can\
take a Reaction to expend 1 charge to succeed on\
that save instead.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].
