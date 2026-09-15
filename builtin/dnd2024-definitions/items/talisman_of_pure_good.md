---
splat_schema: 1
splat_id: 43be2844-96d0-5f1b-b02a-7a33082106e2
splat_kind: item
splat_key: dnd2024:items/talisman_of_pure_good
splat_name: Talisman of Pure Good
splat_origin:
  key: dnd2024:reference/magic_item/talisman_of_pure_good
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248, 249
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: required
    attunement_requirement_text: a Cleric or Paladin
splat_rule:
  mechanic: dnd2024:items/talisman_of_pure_good
  implementation: dnd2024:item_talisman_of_pure_good
  parameters:
    touch_hazard:
      attunement_required: false
      target_types:
      - fiend
      - undead
      triggers:
      - touch_item
      - end_turn_holding_or_carrying_item
      damage:
        roll:
          dice: 8
          sides: 6
          bonus: 0
        type: radiant
    holy_symbol: true
    held_or_worn:
      spell_attack_bonus: 2
    charges:
      initial: 7
      maximum: 7
      recovery: null
      on_zero: destroy_item
    pure_rebuke:
      requires: held_or_worn
      action: magic
      cost: 1
      target:
        kind: creature
        count: 1
        seen: true
        on_ground: true
        range_ft: 120
      save:
        ability: dexterity
        dc: 20
        disadvantage_types:
        - fiend
        - undead
      failure:
        fall_into_fissure: true
        destroy_target: true
        remains: false
      success:
        damage:
          roll:
            dice: 4
            sides: 6
            bonus: 0
          type: psychic
        fall_into_fissure: false
      after_either_result:
        fissure: closes_without_trace
    attunement:
      required: true
      eligibility:
        class_any:
        - cleric
        - paladin
splat_contracts:
- id: dnd2024:item_talisman_of_pure_good
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
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: eeb66fd05243b51bc8db72c0dcb855807e7219c291668cc51d30ef3880dcbc5b
  source_path: builtin/srd-5.2.1/magic_item/talisman_of_pure_good.md
  pages:
  - 248
  - 249
  engine_gaps: &id001
  - dnd2024:item_talisman_of_pure_good
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 64160130841cb6898fbf14c1c746fc51d75e1c7b61d86ae60dc1d86363f278d7
splat_clauses:
- key: source_rules
  text: '# Talisman of Pure Good Wondrous Item, Legendary (Requires Attunement by a Cleric or Paladin)
    This talisman is a mighty symbol of goodness. A Fiend or an Undead that touches the talisman takes
    8d6 Radiant damage and takes the damage again each time it ends its turn holding or carrying the talisman.
    Holy Symbol. You can use the talisman as a Holy Symbol. You gain a +2 bonus to spell attack rolls
    while you wear or hold it. Pure Rebuke. The talisman has 7 charges. While wearing or holding the talisman,
    you can take a Magic action to expend 1 charge and target one creature you can see on the ground within
    120 feet of yourself. A flaming fissure opens under the target, and the target makes a DC 20 Dexterity
    saving throw. If the target is a Fiend or an Undead, it has Disadvantage on the save. On a failed
    save, the target falls into the fissure and is destroyed, leaving no remains. On a successful save,
    the target isn’t cast into the fissure but takes 4d6 Psychic damage from the ordeal. In either case,
    the fissure then closes, leaving no trace of its existence. When you expend the last charge, the talisman
    disperses into motes of golden light and is destroyed.'
  sha256: 479963407c8be96d3faf4efe065dc59853dee180a4118ae530d03984200096e5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Talisman of Pure Good

<!-- SRD page 248 -->

Wondrous Item, Legendary (Requires Attunement by a\
Cleric or Paladin)\
This talisman is a mighty symbol of goodness. A\
Fiend or an Undead that touches the talisman takes\
8d6 Radiant damage and takes the damage again\
each time it ends its turn holding or carrying the\
talisman.\

<!-- SRD page 249 -->

Holy Symbol. You can use the talisman as a Holy\
Symbol. You gain a +2 bonus to spell attack rolls\
while you wear or hold it.\
Pure Rebuke. The talisman has 7 charges. While\
wearing or holding the talisman, you can take a\
Magic action to expend 1 charge and target one\
creature you can see on the ground within 120 feet\
of yourself. A flaming fissure opens under the tar-\
get, and the target makes a DC 20 Dexterity saving\
throw. If the target is a Fiend or an Undead, it has\
Disadvantage on the save. On a failed save, the tar-\
get falls into the fissure and is destroyed, leaving no\
remains. On a successful save, the target isn’t cast\
into the fissure but takes 4d6 Psychic damage from\
the ordeal. In either case, the fissure then closes,\
leaving no trace of its existence. When you expend\
the last charge, the talisman disperses into motes of\
golden light and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]], [[../../srd-5.2.1/pages/249|page 249]].
