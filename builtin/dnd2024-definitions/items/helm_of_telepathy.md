---
splat_schema: 1
splat_id: d0f2dd2f-9c3d-592f-85f4-8423d591d3ce
splat_kind: item
splat_key: dnd2024:items/helm_of_telepathy
splat_name: Helm of Telepathy
splat_origin:
  key: dnd2024:reference/magic_item/helm_of_telepathy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/helm_of_telepathy
  implementation: dnd2024:item_helm_of_telepathy
  parameters:
    equipped: worn
    telepathy_range_ft: 30
    spells:
    - spell:
        key: dnd2024:spells/detect_thoughts
      save_dc: 13
      uses:
        maximum: 1
        recovery_event: time.dawn
      usage_pool: independent_per_spell
    - spell:
        key: dnd2024:spells/suggestion
      save_dc: 13
      uses:
        maximum: 1
        recovery_event: time.dawn
      usage_pool: independent_per_spell
    attunement_required: true
splat_contracts:
- id: dnd2024:item_helm_of_telepathy
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
  - spell.cast_pending
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
  - spatial_targeting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: dc23210a908576594a87ecf129aeb5ba02762499c84e112cef4fcad574e01292
  source_path: builtin/srd-5.2.1/magic_item/helm_of_telepathy.md
  pages:
  - 226
  engine_gaps: &id001
  - dnd2024:item_helm_of_telepathy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: b4dae0d8b43d842192c59cfeee139f84668a197c114372540ca992e5ab059fae
splat_clauses:
- key: source_rules
  text: '# Helm of Telepathy Wondrous Item, Uncommon (Requires Attunement) While wearing this helm, you
    have telepathy with a range of 30 feet, and you can cast Detect Thoughts or Suggestion (save DC 13)
    from the helm. Once either spell is cast from the helm, that spell can’t be cast from it again until
    the next dawn.'
  sha256: 889812f30dbc4c35b831df43243bb27a62f71f936ff8985caf0503b3dd271888
  status: contract_specified
splat_requires_mechanics: *id001
---

# Helm of Telepathy

<!-- SRD page 226 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this helm, you have telepathy with a\
range of 30 feet, and you can cast Detect Thoughts\
or Suggestion (save DC 13) from the helm. Once ei-\
ther spell is cast from the helm, that spell can’t be\
cast from it again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/226|page 226]].
