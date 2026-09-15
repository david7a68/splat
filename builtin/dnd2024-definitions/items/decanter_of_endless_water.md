---
splat_schema: 1
splat_id: 8ac511d2-540a-5027-8272-27e55c9c2a8d
splat_kind: item
splat_key: dnd2024:items/decanter_of_endless_water
splat_name: Decanter of Endless Water
splat_origin:
  key: dnd2024:reference/magic_item/decanter_of_endless_water
splat_sources:
- title: SRD 5.2.1
  locator: pp. 217
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/decanter_of_endless_water
  implementation: dnd2024:item_decanter_of_endless_water
  parameters:
    weight_lb: 2
    activation:
      action: magic
      remove_stopper: true
      command_word: true
      water_type_choice:
      - fresh
      - salt
      end: self_next_turn_start
      options:
        splash:
          gallons: 1
        fountain:
          gallons: 5
        geyser:
          gallons: 30
          shape: line
          length_ft: 30
          width_ft: 1
          aim:
            requires: holding
            action: none
            directions: 1
          target:
            choice:
            - one_creature_in_line
            - one_object_in_line
          creature:
            save:
              ability: strength
              dc: 13
            failure:
              damage:
                roll:
                  dice: 1
                  sides: 4
                  bonus: 0
                type: bludgeoning
              condition: prone
          object:
            worn_or_carried: false
            maximum_weight_lb: 200
            effect: knocked_over
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_decanter_of_endless_water
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
  - spatial_queries
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f0d419e127d5dfb836429e02a64642f584ec8104a63f0247752808a8dc230c2f
  source_path: builtin/srd-5.2.1/magic_item/decanter_of_endless_water.md
  pages:
  - 217
  engine_gaps: &id001
  - dnd2024:item_decanter_of_endless_water
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: fcf0434b2f9f61a45884964224923fdb4a5f5662b3a78ae21b2fd2c41d31e2fe
splat_clauses:
- key: source_rules
  text: '# Decanter of Endless Water Wondrous Item, Uncommon This stoppered flask sloshes when shaken,
    as if it contains water. The decanter weighs 2 pounds. You can take a Magic action to remove the stopper
    and issue one of three command words, whereupon an amount of fresh water or salt water (your choice)
    pours out of the flask. The water stops pouring out at the start of your next turn. Choose from the
    following command words: Splash. The decanter produces 1 gallon of water. Fountain. The decanter produces
    5 gallons of water. Geyser. The decanter produces 30 gallons of water that gushes forth in a Line
    30 feet long and 1 foot wide. If you’re holding the decanter, you can aim the geyser in one direction
    (no action required). One creature of your choice in the Line must succeed on a DC 13 Strength saving
    throw or take 1d4 Bludgeoning damage and have the Prone condition. Instead of a creature, you can
    target one object in the Line that isn’t being worn or carried and that weighs no more than 200 pounds.
    The object is knocked over by the geyser.'
  sha256: efb981f2c7a23d453feba72cc4bbff8a043ea436eb4e29119e0db4ac3e9d91c7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Decanter of Endless Water

<!-- SRD page 217 -->

Wondrous Item, Uncommon\
This stoppered flask sloshes when shaken, as if it\
contains water. The decanter weighs 2 pounds.\
You can take a Magic action to remove the stopper\
and issue one of three command words, whereupon\
an amount of fresh water or salt water (your choice)\
pours out of the flask. The water stops pouring out\
at the start of your next turn. Choose from the fol-\
lowing command words:\
Splash. The decanter produces 1 gallon of water.\
Fountain. The decanter produces 5 gallons of water.\
Geyser. The decanter produces 30 gallons of water\
that gushes forth in a Line 30 feet long and 1 foot\
wide. If you’re holding the decanter, you can aim\
the geyser in one direction (no action required).\
One creature of your choice in the Line must suc-\
ceed on a DC 13 Strength saving throw or take\
1d4 Bludgeoning damage and have the Prone con-\
dition. Instead of a creature, you can target one\
object in the Line that isn’t being worn or carried\
and that weighs no more than 200 pounds. The\
object is knocked over by the geyser.\

Original layout: [[../../srd-5.2.1/pages/217|page 217]].
