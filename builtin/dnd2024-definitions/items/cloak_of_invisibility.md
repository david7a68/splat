---
splat_schema: 1
splat_id: f3314c8d-5295-5e5d-8945-3625bab644fc
splat_kind: item
splat_key: dnd2024:items/cloak_of_invisibility
splat_name: Cloak of Invisibility
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215, 216
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/cloak_of_invisibility
  implementation: dnd2024:item_cloak_of_invisibility
  parameters:
    equipment_state: worn
    charges:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 3
          bonus: 0
    activation:
      action: magic
      gesture: pull_hood_up
      cost: 1
      condition: invisible
      duration:
        value: 1
        unit: hour
      end_on:
      - hood_lowered
      - cloak_removed
      lower_hood_action: none
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_cloak_of_invisibility
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
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: aa6634a51609712fe725df8a301fa46b6df517dc5d61c401cec70e2955e11888
  source_path: builtin/srd-5.2.1/magic_item/cloak_of_invisibility.md
  pages:
  - 215
  - 216
  engine_gaps: &id001
  - dnd2024:item_cloak_of_invisibility
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: c739cc4729f1337aceeed7766ae226f3bfb3107930ff2aa4c91c6b27f18f71d7
splat_clauses:
- key: source_rules
  text: '# Cloak of Invisibility Wondrous Item, Legendary (Requires Attunement) This cloak has 3 charges
    and regains 1d3 expended charges daily at dawn. While wearing the cloak, you can take a Magic action
    to pull its hood over your head and expend 1 charge to give yourself the Invisible condition for 1
    hour. The effect ends early if you pull the hood down (no action required) or cease wearing the cloak.'
  sha256: d8b1e43f43106e48b76c2cca8eb695613c8058ccecf4caf3c40062bb67701b7f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cloak of Invisibility

<!-- SRD page 215 -->

Wondrous Item, Legendary (Requires Attunement)\
This cloak has 3 charges and regains 1d3 expended\
charges daily at dawn. While wearing the cloak, you\
can take a Magic action to pull its hood over your\

<!-- SRD page 216 -->

head and expend 1 charge to give yourself the Invis-\
ible condition for 1 hour. The effect ends early if you\
pull the hood down (no action required) or cease\
wearing the cloak.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]], [[../../srd-5.2.1/pages/216|page 216]].
