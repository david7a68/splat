---
splat_schema: 1
splat_id: aa4e9ca8-b2d8-52e1-9d3b-35342f1a6965
splat_kind: item
splat_key: dnd2024:items/manual_of_golems
splat_name: Manual of Golems
splat_origin:
  key: dnd2024:reference/magic_item/manual_of_golems
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229, 230
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/manual_of_golems
  implementation: dnd2024:item_manual_of_golems
  parameters:
    eligibility:
      spellcaster: true
      level_5_spell_slots_at_least: 2
    ineligible_read_attempt:
      damage:
        amount:
          roll:
            count: 6
            sides: 6
        type: psychic
    type_selection: gm_choice_or_roll
    type_die: 20
    variants:
    - roll:
      - 1
      - 5
      creature:
        key: dnd2024:creatures/clay_golem
      work_days: 30
      cost_gp: 65000
    - roll:
      - 6
      - 17
      creature:
        key: dnd2024:creatures/flesh_golem
      work_days: 60
      cost_gp: 50000
    - roll:
      - 18
      - 18
      creature:
        key: dnd2024:creatures/iron_golem
      work_days: 120
      cost_gp: 100000
    - roll:
      - 19
      - 20
      creature:
        key: dnd2024:creatures/stone_golem
      work_days: 90
      cost_gp: 80000
    crafting:
      manual_at_hand: true
      uninterrupted: true
      maximum_daily_rest_hours: 8
      supplies_cost_required: true
      completion: consume_book_in_flames
      animate: sprinkle_book_ashes_on_golem
      control: creator
      understands_and_obeys_commands: true
    attunement_required: false
splat_contracts:
- id: dnd2024:item_manual_of_golems
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
  - item.read_requested
  - crafting.progressed
  - item.activation_requested
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - item_variants
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2ab085deb2ce84be08010749c9764ad65483bdb7fbab11e3a7c768417c2de6b0
  source_path: builtin/srd-5.2.1/magic_item/manual_of_golems.md
  pages:
  - 229
  - 230
  engine_gaps: &id001
  - dnd2024:item_manual_of_golems
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: d882ab0bccf36d9162e22cc3943543e5f0aab84b059a32bdd66d7225c566ac5d
splat_clauses:
- key: source_rules
  text: '# Manual of Golems Wondrous Item, Very Rare This tome contains information and incantations necessary
    to make a particular type of golem. The GM chooses the type or determines it randomly by rolling on
    the accompanying table. To decipher and use the manual, you must be a spellcaster with at least two
    level 5 spell slots. A creature that can’t use a Manual of Golems and attempts to read it takes 6d6
    Psychic damage. To create a golem, you must spend the time shown on the table, working without interruption
    with the manual at hand and resting no more than 8 hours per day. You must also pay the specified
    cost to purchase supplies. Once you finish creating the golem, the book is consumed in eldritch flames.
    The golem becomes animate when the ashes of the manual are sprinkled on it. See “Monsters” for the
    golem’s stat block. The golem is under your control, and it understands and obeys your commands. 1d20
    Golem Time Cost 1–5 Clay Golem 30 days 65,000 GP 6–17 Flesh Golem 60 days 50,000 GP 18 Iron Golem
    120 days 100,000 GP 19–20 Stone Golem 90 days 80,000 GP'
  sha256: 5534ecec328b4e9069d1ac0174f66bfe4200f290ec652cf52e0f0fe43f3b2a8d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Manual of Golems

<!-- SRD page 229 -->

Wondrous Item, Very Rare\
This tome contains information and incantations\
necessary to make a particular type of golem. The\
GM chooses the type or determines it randomly by\
rolling on the accompanying table. To decipher and\
use the manual, you must be a spellcaster with at\
least two level 5 spell slots. A creature that can’t\
use a Manual of Golems and attempts to read it takes\
6d6 Psychic damage.\
To create a golem, you must spend the time shown\
on the table, working without interruption with the\
manual at hand and resting no more than 8 hours\
per day. You must also pay the specified cost to pur-\
chase supplies.\
Once you finish creating the golem, the book is\
consumed in eldritch flames. The golem becomes\
animate when the ashes of the manual are sprinkled\
on it. See “Monsters” for the golem’s stat block. The\
golem is under your control, and it understands and\
obeys your commands.\

<!-- SRD page 230 -->

1d20 Golem Time Cost\
1–5 Clay Golem 30 days 65,000 GP\
6–17 Flesh Golem 60 days 50,000 GP\
18 Iron Golem 120 days 100,000 GP\
19–20 Stone Golem 90 days 80,000 GP\

Original layout: [[../../srd-5.2.1/pages/229|page 229]], [[../../srd-5.2.1/pages/230|page 230]].
