---
splat_schema: 1
splat_id: 7778c6e8-9f45-54cb-b8c2-b83cd45b4304
splat_kind: item
splat_key: dnd2024:items/manual_of_bodily_health
splat_name: Manual of Bodily Health
splat_origin:
  key: dnd2024:reference/magic_item/manual_of_bodily_health
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/manual_of_bodily_health
  implementation: dnd2024:item_manual_of_bodily_health
  parameters:
    study:
      hours_required: 48
      maximum_elapsed_days: 6
      practice_required: true
      ability: constitution
      increase: 2
      maximum_score: 30
      after_completion: item_loses_magic
      magic_recovery:
        value: 100
        unit: year
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_manual_of_bodily_health
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    study_duration:
      type: duration
      required: true
    elapsed_duration:
      type: duration
      required: true
  outputs:
  - ability_scores
  - item_state
  events:
  - study.completed
  capabilities:
  - downtime
  - ability_score_changes
  - item_resources
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 05afd83410dad15e367296d4e0c2909a80194831d9d14d359343ad9a94134f33
  source_path: builtin/srd-5.2.1/magic_item/manual_of_bodily_health.md
  pages:
  - 229
  engine_gaps: &id001
  - dnd2024:item_manual_of_bodily_health
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 408720d7269123b505ae4245be19606ce5e39ef69d01d0cdf4c3482f06bf22d2
splat_clauses:
- key: source_rules
  text: '# Manual of Bodily Health Wondrous Item, Very Rare This book contains health and nutrition tips,
    and its words are charged with magic. If you spend 48 hours over a period of 6 days or fewer studying
    the book’s contents and practicing its guidelines, your Constitution increases by 2, to a maximum
    of 30. The manual then loses its magic but regains it in a century.'
  sha256: a1d412ffe88a7eda0d645920afae6e901806cb6916b83f4e7d8d7612841af0f9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Manual of Bodily Health

<!-- SRD page 229 -->

Wondrous Item, Very Rare\
This book contains health and nutrition tips, and\
its words are charged with magic. If you spend 48\
hours over a period of 6 days or fewer studying the\
book’s contents and practicing its guidelines, your\
Constitution increases by 2, to a maximum of 30.\
The manual then loses its magic but regains it in a\
century.\

Original layout: [[../../srd-5.2.1/pages/229|page 229]].
