---
splat_schema: 1
splat_id: cf84bbbb-c777-5992-b207-faec02b2afa9
splat_kind: item
splat_key: dnd2024:items/glamoured_studded_leather
splat_name: Glamoured Studded Leather
splat_origin:
  key: dnd2024:reference/magic_item/glamoured_studded_leather
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223, 224
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/glamoured_studded_leather
  implementation: dnd2024:item_glamoured_studded_leather
  parameters:
    equipped: worn
    ac_bonus: 1
    appearance_change:
      action: bonus_action
      illusion: true
      appearance:
        choice: wearer
        kinds:
        - normal_clothing
        - armor
        attributes:
        - color
        - style
        - accessories
      retains:
      - bulk
      - weight
      ends_on:
      - next_appearance_change
      - armor_doffed
    attunement_required: false
splat_contracts:
- id: dnd2024:item_glamoured_studded_leather
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
  - item.activation_requested
  - item.equipment_changed
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 019d099dce20934e6d45b317bb63bf55d05cf0c7e61490fd017290189c2e7dcd
  source_path: builtin/srd-5.2.1/magic_item/glamoured_studded_leather.md
  pages:
  - 223
  - 224
  engine_gaps: &id001
  - dnd2024:item_glamoured_studded_leather
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 8ea9ba521ff9af98a56c24570b382487f9630e917a9ccaf098704dfb88a89033
splat_clauses:
- key: source_rules
  text: '# Glamoured Studded Leather Armor (Studded Leather Armor), Rare While wearing this armor, you
    gain a +1 bonus to Armor Class. You can also take a Bonus Action to cause the armor to assume the
    appearance of a normal set of clothing or some other kind of armor. You decide what it looks like—including
    color, style, and accessories—but the armor retains its normal bulk and weight. The illusory appearance
    lasts until you use this property again or doff the armor.'
  sha256: 56769dd61ae4b32a3ab6ab93b6d31fcb1689beb2f1905a9c3acccdfd2da64221
  status: contract_specified
splat_requires_mechanics: *id001
---

# Glamoured Studded Leather

<!-- SRD page 223 -->

Armor (Studded Leather Armor), Rare\
While wearing this armor, you gain a +1 bonus to\
Armor Class. You can also take a Bonus Action to\

<!-- SRD page 224 -->

cause the armor to assume the appearance of a nor-\
mal set of clothing or some other kind of armor. You\
decide what it looks like—including color, style, and\
accessories—but the armor retains its normal bulk\
and weight. The illusory appearance lasts until you\
use this property again or doff the armor.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]], [[../../srd-5.2.1/pages/224|page 224]].
