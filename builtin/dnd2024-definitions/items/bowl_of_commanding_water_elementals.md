---
splat_schema: 1
splat_id: 09e38628-990e-50a7-87be-311aa6df7f73
splat_kind: item
splat_key: dnd2024:items/bowl_of_commanding_water_elementals
splat_name: Bowl of Commanding Water Elementals
splat_origin:
  key: dnd2024:reference/magic_item/bowl_of_commanding_water_elementals
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/bowl_of_commanding_water_elementals
  implementation: dnd2024:item_bowl_of_commanding_water_elementals
  parameters:
    activation:
      action: magic
      distance_ft: 5
      requires: bowl_filled_with_water
      resource:
        uses: 1
        recovery: dawn
      summon:
        creature:
          key: dnd2024:creatures/water_elemental
        placement:
          unoccupied: true
          selection: nearest_item
        understands: summoner_languages
        commands: obey
        initiative: immediately_after_summoner_same_count
        duration:
          value: 1
          unit: hour
        end_on:
        - death
        - summoner_bonus_action_dismissal
    container:
      diameter_ft: 1
      depth_ft: 0.5
      capacity_gallons: 3
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_bowl_of_commanding_water_elementals
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
    locations:
      required: false
      type: location_list
  outputs:
  - summoned_creatures
  - item_state
  events:
  - item.used
  - turn.started
  - creature.died
  capabilities:
  - summoning
  - initiative
  - item_resources
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 419dea6647b6342f0c5e2d447d560826b4435828522bc8cd46dffe2293a0a35a
  source_path: builtin/srd-5.2.1/magic_item/bowl_of_commanding_water_elementals.md
  pages:
  - 214
  engine_gaps: &id001
  - dnd2024:item_bowl_of_commanding_water_elementals
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 9f0f5e671d3fcd4c8306d6bccf002b86e0e990177fc5e90122a2dc9ec3872e5a
splat_clauses:
- key: source_rules
  text: '# Bowl of Commanding Water Elementals Wondrous Item, Rare While this bowl is filled with water
    and you are within 5 feet of it, you can take a Magic action to summon a Water Elemental. The elemental
    appears in an unoccupied space as close to the bowl as possible, understands your languages, obeys
    your commands, and takes its turn immediately after you on your Initiative count. The elemental disappears
    after 1 hour, when it dies, or when you dismiss it as a Bonus Action. The bowl can’t be used this
    way again until the next dawn. The bowl is about 1 foot in diameter and half as deep. It holds about
    3 gallons.'
  sha256: 72eb8b9cf769951869897116c6bdee339521a08e9b13274655dd2fbf439d742b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bowl of Commanding Water Elementals

<!-- SRD page 214 -->

Wondrous Item, Rare\
While this bowl is filled with water and you are\
within 5 feet of it, you can take a Magic action to\
summon a Water Elemental. The elemental ap-\
pears in an unoccupied space as close to the bowl\
as possible, understands your languages, obeys\
your commands, and takes its turn immediately\
after you on your Initiative count. The elemental\
disappears after 1 hour, when it dies, or when you\
dismiss it as a Bonus Action. The bowl can’t be used\
this way again until the next dawn.\
The bowl is about 1 foot in diameter and half as\
deep. It holds about 3 gallons.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].
