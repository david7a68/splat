---
splat_schema: 1
splat_id: bf03bcc5-313b-5d37-93b7-5dfa68c7e915
splat_kind: item
splat_key: dnd2024:items/periapt_of_wound_closure
splat_name: Periapt of Wound Closure
splat_origin:
  key: dnd2024:reference/magic_item/periapt_of_wound_closure
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
  mechanic: dnd2024:items/periapt_of_wound_closure
  implementation: dnd2024:item_periapt_of_wound_closure
  parameters:
    equipped: worn
    death_save:
      optional: true
      roll_max: 9
      replace_roll: 10
      result: success
    hit_point_die_healing:
      multiplier: 2
      scope: hit_points_restored_by_die
    attunement_required: true
splat_contracts:
- id: dnd2024:item_periapt_of_wound_closure
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
  - death_save.rolled
  - hit_point_die.healing_resolved
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - healing
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 6bd1ec9987304e059188b08476e92fc2cd4e86c50dc3493a9ab7d4bd67996c4d
  source_path: builtin/srd-5.2.1/magic_item/periapt_of_wound_closure.md
  pages:
  - 234
  engine_gaps: &id001
  - dnd2024:item_periapt_of_wound_closure
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 51b16c90e643d550e537af5a62cfd811869b0e7e5f9761b7ff185316d3032282
splat_clauses:
- key: source_rules
  text: '# Periapt of Wound Closure Wondrous Item, Uncommon (Requires Attunement) While wearing this pendant,
    you gain the following benefits. Life Preservation. Whenever you make a Death Saving Throw, you can
    change a roll of 9 or lower to a 10, turning a failed save into a successful one. Natural Healing
    Boost. Whenever you roll a Hit Point Die to regain Hit Points, double the number of Hit Points it
    restores.'
  sha256: 9f9723b185cecb21f96303aca9595832c92ff8d51b6cd526f6c55570d26b7663
  status: contract_specified
splat_requires_mechanics: *id001
---

# Periapt of Wound Closure

<!-- SRD page 234 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this pendant, you gain the following\
benefits.\
Life Preservation. Whenever you make a Death\
Saving Throw, you can change a roll of 9 or lower to\
a 10, turning a failed save into a successful one.\
Natural Healing Boost. Whenever you roll a Hit\
Point Die to regain Hit Points, double the number of\
Hit Points it restores.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].
