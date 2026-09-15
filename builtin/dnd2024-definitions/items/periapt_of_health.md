---
splat_schema: 1
splat_id: 6776851c-fb4c-5fac-a0f3-e0e73424054d
splat_kind: item
splat_key: dnd2024:items/periapt_of_health
splat_name: Periapt of Health
splat_origin:
  key: dnd2024:reference/magic_item/periapt_of_health
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/periapt_of_health
  implementation: dnd2024:item_periapt_of_health
  parameters:
    equipment_state: worn
    healing:
      action: magic
      resource:
        uses: 1
        recovery: dawn
      hit_points:
        dice: 2
        sides: 4
        bonus: 2
    saving_throws:
      advantage: true
      purpose:
      - avoid_poisoned
      - end_poisoned
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_periapt_of_health
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
  source_sha256: 5742349cd36dfef826dc7c302f31a59d2172eb8c295a1b6b61cb325a8f37343f
  source_path: builtin/srd-5.2.1/magic_item/periapt_of_health.md
  pages:
  - 234
  engine_gaps: &id001
  - dnd2024:item_periapt_of_health
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 1201451d214b1533e309f8cae7605b2ba977068c988303eb338dc02f6ca4dc6f
splat_clauses:
- key: source_rules
  text: '# Periapt of Health Wondrous Item, Uncommon (Requires Attunement) While wearing this pendant,
    you can take a Magic action to regain 2d4 + 2 Hit Points. Once used, this property can’t be used again
    until the next dawn. In addition, you have Advantage on saving throws to avoid or end the Poisoned
    condition while you wear this pendant.'
  sha256: d2235e7df542ae8e08ed0af2c63161f5bde4ab9412c4b333d65c4969d873d825
  status: contract_specified
splat_requires_mechanics: *id001
---

# Periapt of Health

<!-- SRD page 234 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this pendant, you can take a Magic\
action to regain 2d4 + 2 Hit Points. Once used, this\
property can’t be used again until the next dawn.\
In addition, you have Advantage on saving throws\
to avoid or end the Poisoned condition while you\
wear this pendant.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].
