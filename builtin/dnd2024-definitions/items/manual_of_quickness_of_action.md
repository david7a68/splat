---
splat_schema: 1
splat_id: a01a6417-c815-552f-ae68-dc11eb74e350
splat_kind: item
splat_key: dnd2024:items/manual_of_quickness_of_action
splat_name: Manual of Quickness of Action
splat_origin:
  key: dnd2024:reference/magic_item/manual_of_quickness_of_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 230
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/manual_of_quickness_of_action
  implementation: dnd2024:item_manual_of_quickness_of_action
  parameters:
    study:
      hours_required: 48
      maximum_elapsed_days: 6
      practice_required: true
      ability: dexterity
      increase: 2
      maximum_score: 30
      after_completion: item_loses_magic
      magic_recovery:
        value: 100
        unit: year
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_manual_of_quickness_of_action
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
  source_sha256: 2414d86ace8a9fa9e3b930912afe5c3a29731d37fa33f1a52a791c643d12f332
  source_path: builtin/srd-5.2.1/magic_item/manual_of_quickness_of_action.md
  pages:
  - 230
  engine_gaps: &id001
  - dnd2024:item_manual_of_quickness_of_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 97908b362250ecb4d40ccddbfc629f288c91d65ed88ec75387fe33805cad7ba4
splat_clauses:
- key: source_rules
  text: '# Manual of Quickness of Action Wondrous Item, Very Rare This book contains coordination and
    balance exercises, and its words are charged with magic. If you spend 48 hours over a period of 6
    days or fewer studying the book’s contents and practicing its guidelines, your Dexterity increases
    by 2, to a maximum of 30. The manual then loses its magic but regains it in a century.'
  sha256: 7ba56daa4e72d09a48fd538b288716fb91e85831868537941c075d0cf739167d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Manual of Quickness of Action

<!-- SRD page 230 -->

Wondrous Item, Very Rare\
This book contains coordination and balance ex-\
ercises, and its words are charged with magic.\
If you spend 48 hours over a period of 6 days or\
fewer studying the book’s contents and practicing\
its guidelines, your Dexterity increases by 2, to a\
maximum of 30. The manual then loses its magic\
but regains it in a century.\

Original layout: [[../../srd-5.2.1/pages/230|page 230]].
