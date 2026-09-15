---
splat_schema: 1
splat_id: 35b045cf-cf79-5cba-a8d6-201145db8236
splat_kind: item
splat_key: dnd2024:items/crystal_ball_of_telepathy
splat_name: Crystal Ball of Telepathy
splat_origin:
  key: dnd2024:reference/magic_item/crystal_ball_of_telepathy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/crystal_ball_of_telepathy
  implementation: dnd2024:item_crystal_ball_of_telepathy
  parameters:
    equipment_state: touching
    spells:
    - spell:
        key: dnd2024:spells/scrying
      save_dc: 17
    - spell:
        key: dnd2024:spells/suggestion
      save_dc: 17
      resource:
        uses: 1
        recovery: dawn
      target:
        seen: true
        within_sensor_ft: 30
      origin: scrying_sensor
      concentration: false
      end_on:
      - scrying_ends
    telepathy:
      targets: seen_creatures
      within_sensor_ft: 30
      requires: scrying_active
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_crystal_ball_of_telepathy
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
  outputs:
  - effects
  - item_state
  events:
  - item.used
  - spell.cast_completed
  capabilities:
  - equipment
  - item_resources
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 4811836e17a2faeb52ee9b493195d14120dae854921f3d2486cf5e427c6087bf
  source_path: builtin/srd-5.2.1/magic_item/crystal_ball_of_telepathy.md
  pages:
  - 216
  engine_gaps: &id001
  - dnd2024:item_crystal_ball_of_telepathy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 10ec78c8b6c5a1a5a04747194b82ae020b48d897477a591d1e99e0d63ebb284a
splat_clauses:
- key: source_rules
  text: '# Crystal Ball of Telepathy Wondrous Item, Legendary (Requires Attunement) While touching this
    crystal orb, you can cast Scrying (save DC 17) with it. In addition, you can communicate telepathically
    with creatures you can see within 30 feet of the spell’s sensor. You can also cast Suggestion (save
    DC 17) through the sensor on one of those creatures. You don’t need to concentrate on this Suggestion
    to maintain it during its duration, but it ends if Scrying ends. You can’t cast Suggestion in this
    way again until the next dawn.'
  sha256: 5029ead7b8a6b765b3f528c02d3277496b7fe769e7cd60b788d61046a4a4cd05
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crystal Ball of Telepathy

<!-- SRD page 216 -->

Wondrous Item, Legendary (Requires Attunement)\
While touching this crystal orb, you can cast Scry-\
ing (save DC 17) with it. In addition, you can com-\
municate telepathically with creatures you can see\
within 30 feet of the spell’s sensor. You can also cast\
Suggestion (save DC 17) through the sensor on one\
of those creatures. You don’t need to concentrate on\
this Suggestion to maintain it during its duration,\
but it ends if Scrying ends. You can’t cast Suggestion\
in this way again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].
