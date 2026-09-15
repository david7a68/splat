---
splat_schema: 1
splat_id: 257e062b-0245-5f12-9887-8d187aac2a7b
splat_kind: item
splat_key: dnd2024:items/sending_stones
splat_name: Sending Stones
splat_origin:
  key: dnd2024:reference/magic_item/sending_stones
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/sending_stones
  implementation: dnd2024:item_sending_stones
  parameters:
    pair_size: 2
    equipped: touching
    spells:
    - spell:
        key: dnd2024:spells/sending
      target: other_stone_bearer
      uses:
        maximum: 1
        recovery_event: time.dawn
      usage_pool: shared_pair
    no_other_bearer:
      inform_user: true
      cast_spell: false
      expend_use: false
    one_stone_destroyed:
      other_stone_magic: false
    attunement_required: false
splat_contracts:
- id: dnd2024:item_sending_stones
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
  - item.destroyed
  capabilities:
  - item_effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 0a9c513bdcbb7ac5b3a8ba39f19e7187e0183333a459f5a258334732f2a4a59a
  source_path: builtin/srd-5.2.1/magic_item/sending_stones.md
  pages:
  - 243
  engine_gaps: &id001
  - dnd2024:item_sending_stones
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: bb4cd8eb31eac70d21c3828c41922f47c0547fa49147d2c4ac58a3f6f9431ac2
splat_clauses:
- key: source_rules
  text: '# Sending Stones Wondrous Item, Uncommon Sending Stones come in pairs, with each stone carved
    to match the other so the pairing is easily recognized. While you touch one stone, you can cast Sending
    from it. The target is the bearer of the other stone. If no creature bears the other stone, you know
    that fact as soon as you use the stone, and you don’t cast the spell. Once Sending is cast using either
    stone, the stones can’t be used again until the next dawn. If one of the stones in a pair is destroyed,
    the other one becomes nonmagical.'
  sha256: e02d6b83f648cb49adeb4b77a6118dd5e930ea61ca79a9bdcf77e08455a3a4f0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sending Stones

<!-- SRD page 243 -->

Wondrous Item, Uncommon\
Sending Stones come in pairs, with each stone\
carved to match the other so the pairing is easily\
recognized. While you touch one stone, you can cast\
Sending from it. The target is the bearer of the other\
stone. If no creature bears the other stone, you\
know that fact as soon as you use the stone, and you\
don’t cast the spell.\
Once Sending is cast using either stone, the stones\
can’t be used again until the next dawn. If one of the\
stones in a pair is destroyed, the other one becomes\
nonmagical.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].
