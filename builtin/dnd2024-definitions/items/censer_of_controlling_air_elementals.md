---
splat_schema: 1
splat_id: 628203ef-4479-58ab-b84a-0d5d2be14f57
splat_kind: item
splat_key: dnd2024:items/censer_of_controlling_air_elementals
splat_name: Censer of Controlling Air Elementals
splat_origin:
  key: dnd2024:reference/magic_item/censer_of_controlling_air_elementals
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/censer_of_controlling_air_elementals
  implementation: dnd2024:item_censer_of_controlling_air_elementals
  parameters:
    activation:
      action: magic
      requires: gently_swinging_censer
      resource:
        uses: 1
        recovery: dawn
      summon:
        creature:
          key: dnd2024:creatures/air_elemental
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
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_censer_of_controlling_air_elementals
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
  source_sha256: 9f7d56f3678b2faefbedefe9713d280a445a624a207a8ebbed7f1491f554230d
  source_path: builtin/srd-5.2.1/magic_item/censer_of_controlling_air_elementals.md
  pages:
  - 215
  engine_gaps: &id001
  - dnd2024:item_censer_of_controlling_air_elementals
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: e73b920b20e5dd9bd9a73c9db5d434aff345e089bb77230ada9d75b99f35862e
splat_clauses:
- key: source_rules
  text: '# Censer of Controlling Air Elementals Wondrous Item, Rare While gently swinging this censer,
    you can take a Magic action to summon an Air Elemental. The elemental appears in an unoccupied space
    as close to the censer as possible, understands your languages, obeys your commands, and takes its
    turn immediately after you on your Initiative count. The elemental disappears after 1 hour, when it
    dies, or when you dismiss it as a Bonus Action. The censer can’t be used this way again until the
    next dawn.'
  sha256: 6b4b496cfb76d33f0ef237233f663bbffcacb676920ba46523250e38a51feb25
  status: contract_specified
splat_requires_mechanics: *id001
---

# Censer of Controlling Air Elementals

<!-- SRD page 215 -->

Wondrous Item, Rare\
While gently swinging this censer, you can take a\
Magic action to summon an Air Elemental. The\
elemental appears in an unoccupied space as close\
to the censer as possible, understands your lan-\
guages, obeys your commands, and takes its turn\
immediately after you on your Initiative count. The\
elemental disappears after 1 hour, when it dies, or\
when you dismiss it as a Bonus Action. The censer\
can’t be used this way again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].
