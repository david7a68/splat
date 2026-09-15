---
splat_schema: 1
splat_id: 99965f86-2526-5f91-9b40-1114afbf6486
splat_kind: item
splat_key: dnd2024:items/belt_of_dwarvenkind
splat_name: Belt of Dwarvenkind
splat_origin:
  key: dnd2024:reference/magic_item/belt_of_dwarvenkind
splat_sources:
- title: SRD 5.2.1
  locator: pp. 213
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/belt_of_dwarvenkind
  implementation: dnd2024:item_belt_of_dwarvenkind
  parameters:
    equipment_state: worn
    language: dwarvish
    checks:
    - ability: charisma
      skill: persuasion
      target_ancestries:
      - dwarf
      - duergar
      advantage: true
    ability:
      constitution:
        increase: 2
        maximum: 20
    beard:
      requires: attuned
      event: dawn
      chance_percent: 50
      requires_can_grow_beard: true
      result_by_existing_beard:
        absent: full
        present: thicker
    non_dwarf_non_duergar:
      darkvision_ft: 60
      damage_resistance: poison
      save_advantage:
        avoid_or_end_condition: poisoned
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_belt_of_dwarvenkind
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
  - equipment.changed
  - attunement.changed
  - time.dawn
  - check.pending
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: bd964279067736863facb91fdd30aece8762c81e999abf2b47ad745d2b337e62
  source_path: builtin/srd-5.2.1/magic_item/belt_of_dwarvenkind.md
  pages:
  - 213
  engine_gaps: &id001
  - dnd2024:item_belt_of_dwarvenkind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: ade648369bf3bcb35f172fb7ff91a5220367f4abda08b10fa1b9724383249033
splat_clauses:
- key: source_rules
  text: '# Belt of Dwarvenkind Wondrous Item, Rare (Requires Attunement) While wearing this belt, you
    gain the following benefits: Dwarvish. You know Dwarvish. Friend of Dwarvenkind. You have Advantage
    on Charisma (Persuasion) checks made to interact with dwarves and duergar. Toughness. Your Constitution
    increases by 2, to a maximum of 20. In addition, while attuned to the belt, you have a 50 percent
    chance each day at dawn of growing a full beard if you can grow one, or a thicker beard if you already
    have one. If you aren’t a dwarf or duergar, you gain the following additional benefits while wearing
    the belt: Darkvision. You have Darkvision with a range of 60 feet. Resilience. You have Resistance
    to Poison damage. You also have Advantage on saving throws you make to avoid or end the Poisoned condition.'
  sha256: f9018c2355189a8abcc2114bdfda7dd47a9841b585fa4dd7b48c44670a6a089d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Belt of Dwarvenkind

<!-- SRD page 213 -->

Wondrous Item, Rare (Requires Attunement)\
While wearing this belt, you gain the following\
benefits:\
Dwarvish. You know Dwarvish.\
Friend of Dwarvenkind. You have Advantage on\
Charisma (Persuasion) checks made to interact\
with dwarves and duergar.\
Toughness. Your Constitution increases by 2, to a\
maximum of 20.\
In addition, while attuned to the belt, you have a 50\
percent chance each day at dawn of growing a full\
beard if you can grow one, or a thicker beard if you\
already have one.\
If you aren’t a dwarf or duergar, you gain the fol-\
lowing additional benefits while wearing the belt:\
Darkvision. You have Darkvision with a range of 60\
feet.\
Resilience. You have Resistance to Poison damage.\
You also have Advantage on saving throws you\
make to avoid or end the Poisoned condition.\

Original layout: [[../../srd-5.2.1/pages/213|page 213]].
