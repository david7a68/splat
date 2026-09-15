---
splat_schema: 1
splat_id: 6390df81-df8b-5e4f-850a-e6133818db83
splat_kind: item
splat_key: dnd2024:items/talisman_of_ultimate_evil
splat_name: Talisman of Ultimate Evil
splat_origin:
  key: dnd2024:reference/magic_item/talisman_of_ultimate_evil
splat_sources:
- title: SRD 5.2.1
  locator: pp. 249
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/talisman_of_ultimate_evil
  implementation: dnd2024:item_talisman_of_ultimate_evil
  parameters:
    touch_harm:
      target_type_not_in:
      - fiend
      - undead
      triggers:
      - touch
      - turn_end_holding_or_carrying
      damage:
        amount:
          roll:
            count: 8
            sides: 6
        type: necrotic
      requires_attunement: false
    holy_symbol: true
    while_worn_or_held:
      spell_attack_bonus: 2
    charges:
      initial: 6
      maximum: 6
      recovery: null
    ultimate_end:
      equipped_any:
      - worn
      - held
      action: magic
      charge_cost: 1
      target:
        visible: true
        on_ground: true
        range_ft: 120
        creature_count: 1
      save:
        ability: dexterity
        dc: 20
        failure:
          destroy_target: true
          remains: false
        success:
          damage:
            amount:
              roll:
                count: 4
                sides: 6
            type: psychic
      save_disadvantage_types:
      - celestial
      fissure: closes_without_trace
    at_zero_charges:
      destroy_item: true
      residue: slime
    attunement_required: true
splat_contracts:
- id: dnd2024:item_talisman_of_ultimate_evil
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
  - item.touched
  - turn.ended
  - item.activation_requested
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - item_charges
  - spatial_targeting
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 98b7fb880f0c762cdc1ba1a782c863b01cef5f7d4c38f4e01beb8fc651eea18d
  source_path: builtin/srd-5.2.1/magic_item/talisman_of_ultimate_evil.md
  pages:
  - 249
  engine_gaps: &id001
  - dnd2024:item_talisman_of_ultimate_evil
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 1c5764315647e04d921737f3cd95211d114624c63f899ff323ccd85f045950fc
splat_clauses:
- key: source_rules
  text: '# Talisman of Ultimate Evil Wondrous Item, Legendary (Requires Attunement) This item symbolizes
    unrepentant evil. A creature that isn’t a Fiend or an Undead that touches the talisman takes 8d6 Necrotic
    damage and takes the damage again each time it ends its turn holding or carrying the talisman. Holy
    Symbol. You can use the talisman as a Holy Symbol. You gain a +2 bonus to spell attack rolls while
    you wear or hold it. Ultimate End. The talisman has 6 charges. While wearing or holding the talisman,
    you can take a Magic action to expend 1 charge and target one creature you can see on the ground within
    120 feet of yourself. A flaming fissure opens under the target, and the target makes a DC 20 Dexterity
    saving throw. If the target is a Celestial, it has Disadvantage on the save. On a failed save, the
    target falls into the fissure and is destroyed, leaving no remains. On a successful save, the target
    isn’t cast into the fissure but takes 4d6 Psychic damage from the ordeal. In either case, the fissure
    then closes, leaving no trace of its existence. When you expend the last charge, the talisman dissolves
    into foul-smelling slime and is destroyed.'
  sha256: 1c9a883fbffa457e323055ef02775435708f357c803820e41e3e6f9b0fd7d064
  status: contract_specified
splat_requires_mechanics: *id001
---

# Talisman of Ultimate Evil

<!-- SRD page 249 -->

Wondrous Item, Legendary (Requires Attunement)\
This item symbolizes unrepentant evil. A creature\
that isn’t a Fiend or an Undead that touches the\
talisman takes 8d6 Necrotic damage and takes the\
damage again each time it ends its turn holding or\
carrying the talisman.\
Holy Symbol. You can use the talisman as a Holy\
Symbol. You gain a +2 bonus to spell attack rolls\
while you wear or hold it.\
Ultimate End. The talisman has 6 charges. While\
wearing or holding the talisman, you can take a\
Magic action to expend 1 charge and target one\
creature you can see on the ground within 120\
feet of yourself. A flaming fissure opens under the\
target, and the target makes a DC 20 Dexterity\
saving throw. If the target is a Celestial, it has Dis-\
advantage on the save. On a failed save, the target\
falls into the fissure and is destroyed, leaving no re-\
mains. On a successful save, the target isn’t cast into\
the fissure but takes 4d6 Psychic damage from the\
ordeal. In either case, the fissure then closes, leav-\
ing no trace of its existence. When you expend the\
last charge, the talisman dissolves into foul-smell-\
ing slime and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/249|page 249]].
