---
splat_schema: 1
splat_id: d3eece81-23d6-5ce1-b296-e84b65be3d85
splat_kind: item
splat_key: dnd2024:items/stone_of_controlling_earth_elementals
splat_name: Stone of Controlling Earth Elementals
splat_origin:
  key: dnd2024:reference/magic_item/stone_of_controlling_earth_elementals
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/stone_of_controlling_earth_elementals
  implementation: dnd2024:item_stone_of_controlling_earth_elementals
  parameters:
    weight_lb: 5
    activation:
      action: magic
      requires: stone_touching_ground
      resource:
        uses: 1
        recovery: dawn
      summon:
        creature:
          key: dnd2024:creatures/earth_elemental
        placement:
          unoccupied: true
          chooser: user
          range_ft: 30
        commands: obey
        initiative: immediately_after_user_same_count
        duration:
          value: 1
          unit: hour
        end_on:
        - death
        - user_bonus_action_dismissal
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_stone_of_controlling_earth_elementals
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
  source_sha256: 6b0478f26104cdc12e482ac3f02837d45bcd8c8a66265870670408ac2df23027
  source_path: builtin/srd-5.2.1/magic_item/stone_of_controlling_earth_elementals.md
  pages:
  - 248
  engine_gaps: &id001
  - dnd2024:item_stone_of_controlling_earth_elementals
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 15a06d4e928588131f338f296129d74b9d8ef73892b3993882249a14754385a2
splat_clauses:
- key: source_rules
  text: '# Stone of Controlling Earth Elementals Wondrous Item, Rare While touching this 5-pound stone
    to the ground, you can take a Magic action to summon an Earth Elemental. The elemental appears in
    an unoccupied space you choose within 30 feet of yourself, obeys your commands, and takes its turn
    immediately after you on your Initiative count. The elemental disappears after 1 hour, when it dies,
    or when you dismiss it as a Bonus Action. The stone can’t be used this way again until the next dawn.'
  sha256: 0a3b0cf337b073bcfcc928503017410231b3db09860726cea7e4fe7ce172bd6b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stone of Controlling Earth Elementals

<!-- SRD page 248 -->

Wondrous Item, Rare\
While touching this 5-pound stone to the ground,\
you can take a Magic action to summon an Earth\
Elemental. The elemental appears in an unoccupied\
space you choose within 30 feet of yourself, obeys\
your commands, and takes its turn immediately\
after you on your Initiative count. The elemental\
disappears after 1 hour, when it dies, or when you\
dismiss it as a Bonus Action. The stone can’t be used\
this way again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]].
